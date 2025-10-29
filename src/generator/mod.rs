mod uniform;
pub use uniform::Uniform;

mod zipfian;
pub use zipfian::Zipfian;

mod sequential;
pub use sequential::Sequential;

pub trait Generator: Send + Sync {
    fn next(&self) -> u64;
}
