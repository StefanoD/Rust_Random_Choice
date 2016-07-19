extern crate rand;

use self::rand::{thread_rng, Rng};

/**
* Chooses n samples by their weights. The greater their weights the more likely they get chosen.
*
* @invariant sum of weights must not overflow.
* @param samples The to be selected samples
* @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
* @param n Number of randomly chosen indices by weight.
* @return randomly selected samples by their weight
*/
pub fn random_choice<'a, T>(samples: &'a Vec<T>, weights: &Vec<f64>, n: usize) -> Vec<&'a T>{
    // TODO Check, if weight.len() > 0 and n > 0

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


#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        let test_vec: Vec<f64> = vec![3.1, 1.2, 1.3, 2.0];
        //let sum:f64 = test_vec.iter().fold(0.0, |acc, &i| acc + i);

        //assert_eq!(6.0, sum);

        let choices = super::random_choice(&test_vec, &test_vec, 4 as usize);

        for choice in choices {
            print!("{}, ", choice);
        }
    }
}