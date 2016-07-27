// Licensed under the Apache License, Version 2.0
// <http://www.apache.org/licenses/LICENSE-2.0>

//! # Rust Random Choice
//! This is an implementation of the stochastic universal sampling algorithm:
//! https://en.wikipedia.org/wiki/Stochastic_universal_sampling
//!
//! ## Advantages
//! - Blazingly fast: O(n) (Roulette wheel selection algorithm: O(n * log n))
//! - Low Memory Usage: O(n); in place variant: O(1)
//! - There is a good diversity for the case, that all weights are equally distributed (in contrast to the roulette wheel selection algorithm which tends to select the same sample n times)
//! - The sum of the weights don't have to be 1.0, but must not overflow
//!
//! ## Applications
//! - **Evolutionary algorithms**: Choose the _n_ fittest populations by their fitness **_fi_**
//! - **Monte Carlo Localization**: Resampling of _n_ particles by their weight **_w_**
//!
//! # Examples
//! ## In Place Variant
//! ```
//! extern crate random_choice;
//! use self::random_choice::random_choice;
//!
//! # fn main() {
//! let mut samples = vec!["hi", "this", "is", "a", "test!"];
//! let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];
//!
//! random_choice().random_choice_in_place_f64(&mut samples, &weights);
//!
//! for sample in samples {
//!     print!("{}, ", sample);
//! }
//! # }
//! ```
//! ## N Selection Variant
//! ```
//! extern crate random_choice;
//! use self::random_choice::random_choice;
//!
//!# fn main() {
//! let capacity: usize = 500;
//! let mut samples: Vec<usize> = Vec::with_capacity(capacity);
//! let mut weights: Vec<f64> = Vec::with_capacity(capacity);
//!
//! for i in 0..capacity {
//!     samples.push(i);
//!     weights.push(i as f64);
//! }
//!
//! let number_choices = 10000;
//! let choices = random_choice().random_choice_f64(&samples, &weights, number_choices);
//!
//! assert!(choices.len() == number_choices);
//!
//! for choice in choices {
//!     print!("{}, ", choice);
//! }
//! # }
//! ```
//! ## With custom seed
//! ```
//! extern crate rand;
//! extern crate random_choice;
//! use random_choice::RandomChoice;
//! use rand::SeedableRng;

//! fn main() {
//!     let mut samples = vec!["hi", "this", "is", "a", "test!"];
//!     let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];
//! 
//!     let rng = rand::StdRng::from_seed(&[5000, 44, 55, 199]);
//!
//!     let mut random_choice = RandomChoice::new(rng);
//!     random_choice.random_choice_in_place_f64(&mut samples, &weights);

//!     for sample in samples {
//!         print!("{}, ", sample);
//!     }
//!#}
//! ```

extern crate rand;

use self::rand::{thread_rng, ThreadRng, Rng};

pub struct RandomChoice<RNG: Rng> {
    rng: RNG,
}

/// Creates a new RandomChoice struct using the ThreadRng
pub fn random_choice() -> RandomChoice<ThreadRng> {
    RandomChoice::new(thread_rng())
}


impl<RNG: Rng> RandomChoice<RNG> {
    /// Creates a new RandomChoice struct.
    /// @param rng the random number generator to use with this stuct.
    pub fn new(rng: RNG) -> Self {
        RandomChoice { rng: rng }
    }

    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    ///
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    /// @param n Number of randomly chosen samples by weight.
    /// @return randomly selected samples by their weights
    pub fn random_choice_f64<'a, T>(&mut self,
                                    samples: &'a [T],
                                    weights: &[f64],
                                    n: usize)
                                    -> Vec<&'a T> {
        if weights.len() == 0 || n == 0 {
            return Vec::new();
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = self.rng.next_f64() * spoke_gap;

        let mut i: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut choices: Vec<&T> = Vec::with_capacity(n);
        let mut current_spoke: f64 = spin;

        while current_spoke < sum {
            while accumulated_weights < current_spoke {
                i += 1;
                accumulated_weights += weights[i];
            }
            choices.push(&samples[i]);
            current_spoke += spoke_gap;
        }

        // add this condition, because float leads to inaccurate
        // calculations which can miss some samples
        while choices.len() < weights.len() {
            choices.push(&samples[i]);
        }

        choices
    }

    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    /// The result gets saved directly in the samples argument.
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    pub fn random_choice_in_place_f64<T: Clone>(&mut self, samples: &mut [T], weights: &[f64]) {
        if weights.len() < 2 {
            return;
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let n: usize = weights.len();
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = self.rng.next_f64() * spoke_gap;

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut current_spoke: f64 = spin;

        while current_spoke < sum {
            while accumulated_weights < current_spoke {
                j += 1;
                accumulated_weights += weights[j];
            }
            samples[i] = samples[j].clone();
            current_spoke += spoke_gap;
            i += 1
        }

        // add this condition, because float leads to inaccurate
        // calculations which can miss some samples
        while i < weights.len() {
            samples[i] = samples[j].clone();
            i += 1
        }

    }

    pub fn random_choice_f32<'a, T>(&mut self,
                                    samples: &'a [T],
                                    weights: &[f32],
                                    n: usize)
                                    -> Vec<&'a T> {
        if weights.len() == 0 || n == 0 {
            return Vec::new();
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i as f64);
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = self.rng.next_f64() * spoke_gap;

        let mut i: usize = 0;
        let mut accumulated_weights = weights[0] as f64;
        let mut choices: Vec<&T> = Vec::with_capacity(n);
        let mut current_spoke: f64 = spin;

        while current_spoke < sum {
            while accumulated_weights < current_spoke {
                i += 1;
                accumulated_weights += weights[i] as f64;
            }
            choices.push(&samples[i]);
            current_spoke += spoke_gap;
        }

        // add this condition, because float leads to inaccurate
        // calculations which can miss some samples
        while choices.len() < weights.len() {
            choices.push(&samples[i]);
        }

        choices
    }

    pub fn random_choice_in_place_f32<T: Clone>(&mut self, samples: &mut [T], weights: &[f32]) {
        if weights.len() < 2 {
            return;
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i as f64);
        let n: usize = weights.len();
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = self.rng.next_f64() * spoke_gap;

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut accumulated_weights = weights[0] as f64;
        let mut current_spoke: f64 = spin;

        while current_spoke < sum {
            while accumulated_weights < current_spoke {
                j += 1;
                accumulated_weights += weights[j] as f64;
            }
            samples[i] = samples[j].clone();
            current_spoke += spoke_gap;
            i += 1
        }

        // add this condition, because float leads to inaccurate
        // calculations which can miss some samples
        while i < weights.len() {
            samples[i] = samples[j].clone();
            i += 1
        }
    }
}