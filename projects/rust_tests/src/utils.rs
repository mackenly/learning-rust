pub mod inputs {
    #[allow(dead_code)]
    pub(crate) fn prompt_for_string(prompt: &str) -> &str {
        println!("{}", prompt);
        let input: &str = "";
        std::io::stdin()
            .read_line(&mut input.to_string())
            .expect("Failed to read line");
        input.trim()
    }
}

pub mod math {
    #[allow(dead_code)]
    pub(crate) fn adder(a: i32, b: i32) -> i32 {
        a + b
    }

    #[allow(dead_code)]
    pub(crate) fn subtractor(a: i32, b: i32) -> i32 {
        a - b
    }

    #[allow(dead_code)]
    pub(crate) fn multiplier(a: i32, b: i32) -> i32 {
        a * b
    }

    #[allow(dead_code)]
    pub(crate) fn divider(a: i32, b: i32) -> i32 {
        a / b
    }

    #[allow(dead_code)]
    pub(crate) fn squarer(a: i32) -> i32 {
        a * a
    }

    #[allow(dead_code)]
    pub(crate) fn cuber(a: i32) -> i32 {
        a * a * a
    }

    #[allow(dead_code)]
    pub(crate) fn power(a: i32, b: i32) -> i32 {
        let mut result: i32 = 1;
        for _ in 0..b {
            result *= a;
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_adder() {
            assert_eq!(adder(1, 2), 3);
        }

        #[test]
        fn test_subtractor() {
            assert_eq!(subtractor(1, 2), -1);
        }

        #[test]
        fn test_multiplier() {
            assert_eq!(multiplier(1, 2), 2);
        }

        #[test]
        fn test_divider() {
            assert_eq!(divider(1, 2), 0);
        }

        #[test]
        fn test_squarer() {
            assert_eq!(squarer(2), 4);
        }

        #[test]
        fn test_cuber() {
            assert_eq!(cuber(2), 8);
        }

        #[test]
        fn test_power() {
            assert_eq!(power(2, 3), 8);
        }
    }
}
