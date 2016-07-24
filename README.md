![](https://travis-ci.org/StefanoD/Rust_Random_Choice.svg?branch=master)

# Rust Random Choice
Chooses samples randomly by their weights/probabilities.

### Advantages

- There is a good diversity for the case that all weights are equally distributed (in contrast to the roulette wheel selection algorithm which tends to select the same sample n times)
- Blazingly fast: O(n) (Roulette wheel selection algorithm: O(n * log n))
- Memory Usage: O(n); in place variant: O(1)
- The sum of the weights don't have to be 1.0, but must not overflow

This algorithm is based on the stochastic universal sampling algorithm.

### Applications
- **Evolutionary algorithms**: Choose the _n_ fittest populations by their fitness **_fi_**
- **Monte Carlo Localization**: Resampling of _n_ particles by their weight **_w_**

## Examples
### In Place Variant
```rust
extern crate random_choice;
use self::random_choice::RandomChoice;

let mut samples = vec!["hi", "this", "is", "a", "test!"];
let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

RandomChoice::random_choice_in_place_f64(&mut samples, &weights);
 
for sample in samples {
    print!("{}, ", sample);
}
```
### N Selection Variant
```rust
extern crate random_choice;
use self::random_choice::RandomChoice;

let capacity: usize = 500;
let mut samples: Vec<usize> = Vec::with_capacity(capacity);
let mut weights: Vec<f64> = Vec::with_capacity(capacity);

for i in 0..capacity {
    samples.push(i);
    weights.push(i as f64);
}

let number_choices = 10000;
let choices = RandomChoice::random_choice_f64(&samples, &weights, number_choices);

assert!(choices.len() == number_choices);

for choice in choices {
    print!("{}, ", choice);
}
```
