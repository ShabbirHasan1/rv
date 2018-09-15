extern crate rand;
extern crate rv;

use rand::Rng;
use rv::data::DataOrSuffStat;
use rv::dist::{Bernoulli, Beta};
use rv::prelude::BernoulliData;
use rv::traits::*;
use rv::ConjugateModel;
use std::sync::Arc;

fn main() {
    let u = rand::distributions::Open01;
    let mut rng = rand::thread_rng();

    // Generate some 1000 coin flips from a coin that will come up head 70%
    // of the time.
    let flips: Vec<bool> = (0..1000)
        .map(|_| {
            let x: f64 = rng.sample(u);
            x < 0.7
        }).collect();

    // Use the Jeffreys prior of Beta(0.5, 0.5)
    let prior = Beta::jeffreys();
    let xs: BernoulliData<bool> = DataOrSuffStat::Data(&flips);

    // Generate the posterior distributoin P(θ|x)
    let posterior = prior.posterior(&xs);

    // Print the mean. The posterior mean for Bernoulli likelihood with Beta
    // prior.
    let posterior_mean: f64 = posterior.mean().expect("Mean undefined");
    println!(
        "Posterior mean: {} (should be close to 0.7)",
        posterior_mean
    );

    // Samw thing, only using ConjugateModel
    let mut model: ConjugateModel<bool, Bernoulli, Beta> =
        ConjugateModel::new(&Bernoulli::uniform(), Arc::new(prior));

    // Show the data to to the model
    model.observe_many(&flips);

    // draw from the posterior predictive
    let ys = model.sample(10, &mut rng);
    println!("Posterior predictive samples: {:?}", ys);
}
