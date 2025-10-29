mod uniform;
pub use uniform::UniformGenerator;

mod zipfian;
pub use zipfian::ZipfianGenerator;

mod sequential;
pub use sequential::SequentialGenerator;

pub trait Generator {
    fn next(&self) -> u64;
}
