use std::f64;

use criterion::Benchmark;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rand::distributions::Beta;
use rv::traits::Rv;

fn draw_rand_distr<R: rand::Rng>(rng: &mut R) -> f64 {
    let beta = Beta::new(5.0, 2.0);
    rng.sample(beta)
}

fn draw_rv<R: rand::Rng>(mut rng: &mut R) -> f64 {
    let beta = rv::dist::Beta::new(5.0, 2.0).unwrap();
    beta.draw(&mut rng)
}

fn draw_2u<R: rand::Rng>(rng: &mut R) -> f64 {
    let a = rng.gen::<f64>().powf(1.0 / 5.0);
    let b = rng.gen::<f64>().powf(1.0 / 2.0);
    a / (a + b)
}

fn draw_2u_recip<R: rand::Rng>(rng: &mut R) -> f64 {
    let a = rng.gen::<f64>().powf(5.0_f64.recip());
    let b = rng.gen::<f64>().powf(2.0_f64.recip());

    a / (a + b)
}

fn bench_beta_draw(c: &mut Criterion) {
    c.bench(
        "beta_draw",
        Benchmark::new("rand_distr", |b| {
            let mut rng = rand::thread_rng();
            b.iter(|| draw_rand_distr(&mut rng))
        })
        .with_function("draw_rv", |b| {
            let mut rng = rand::thread_rng();
            b.iter(|| draw_rv(&mut rng))
        })
        .with_function("draw_2_uniform", |b| {
            let mut rng = rand::thread_rng();
            b.iter(|| draw_2u(&mut rng))
        })
        .with_function("draw_2_uniform_recip", |b| {
            let mut rng = rand::thread_rng();
            b.iter(|| draw_2u_recip(&mut rng))
        }),
    );
}

criterion_group!(beta_benches, bench_beta_draw);
criterion_main!(beta_benches);
