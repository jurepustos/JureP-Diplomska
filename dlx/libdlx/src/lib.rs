extern crate core;

mod dlx_table;
// mod dlx_table2;
mod dlx;
mod test_cases;
mod dlx2;

pub use dlx2::dlx as dlx2;
// pub use dlx2::dlx2 as dlx22;
pub use dlx::{dlx, dlx_run, DLXIter};
pub use test_cases::{generate_exact_cover};
