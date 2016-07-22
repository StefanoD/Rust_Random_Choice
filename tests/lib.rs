extern crate random_choice;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use random_choice::RandomChoice;    

    #[test]
    fn test_random_choice_64() {
        let capacity: usize = 1000;
        let mut samples: Vec<usize> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push(i + 1);
            weights.push((i + 1usize) as f64);
        }

        let choices = RandomChoice::random_choice_f64(&samples, &weights, 4 as usize);

        let mut weight_counter = HashMap::with_capacity(capacity);

        for choice in choices {
            let counter = weight_counter.entry(choice).or_insert(0);
            *counter += 1;
        }

        for key in weight_counter.keys() {
            println!("{}", key);
        }
    }

    #[test]
    fn test_random_choice_in_place_64() {
        let mut samples = vec!["hi", "this", "is", "a", "test!"];
        let weights: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        RandomChoice::random_choice_in_place_f64(&mut samples, &weights);

        for sample in samples {
            print!("{}, ", sample);
        }
    }

}