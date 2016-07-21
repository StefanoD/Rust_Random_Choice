#![cfg_attr(test, feature(test))]

extern crate random_choice;
extern crate test;



#[cfg(test)]
mod benches {    
    use test::Bencher;
    use random_choice::RandomChoice;

    #[bench]
    fn bench_random_choice_64(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            RandomChoice::random_choice_f64(&samples, &weights, 1200 as usize);
        });
    }

    #[bench]
    fn bench_random_choice_in_place_64(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            RandomChoice::random_choice_in_place_f64(&mut samples, &weights);
        });
    }

    #[bench]
    fn bench_random_choice_32(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            RandomChoice::random_choice_f32(&samples, &weights, 1200 as usize);
        });
    }

    #[bench]
    fn bench_random_choice_in_place_32(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            RandomChoice::random_choice_in_place_f32(&mut samples, &weights);
        });
    }
}
