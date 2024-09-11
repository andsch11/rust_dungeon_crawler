#[cfg(test)]
mod my_test_random_number_generator {
    use crate::model::random_number_generator::{GenerateRandomNumber, PreProgrammedRandomNumbers};
    use std::collections::VecDeque;

    #[test]
    fn test_basic() {
        let mut pprng = PreProgrammedRandomNumbers::new(VecDeque::from([0.1, 0.2, 0.3]));
        assert_eq!(pprng.range(0, 1), 0);
        assert_eq!(pprng.range(0, 10), 2);
        assert_eq!(pprng.range(0, 100), 30);
    }
}
