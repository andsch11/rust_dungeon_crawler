use bracket_lib::random::RandomNumberGenerator;
use std::collections::VecDeque;

pub trait GenerateRandomNumber {
    fn range(&mut self, min: usize, max: usize) -> usize;
}

pub struct PreProgrammedRandomNumbers {
    pre_programmed_random_numbers: VecDeque<f32>,
}

impl PreProgrammedRandomNumbers {
    #[allow(dead_code)]
    pub fn new(pre_programmed_random_numbers: VecDeque<f32>) -> Self {
        pre_programmed_random_numbers.iter().for_each(|x| {
            assert!(*x >= 0f32);
            assert!(*x <= 1f32);
        });
        Self {
            pre_programmed_random_numbers,
        }
    }
}

impl GenerateRandomNumber for PreProgrammedRandomNumbers {
    fn range(&mut self, min: usize, max: usize) -> usize {
        let delta = max - min;
        let factor = self.pre_programmed_random_numbers.pop_front().unwrap();
        let scaled_delta = (delta as f32 * factor) as usize;
        min + scaled_delta
    }
}

pub struct RealRng {
    rng: RandomNumberGenerator,
}
impl RealRng {
    pub fn new() -> Self {
        let rng = RandomNumberGenerator::new();
        Self { rng }
    }
}

impl GenerateRandomNumber for RealRng {
    fn range(&mut self, min: usize, max: usize) -> usize {
        if min == max {
            return min;
        }
        self.rng.range(min, max)
    }
}
