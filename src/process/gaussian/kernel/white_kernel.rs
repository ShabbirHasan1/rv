use super::{CovGrad, Kernel};
use nalgebra::base::constraint::{SameNumberOfColumns, ShapeConstraint};
use nalgebra::base::storage::Storage;
use nalgebra::{DMatrix, DVector, Dim, Matrix};
use std::f64;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// White Noise Kernel
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct WhiteKernel {
    /// Level of the noise
    noise_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

impl WhiteKernel {
    pub fn new(noise_level: f64) -> Self {
        Self {
            noise_level,
            lower_bound: 1E-5,
            upper_bound: 1E5,
        }
    }

    pub fn with_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
            ..self
        }
    }
}

impl Kernel for WhiteKernel {
    fn covariance<R1, R2, C1, C2, S1, S2>(
        &self,
        x1: &Matrix<f64, R1, C1, S1>,
        x2: &Matrix<f64, R2, C2, S2>,
    ) -> DMatrix<f64>
    where
        R1: Dim,
        R2: Dim,
        C1: Dim,
        C2: Dim,
        S1: Storage<f64, R1, C1>,
        S2: Storage<f64, R2, C2>,
        ShapeConstraint: SameNumberOfColumns<C1, C2>,
    {
        DMatrix::zeros(x1.nrows(), x2.nrows())
    }

    fn is_stationary(&self) -> bool {
        true
    }

    fn diag<R, C, S>(&self, x: &Matrix<f64, R, C, S>) -> DVector<f64>
    where
        R: Dim,
        C: Dim,
        S: Storage<f64, R, C>,
    {
        let n = x.nrows();
        DVector::from_element(n, self.noise_level)
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.noise_level.ln()]
    }

    fn parameter_bounds(&self) -> (Vec<f64>, Vec<f64>) {
        (vec![self.lower_bound], vec![self.upper_bound])
    }

    fn from_parameters(param_vec: &[f64]) -> Self {
        assert_eq!(param_vec.len(), 1, "Only one parameter expected");
        Self::new(param_vec[0].exp())
    }

    fn consume_parameters(params: &[f64]) -> (Self, &[f64]) {
        assert!(params.len() > 0, "WhiteKernel requires one parameters");
        let (cur, next) = params.split_at(1);
        let ck = Self::from_parameters(cur);
        (ck, next)
    }

    fn covariance_with_gradient<R, C, S>(
        &self,
        x: &Matrix<f64, R, C, S>,
    ) -> (DMatrix<f64>, CovGrad)
    where
        R: Dim,
        C: Dim,
        S: Storage<f64, R, C>,
    {
        let n = x.nrows();
        let cov = DMatrix::from_diagonal_element(n, n, self.noise_level);
        let grad = CovGrad::new(&[DMatrix::from_diagonal_element(
            x.nrows(),
            x.nrows(),
            self.noise_level,
        )]);
        (cov, grad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::relative_eq;

    #[test]
    fn white_kernel() {
        const PI: f64 = std::f64::consts::PI;
        let kernel = WhiteKernel::new(PI);

        assert::close(kernel.parameters()[0], PI.ln(), 1E-10);

        assert!(relative_eq(kernel.parameters(), vec![PI.ln()], 1E-8, 1E-8,));

        let x = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let y = DMatrix::from_row_slice(2, 2, &[5.0, 7.0, 6.0, 8.0]);

        let cov = kernel.covariance(&x, &y);
        let expected_cov = DMatrix::from_row_slice(2, 2, &[0.0, 0.0, 0.0, 0.0]);
        assert!(cov.relative_eq(&expected_cov, 1E-8, 1E-8));

        let (cov, grad) = kernel.covariance_with_gradient(&x);

        println!("cov = {}\ngrad = {}", cov, grad);

        let expected_cov = DMatrix::from_row_slice(2, 2, &[PI, 0.0, 0.0, PI]);

        let expected_grad = CovGrad::from_row_slices(2, 1, &[PI, 0.0, 0.0, PI]);
        assert!(cov.relative_eq(&expected_cov, 1E-8, 1E-8));
        assert!(grad.relative_eq(&expected_grad, 1E-8, 1E-8));
    }
}