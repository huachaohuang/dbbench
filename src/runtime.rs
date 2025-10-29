use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::Result;

use crate::{
    dataset::Dataset,
    db::Db,
    workload::{Operation, Workload},
};

pub struct Runtime {
    db: Box<dyn Db>,
    dataset: Dataset,
    workload: Workload,
}

impl Runtime {
    pub fn new(db: Box<dyn Db>, dataset: Dataset, workload: Workload) -> Self {
        Self {
            db,
            dataset,
            workload,
        }
    }

    pub fn run(self, num_threads: usize, num_operations: usize) {
        let context = Arc::new(Context::new(
            self.db,
            self.dataset,
            self.workload,
            num_operations,
        ));
        let mut handles = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let context = context.clone();
            handles.push(std::thread::spawn(move || context.run()));
        }
        for handle in handles {
            handle.join().unwrap().unwrap();
        }
    }
}

struct Context {
    db: Box<dyn Db>,
    dataset: Dataset,
    workload: Workload,
    statistics: Statistics,
    max_operations: usize,
    num_operations: AtomicUsize,
}

impl Context {
    fn new(db: Box<dyn Db>, dataset: Dataset, workload: Workload, max_operations: usize) -> Self {
        Self {
            db,
            dataset,
            workload,
            statistics: Statistics::new(),
            max_operations,
            num_operations: AtomicUsize::new(0),
        }
    }

    fn run(&self) -> Result<()> {
        let mut kbuf = Vec::new();
        let mut vbuf = Vec::new();
        while let Some(op) = self.next_operation() {
            match op {
                Operation::Read => {
                    self.dataset.next(&mut kbuf);
                    self.statistics.record(op, || self.db.read(&kbuf));
                }
                Operation::Scan => {
                    self.dataset.next(&mut kbuf);
                    self.statistics.record(op, || self.db.scan(&kbuf, 10));
                }
                Operation::Write => {
                    self.dataset.next_record(&mut kbuf, &mut vbuf);
                    self.statistics.record(op, || self.db.write(&kbuf, &vbuf));
                }
            }
        }
        Ok(())
    }

    fn next_operation(&self) -> Option<Operation> {
        let current = self.num_operations.fetch_add(1, Ordering::Relaxed);
        if current >= self.max_operations {
            None
        } else {
            Some(self.workload.next())
        }
    }
}

struct LastReport {
    time: Instant,
    histograms: [Histogram; Operation::COUNT],
}

impl LastReport {
    fn new() -> Self {
        Self {
            time: Instant::now(),
            histograms: Default::default(),
        }
    }
}

struct Statistics {
    count: AtomicUsize,
    failure: AtomicUsize,
    histograms: [AtomicHistogram; Operation::COUNT],
    last_count: AtomicUsize,
    last_report: Mutex<LastReport>,
}

impl Statistics {
    fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
            failure: AtomicUsize::new(0),
            histograms: Default::default(),
            last_count: AtomicUsize::new(0),
            last_report: Mutex::new(LastReport::new()),
        }
    }
}

impl Statistics {
    fn record<F>(&self, op: Operation, f: F)
    where
        F: FnOnce() -> Result<()>,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed().as_micros() as u64;
        self.count.fetch_add(1, Ordering::Relaxed);
        match result {
            Ok(_) => {
                self.histograms[op as usize].add(duration);
            }
            Err(_) => {
                self.failure.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn report(&self) {
        let count = self.count.load(Ordering::Relaxed);
        let failure = self.failure.load(Ordering::Relaxed);
        let last_count = self.last_count.load(Ordering::Relaxed);
        if last_count - count < 1000 {
            return;
        }
        let Ok(mut last_report) = self.last_report.try_lock() else {
            return;
        };

        let now = Instant::now();
        let duration = now.duration_since(last_report.time);
        if duration < Duration::from_secs(1) {
            return;
        }
        last_report.time = now;
        self.last_count.store(count, Ordering::Release);

        println!("--- Total: {} Failures: {} ---", count, failure);
        for (i, (hist, last_hist)) in self
            .histograms
            .iter()
            .zip(last_report.histograms.iter_mut())
            .enumerate()
        {
            let current = hist.load();
            let interval = current.sub(last_hist);
            *last_hist = current;
            println!("{:5?} - {}", Operation::from(i), interval.report(duration));
        }
    }
}

const GROUPING_POWER: u8 = 8;
const MAX_VALUE_POWER: u8 = 64;

struct Histogram {
    count: usize,
    histogram: histogram::Histogram,
}

impl Histogram {
    fn new() -> Self {
        Self {
            count: 0,
            histogram: histogram::Histogram::new(GROUPING_POWER, MAX_VALUE_POWER).unwrap(),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        let count = self.count.checked_sub(other.count).unwrap();
        let histogram = self.histogram.checked_sub(&other.histogram).unwrap();
        Self { count, histogram }
    }

    fn report(&self, duration: Duration) -> String {
        let ops = self.count as f64 / duration.as_secs_f64();
        let p50 = self.percentile(50.0);
        let p95 = self.percentile(95.0);
        let p99 = self.percentile(99.0);
        let max = self.percentile(100.0);
        format!(
            "OPS: {:>7}, P50: {:>7}us, P95: {:>7}us, P99: {:>7}us, MAX: {:>7}us",
            ops, p50, p95, p99, max
        )
    }

    fn percentile(&self, percentile: f64) -> u64 {
        let bucket = self.histogram.percentile(percentile).unwrap().unwrap();
        (bucket.end() - bucket.start()) / 2
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

struct AtomicHistogram {
    count: AtomicUsize,
    histogram: histogram::AtomicHistogram,
}

impl AtomicHistogram {
    fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
            histogram: histogram::AtomicHistogram::new(GROUPING_POWER, MAX_VALUE_POWER).unwrap(),
        }
    }

    fn add(&self, us: u64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.histogram.increment(us).unwrap();
    }

    fn load(&self) -> Histogram {
        Histogram {
            count: self.count.load(Ordering::Relaxed),
            histogram: self.histogram.load(),
        }
    }
}

impl Default for AtomicHistogram {
    fn default() -> Self {
        Self::new()
    }
}
