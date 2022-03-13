pub fn example(nr: f32, places: usize) -> f32 {
    let multiplier = 10_f32.powi(places as i32);
    (nr * multiplier + 0.5).floor() / multiplier
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn round_positive() {
        assert_eq!(example(3.123456, 2), 3.12);
        assert_eq!(example(3.123456, 4), 3.1235);
        assert_eq!(example(3.999999, 2), 4.0);
        assert_eq!(example(3.0, 2), 3.0);
        assert_eq!(example(9.99999, 2), 10.0);
        assert_eq!(example(0_f32, 2), 0_f32);
    }

    #[test]
    fn round_negative() {
        assert_eq!(example(-3.123456, 2), -3.12);
        assert_eq!(example(-3.123456, 4), -3.1235);
        assert_eq!(example(-3.999999, 2), -4.0);
        assert_eq!(example(-3.0, 2), -3.0);
        assert_eq!(example(-9.99999, 2), -10.0);
    }
}