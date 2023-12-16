#[macro_export]
macro_rules! test_real_input {
    ($(#[$attrs:meta])* $day:literal, $answer_a:expr) => {
        test_real_input!($(#[$attrs])* $day, $answer_a, None);
    };
    ($(#[$attrs:meta])* $day:literal, $answer_a:expr, $answer_b:expr) => {
        $(#[$attrs])*
        #[test]
        fn test_real_input() {
            let (a, b) = match main(format!("data/day{}.txt", $day).as_ref()) {
                Ok(answers) => answers,
                Err(e) => panic!("Solution failed to complete: {}", e),
            };

            let expected_answer_a = $answer_a;
            let expected_answer_b = Option::from($answer_b);

            if expected_answer_b.is_none() {
                assert_eq!(a, expected_answer_a);
            } else {
                assert_eq!((a, b), (expected_answer_a, expected_answer_b));
            }
        }
    };
}
