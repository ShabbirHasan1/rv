//! Chinese Restaurant Process
//!
//! [The Chinese Restaurant Process](https://en.wikipedia.org/wiki/Chinese_restaurant_process) (CRP)
//! is a distribution over partitions of items. The CRP defines a process by
//! which entities are assigned to an unknown number of partition.
//!
//! The CRP is parameterized CRP(α) where α is the 'discount' parameter in
//! (0, ∞). Higher α causes there to be more partitions, as it encourages new
//! entries to create new partitions.
extern crate rand;
extern crate special;

use self::rand::Rng;
use self::special::Gamma as SGamma;
use partition::Partition;
use traits::*;
use utils::pflip;

/// Chinese Restaurant Process
pub struct Crp {
    /// Discount parameter
    pub alpha: f64,
    /// number of items in the partition
    pub n: usize,
}

impl Crp {
    /// Create an empty `Crp` with parameter alpha
    pub fn new(alpha: f64, n: usize) -> Self {
        Crp { alpha: alpha, n: n }
    }
}

impl Rv<Partition> for Crp {
    fn ln_f(&self, x: &Partition) -> f64 {
        let gsum = x
            .counts
            .iter()
            .fold(0.0, |acc, ct| acc + (*ct as f64).ln_gamma().0);

        gsum + (x.k() as f64) * self.alpha.ln() + self.alpha.ln_gamma().0
            - (x.len() as f64 + self.alpha).ln_gamma().0
    }

    #[inline]
    fn ln_normalizer(&self) -> f64 {
        0.0
    }

    fn draw<R: Rng>(&self, rng: &mut R) -> Partition {
        let mut k = 1;
        let mut weights: Vec<f64> = vec![1.0];
        let mut z: Vec<usize> = Vec::with_capacity(self.n);
        z.push(0);

        for _ in 1..self.n {
            weights.push(self.alpha);
            let zi = pflip(&weights, 1, rng)[0];
            z.push(k);

            if zi == k {
                weights[zi] = 1.0;
                k += 1;
            } else {
                weights.truncate(k);
                weights[zi] += 1.0;
            }
        }
        // convert weights to counts, correcting for possible floating point
        // errors
        let counts: Vec<usize> =
            weights.iter().map(|w| (w + 0.5) as usize).collect();

        Partition {
            z: z,
            counts: counts,
        }
    }
}

impl Support<Partition> for Crp {
    #[inline]
    fn contains(&self, _x: &Partition) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    extern crate assert;
    use super::*;

    const TOL: f64 = 1E-12;

    #[test]
    fn new() {
        let crp = Crp::new(1.2, 808);
        assert::close(crp.alpha, 1.2, TOL);
        assert_eq!(crp.n, 808);
    }
}
