/**
 * Fibonacci sequence using tabulation
 */
pub fn fibonacci(number: usize) -> usize {
    match number {
        0 => 0,
        1 => 1,
        _ => {
            let mut table: Vec<usize> = vec![0; number + 1];
            table[1] = 1;
            for i in 2..=number {
                table[i] = table[i - 1] + table[i - 2];
            }
            table[number]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn can_get_fibonacci_of_zero() {
        let result = fibonacci(0);
        assert_eq!(result, 0, "Result: {}", result);
    }

    #[test]
    fn can_get_fibonacci_of_six() {
        let result = fibonacci(6);
        assert_eq!(result, 8, "Result: {}", result);
    }

    #[test]
    fn can_get_fibonacci_of_seven() {
        let result = fibonacci(7);
        assert_eq!(result, 13, "Result: {}", result);
    }

    #[test]
    fn can_get_a_big_fibonacci_number() {
        let result = fibonacci(70);
        assert_eq!(result, 190392490709135, "Result: {}", result);
    }
}
