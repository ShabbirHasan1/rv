//! Provides random variables for probabilistic modeling.
//!
//! The [`dist`] module provides a number of probability distributions with
//! various traits implemented in the [`traits`] module. You can do all the
//! standard probability distribution stuff like evaluate the PDF/PMF and draw
//! values of different types.
//!
//! The [`prelude`] module provides all the distributions, all the traits, and
//! creates a few useful type aliases.
//!
//! # Design
//!
//! Random variables are designed to be flexible. For example, we don't just
//! want a `Beta` distribution that works with `f64`; we want it to work with a
//! bunch of things
//!
//! ```
//! use rv::prelude::*;
//!
//! // Beta(0.5, 0.5)
//! let beta = Beta::jeffreys();
//!
//! let mut rng = rand::thread_rng();
//!
//! // 100 f64 weights in (0, 1)
//! let f64s: Vec<f64> = beta.sample(100, &mut rng);
//! let pdf_x = beta.ln_pdf(&f64s[42]);
//!
//! // 100 f32 weights in (0, 1)
//! let f32s: Vec<f32> = beta.sample(100, &mut rng);
//! let pdf_y = beta.ln_pdf(&f32s[42]);
//!
//! // 100 Bernoulli distributions -- Beta is a prior on the weight
//! let berns: Vec<Bernoulli> = beta.sample(100, &mut rng);
//! let pdf_bern = beta.ln_pdf(&berns[42]);
//! ```
//!
//! # Examples
//!
//! For more examples, check out the `examples` directory.
//!
//! ## Conjugate analysis of coin flips
//!
//! ```rust
//! use rv::prelude::*;
//!
//! let mut rng = rand::thread_rng();
//!
//! // A sequence of observations
//! let flips = vec![true, false, true, true, true, false, true];
//!
//! // Construct the Jeffreys prior of Beta(0.5, 0.5)
//! let prior = Beta::jeffreys();
//!
//! // Packages the data in a wrapper that marks it as having come from
//! // Bernoulli trials.
//! let obs: BernoulliData<bool> = DataOrSuffStat::Data(&flips);
//!
//! // Generate the posterior distribution P(θ|x); the distribution of
//! // probable coin weights
//! let posterior: Beta = prior.posterior(&obs);
//!
//! // What is the probability that the next flip would come up heads
//! // (true) given the observed flips (posterior predictive)?
//! let p_heads = prior.pp(&true, &obs);
//! ```
#[cfg(feature = "serde1")]
extern crate serde;

// Test the README
use doc_comment::doctest;
doctest!("../README.md");

pub mod consts;
pub mod data;
pub mod dist;
pub mod misc;
mod model;
pub mod prelude;
#[cfg(feature = "process")]
pub mod process;
pub mod test;
pub mod traits;

pub use crate::model::ConjugateModel;

#[macro_export]
macro_rules! impl_display {
    ($kind: ty) => {
        impl ::std::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", String::from(self))
            }
        }
    };
}

#[macro_export]
macro_rules! clone_cache_f64 {
    ($this:ident, $($field:tt)+) => {
        if let Some(&val) = $this.$($field)+.get() {
            OnceCell::from(val)
        } else {
            OnceCell::new()
        }
    }
}

#[macro_export]
macro_rules! extract_stat {
    ($fx: ty, $stat_type: ty) => {
        fn extract_stat(x: &DataOrSuffStat<f64, $fx>) -> $stat_type {
            match x {
                DataOrSuffStat::SuffStat(ref s) => (*s).clone(),
                DataOrSuffStat::Data(xs) => {
                    let mut stat = $stat_type::new();
                    xs.iter().for_each(|y| stat.observe(y));
                    stat
                }
                DataOrSuffStat::None => $stat_type::new(),
            }
        }
    };
}
