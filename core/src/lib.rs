#![allow(clippy::module_name_repetitions)]

pub use generate::generate_hypothesis;
pub use merge::merge_hypothesis;
pub use model::SchemaHypothesis;
pub use renderer::{render_json_schema, render_schema};

mod generate;
mod merge;
pub mod model;
mod renderer;
mod utils;
