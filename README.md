# rv

Random variables for rust

## Design

Random variables are designed to be flexible. For example, we don't just want a
`Beta` distribution that works with `f64`; we want it to work with a bunch of
things like


```rust
extern crate rand;
extern crate rv;

use rv::prelude::*;

// Beta(0.5, 0.5)
let beta = Beta::jeffreys();

let mut rng = rand::thread_rng();

// 100 f64 weights in (0, 1)
let f64s: Vec<f64> = beta.sample(100, &mut rng);
let pdf_x = beta.ln_pdf(&f64s[42]);

// 100 f32 weights in (0, 1)
let f32s: Vec<f32> = beta.sample(100, &mut rng);
let pdf_y = beta.ln_pdf(&f32s[42]);

// 100 Bernoulli distributions -- Beta is a prior on the weight
let berns: Vec<Bernoulli> = beta.sample(100, &mut rng);
let pdf_bern = beta.ln_pdf(&berns[42]);
```

## Contributing

1. All PRs should be branched off `dev`.
2. Please create an issue before starting any work. We're far from stable, so
   we might actually be working on what you want, or we might be working on
   something that will change the way you might implement it.
3. If you plan on implementing a new distribution, implement at least `Rv`,
   `Support`, and either `ContinuousDistr` or `DiscreteDistr`. Of course, more
   is better!
4. Implement new distributions for the appropriate types. For example, don't
   just implement `Rv<f64>`, also implement `Rv<f32>`. Check out other
   distributions to see how it can be done easily with macros.
5. Write tests, docs, and doc tests.
6. Use `rustfmt`. We've included a `.rustfmt.toml` in the project directory.
