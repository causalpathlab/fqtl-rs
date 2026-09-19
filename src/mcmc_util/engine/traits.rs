use nalgebra::{DMatrix, DVector};

/// Minimal trait for types usable as ESS parameters.
pub trait EssParam: Clone {
    /// Elliptical combination: `a * self + b * other`
    fn linear_combine(&self, a: f32, other: &Self, b: f32) -> Self;
}

/// Optional trait enabling summary statistics (mean, variance, quantile).
pub trait EssParamSummary: EssParam {
    fn as_slice(&self) -> &[f32];
    fn dim(&self) -> usize;
}

impl EssParam for DVector<f32> {
    fn linear_combine(&self, a: f32, other: &Self, b: f32) -> Self {
        self * a + other * b
    }
}

impl EssParamSummary for DVector<f32> {
    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn dim(&self) -> usize {
        self.nrows()
    }
}

impl EssParam for DMatrix<f32> {
    fn linear_combine(&self, a: f32, other: &Self, b: f32) -> Self {
        self * a + other * b
    }
}

impl EssParamSummary for DMatrix<f32> {
    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn dim(&self) -> usize {
        self.nrows() * self.ncols()
    }
}

impl<P: EssParam> EssParam for Vec<P> {
    fn linear_combine(&self, a: f32, other: &Self, b: f32) -> Self {
        self.iter()
            .zip(other.iter())
            .map(|(s, o)| s.linear_combine(a, o, b))
            .collect()
    }
}
