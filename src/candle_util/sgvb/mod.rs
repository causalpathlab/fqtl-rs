//! Stochastic Gradient Variational Bayes (SGVB) — folded from candle-util.
pub mod cavi_susie;
mod composite_model;
mod gaussian_prior;
pub mod likelihood;
mod regression_linear;
mod sgvb_util;
mod susie_util;
mod traits;
mod variational_bisusie;
mod variational_gaussian;
mod variational_io;
mod variational_spike_slab;
mod variational_susie;

pub use composite_model::{
    composite_local_reparam_loss, samples_local_reparam_loss, CompositeModel,
};
pub use gaussian_prior::{FixedGaussianPrior, GaussianPrior, MixtureGaussianPrior, PriorKind};
pub use likelihood::{
    lgamma_approx, FixedGaussianLikelihood, GaussianLikelihood, NegativeBinomialLikelihood,
    OffsetPoissonLikelihood, PoissonLikelihood, RssLikelihood, RssSvd, WeightedGaussianLikelihood,
};
pub use regression_linear::{GaussianRegressionSGVB, RegressionSGVB, SusieRegressionSGVB};
pub use sgvb_util::{
    antithetic_epsilon, generic_local_reparam_loss, local_reparam_loss, SGVBConfig,
};
pub use susie_util::pip_from_alpha;
pub use traits::{
    AnalyticalKL, BlackBoxLikelihood, ComponentVariational, IndependentGateVariational,
    LocalReparamModel, LocalReparamSample, Prior, VariationalDistribution,
};
pub use variational_bisusie::BiSusieVar;
pub use variational_gaussian::GaussianVar;
pub use variational_io::{SparseVariationalOutput, VariationalOutput};
pub use variational_spike_slab::SpikeSlabVar;
pub use variational_susie::SusieVar;
