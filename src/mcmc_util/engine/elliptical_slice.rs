use rand::{Rng, RngExt};
use std::f32::consts::PI;

use super::traits::EssParam;

/// Hard cap on bracket-shrinkage iterations inside [`elliptical_slice_step`].
pub(crate) const MAX_BRACKET_ITERS: usize = 64;

/// Bracket-width floor; below this the proposal is indistinguishable from current.
pub(crate) const BRACKET_MIN_WIDTH: f32 = 1e-6;

/// One ESS transition. Returns `(new_params, new_lnpdf)`.
pub fn elliptical_slice_step<P: EssParam>(
    current: &P,
    prior_sample: &P,
    lnpdf: &impl Fn(&P) -> f32,
    cur_lnpdf: f32,
    rng: &mut impl Rng,
) -> (P, f32) {
    let u: f32 = rng.random();
    let hh = u.ln() + cur_lnpdf;

    let phi: f32 = rng.random_range(0.0..2.0 * PI);
    let mut phi_min = phi - 2.0 * PI;
    let mut phi_max = phi;

    let mut angle = phi;
    for _ in 0..MAX_BRACKET_ITERS {
        let proposal = current.linear_combine(angle.cos(), prior_sample, angle.sin());
        let new_lnpdf = lnpdf(&proposal);

        if new_lnpdf > hh {
            return (proposal, new_lnpdf);
        }

        if angle < 0.0 {
            phi_min = angle;
        } else {
            phi_max = angle;
        }
        if phi_max - phi_min < BRACKET_MIN_WIDTH {
            break;
        }
        angle = rng.random_range(phi_min..phi_max);
    }

    (current.clone(), cur_lnpdf)
}
