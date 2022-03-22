extern crate core;

mod dlx_table;
mod dlx;
mod test_cases;

pub use dlx::{dlx, dlx_run, DLXIter};
pub use test_cases::{generate_exact_cover};
