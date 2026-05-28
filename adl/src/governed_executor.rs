#![allow(clippy::module_name_repetitions)]

#[path = "governed_executor_parts/logic.rs"]
mod logic;

#[path = "governed_executor_parts/tests.rs"]
#[cfg(test)]
mod tests;

pub use logic::*;
