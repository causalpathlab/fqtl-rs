//! Likelihood functions for SGVB inference.
mod gaussian;
mod negbinom;
mod poisson;
mod rss;
mod weighted_gaussian;

pub use gaussian::{FixedGaussianLikelihood, GaussianLikelihood};
pub use negbinom::{lgamma_approx, NegativeBinomialLikelihood};
pub use poisson::{OffsetPoissonLikelihood, PoissonLikelihood};
pub use rss::{RssLikelihood, RssSvd};
pub use weighted_gaussian::WeightedGaussianLikelihood;
