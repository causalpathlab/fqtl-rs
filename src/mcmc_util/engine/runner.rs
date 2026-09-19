use rand::rngs::SmallRng;
use rand::SeedableRng;

use super::model::McmcModel;

/// Configuration for the MCMC runner.
pub struct McmcConfig {
    /// Number of posterior samples to collect.
    pub n_samples: usize,
    /// Warmup (burn-in) iterations.
    pub warmup: usize,
    /// Thinning interval.
    pub thin: usize,
    /// RNG seed.
    pub seed: u64,
}

/// Run a single MCMC chain.
pub fn run_mcmc<M: McmcModel>(model: &M, config: &McmcConfig) -> M::Result {
    let total = config.warmup + config.n_samples * config.thin;
    let mut rng = SmallRng::seed_from_u64(config.seed);
    let mut state = model.init(&mut rng);
    let mut samples = Vec::with_capacity(config.n_samples);

    for iter in 0..total {
        model.sweep(&mut state, &mut rng);
        if iter >= config.warmup && (iter - config.warmup).is_multiple_of(config.thin) {
            samples.push(model.collect(&state));
        }
    }

    model.summarize(samples)
}
