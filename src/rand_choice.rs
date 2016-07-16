extern crate rand;

use self::rand::{thread_rng, Rng};

/**
* Chooses n samples of weights. The greater the weight the more likely it gets chosen.
* This function returns only the indices of the weights.
*
* @invariant sum of weights must not overflow.
*
* @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
* @param n Number of randomly chosen indices by weight.
* @return Indices of the weights. Randomly chosen by their weight.
*/
pub fn random_choice(weights: Vec<f64>, n: usize) -> Vec<usize>{
    // Save accumulated sum of the weights in order to calculate the intervals
    // between accumulated weights
    let mut accumulated_sum = Vec::with_capacity(weights.len());

    accumulated_sum.push(weights[0]);

    for i in 1 .. weights.len() {
        let acc_sum = accumulated_sum[i - 1] + weights[i];
        accumulated_sum.push(acc_sum);
    }

    let sum: f64 = accumulated_sum[weights.len() - 1];
    let spoke_gap: f64 = sum / n as f64;

    // next_f64() ∈ [0.0, 1.0)
    let spin = thread_rng().next_f64() * spoke_gap;
    let mut choices: Vec<usize> = Vec::with_capacity(n);
    let mut i: usize = 0;
    let mut current_spoke: f64 = spin;

    for _ in 0 .. n {
        current_spoke += spoke_gap;
        current_spoke %= sum;

        // accumulated_sum[i] is the interval end
        // (accumulated_sum[i] - weight[i]) is the interval begin
        // Following condition must be true: interval begin <= current_spoke < interval end
        // If the condition is not true, then increment i in order to get the next greater intervals.
        while accumulated_sum[i] <= current_spoke || (accumulated_sum[i] - weights[i]) > current_spoke {
            i += 1;
            i %= weights.len();
        }
        choices.push(i);
    }

    choices
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        let test_vec = vec![1.0, 2.0, 3.0, 10.0];
        //let sum:f64 = test_vec.iter().fold(0.0, |acc, &i| acc + i);

        //assert_eq!(6.0, sum);

        let indices = super::random_choice(test_vec, 10 as usize);

        for index in indices {
            print!("{}, ", index);
        }
    }
}