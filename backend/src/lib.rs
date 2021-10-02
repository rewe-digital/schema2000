#![allow(clippy::module_name_repetitions)]

pub use generate::generate_hypothesis;
pub use merge::merge_hypothesis;
pub use model::*;

mod generate;
mod merge;
mod model;
mod utils;
