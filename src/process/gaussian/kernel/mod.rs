//! Gaussian Processes

use nalgebra::base::constraint::{SameNumberOfColumns, ShapeConstraint};
use nalgebra::base::storage::Storage;
use nalgebra::{DMatrix, DVector, Dim, Matrix};
use std::f64;

mod covgrad;
pub use covgrad::*;

mod misc;
pub use self::misc::*;

mod constant_kernel;
pub use self::constant_kernel::*;

mod ops;
pub use self::ops::*;

mod rbf;
pub use self::rbf::*;
mod white_kernel;
pub use self::white_kernel::*;
mod rational_quadratic;
pub use self::rational_quadratic::*;
mod exp_sin_squared;
pub use self::exp_sin_squared::*;

/// Kernel Function
pub trait Kernel: std::fmt::Debug + Clone + PartialEq {
    // Returns the covariance matrix for two equal sized vectors
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
        ShapeConstraint: SameNumberOfColumns<C1, C2>;
    /// Reports if the given kernel function is stationary.
    fn is_stationary(&self) -> bool;
    /// Returns the diagnal of the kernel(x, x)
    fn diag<R, C, S>(&self, x: &Matrix<f64, R, C, S>) -> DVector<f64>
    where
        R: Dim,
        C: Dim,
        S: Storage<f64, R, C>;

    /// Return the corresponding parameter vector
    /// The parameters here are in a log-scale
    fn parameters(&self) -> Vec<f64>;

    /// Returns the bounds on the parameters
    fn parameter_bounds(&self) -> (Vec<f64>, Vec<f64>);

    /// Create a new kernel of the given type from the provided parameters.
    /// The parameters here are in a log-scale
    fn from_parameters(param: &[f64]) -> Self;

    /// Takes a sequence of parameters and consumes only the ones it needs
    /// to create itself.
    /// The parameters here are in a log-scale
    fn consume_parameters(params: &[f64]) -> (Self, &[f64]);

    /// Covariance and Gradient with the log-scaled hyper-parameters
    fn covariance_with_gradient<R, C, S>(
        &self,
        x: &Matrix<f64, R, C, S>,
    ) -> (DMatrix<f64>, CovGrad)
    where
        R: Dim,
        C: Dim,
        S: Storage<f64, R, C>;

    fn add<B: Kernel>(self, other: B) -> AddKernel<Self, B> {
        AddKernel::new(self, other)
    }

    fn mul<B: Kernel>(self, other: B) -> ProductKernel<Self, B> {
        ProductKernel::new(self, other)
    }
}

macro_rules! impl_mul_add {
    ($type: ty) => {
        impl<B> std::ops::Mul<B> for $type
        where
            B: Kernel,
        {
            type Output = ProductKernel<$type, B>;

            fn mul(self, rhs: B) -> Self::Output {
                ProductKernel::new(self, rhs)
            }
        }

        impl<B> std::ops::Add<B> for $type
        where
            B: Kernel,
        {
            type Output = AddKernel<$type, B>;

            fn add(self, rhs: B) -> Self::Output {
                AddKernel::new(self, rhs)
            }
        }
    };
}

impl_mul_add!(ConstantKernel);
impl_mul_add!(RBFKernel);
impl_mul_add!(ExpSineSquaredKernel);
impl_mul_add!(RationalQuadratic);
impl_mul_add!(WhiteKernel);