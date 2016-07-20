/// Implementation of Stochastic universal sampling
/// https://en.wikipedia.org/wiki/Stochastic_universal_sampling
/// Runtime: O(n)
/// Memory Usage: O(n) or O(1) (in place variant) 

extern crate rand;

use self::rand::{thread_rng, Rng};

pub struct RandomChoice;


impl RandomChoice {
    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    ///
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    /// @param n Number of randomly chosen samples by weight.
    /// @return randomly selected samples by their weights
    pub fn random_choice<'a, T>(samples: &'a [T], weights: &[f64], n: usize) -> Vec<&'a T> {
        if weights.len() == 0 || n == 0 { return Vec::new(); }

        let sum:f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f64() * spoke_gap;
        
        let mut i: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut choices: Vec<&T> = Vec::with_capacity(n);
        let mut current_spoke: f64 = spin;

        for _ in 0 .. n {
            while accumulated_weights < current_spoke {
                i += 1;
                accumulated_weights += weights[i];
            }
            choices.push(&samples[i]);
            current_spoke += spoke_gap;
        }

        choices
    }

    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    /// The result gets saved directly in the samples argument.
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    pub fn random_choice_in_place<T: Clone>(samples: &mut [T], weights: &[f64]) {
        if weights.len() < 2 { return; }

        let sum:f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let n: usize = weights.len();
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f64() * spoke_gap;
        
        let mut j: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut current_spoke: f64 = spin;

        for i in 0 .. n {
            while accumulated_weights < current_spoke {
                j += 1;
                accumulated_weights += weights[j];
            }
            samples[i] = samples[j].clone();
            current_spoke += spoke_gap;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_random_choice() {
        let test_vec: Vec<f64> = vec![3.1, 1.2, 1.3, 2.0];
        //let sum:f64 = test_vec.iter().fold(0.0, |acc, &i| acc + i);

        //assert_eq!(6.0, sum);

        let choices = super::RandomChoice::random_choice(&test_vec, &test_vec, 4 as usize);

        for choice in choices {
            print!("{}, ", choice);
        }
    }

    #[test]
    fn test_random_choice_in_place() {
        let mut samples = vec!["hi", "this", "is", "a", "test!"];
        let weights: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        super::RandomChoice::random_choice_in_place(&mut samples, &weights);

        for sample in samples {
            print!("{}, ", sample);
        }
    }

    #[test]
    fn test_random_choice_in_place_2() {
        struct Foo {j: i32, };
        let test1 = Foo {j: 1};
        let test2 = Foo {j: 2};
        let test3 = Foo {j: 3}; 
        let mut samples: Vec<&Foo> = Vec::new();
        
        samples.push(&test1);
        samples.push(&test2);
        samples.push(&test3);
        
        let weights: Vec<f64> = vec![1.0, 1.0, 1.0];

        super::RandomChoice::random_choice_in_place(&mut samples, &weights);

        for sample in samples {
            print!("foo{}, ", sample.j);
        }
    }
}