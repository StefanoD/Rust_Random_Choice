extern crate random_choice;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use random_choice::RandomChoice;

    #[test]
    fn test_random_choice_f64() {
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

        let mut weight_counter = BTreeMap::new();

        for choice in choices {
            let counter = weight_counter.entry(choice).or_insert(0);
            *counter += 1;
        }

        let mut last_value: usize = 0;

        for (_, value) in &weight_counter {
            assert!((last_value as i32 - (*value) as i32).abs() <= 2);
            last_value = *value;
            //println!("({}, {})", key, value);
        }
    }

    #[test]
    fn test_random_choice_f32() {
        let capacity: usize = 500;
        let mut samples: Vec<usize> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push(i);
            weights.push(i as f32);
        }

        let number_choices = 10000;
        let choices = RandomChoice::random_choice_f32(&samples, &weights, number_choices);

        assert!(choices.len() == number_choices);

        let mut weight_counter = BTreeMap::new();

        for choice in choices {
            let counter = weight_counter.entry(choice).or_insert(0);
            *counter += 1;
        }

        let mut last_value: usize = 0;

        for (_, value) in &weight_counter {
            assert!((last_value as i32 - (*value) as i32).abs() <= 2);
            last_value = *value;
            //println!("({}, {})", key, value);
        }
    }

    #[test]
    fn test_random_choice_zero_elements_f64() {
        let capacity: usize = 1000;
        let mut samples: Vec<usize> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push(i + 1);
            weights.push((i + 1usize) as f64);
        }

        let choices = RandomChoice::random_choice_f64(&samples, &weights, 0 as usize);

        assert!(choices.len() == 0);
    }

    #[test]
    fn test_random_choice_in_place_f64() {
        let mut samples = vec!["hi", "this", "is", "a", "test!"];
        let weights: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        RandomChoice::random_choice_in_place_f64(&mut samples, &weights);

        for sample in samples {
            print!("{}, ", sample);
        }
    }

}