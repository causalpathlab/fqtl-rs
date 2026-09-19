//! MCMC engine: sampling primitives and generic runner.

mod elliptical_slice;
mod model;
mod runner;
pub mod traits;

pub use elliptical_slice::elliptical_slice_step;
pub use model::McmcModel;
pub use runner::{run_mcmc, McmcConfig};
pub use traits::{EssParam, EssParamSummary};
