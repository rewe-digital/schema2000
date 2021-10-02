#![allow(clippy::module_name_repetitions)]

pub use generate::generate_hypothesis;
pub use merge::merge_hypothesis;
pub use model::SchemaHypothesis;

mod generate;
mod merge;
pub mod model;
mod utils;
