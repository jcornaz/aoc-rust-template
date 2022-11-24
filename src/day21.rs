type Output = u64;

pub fn part_1(input: &str) -> Output {
    input.parse().unwrap()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"

    "#;

    const INPUT: &str = include_str!("day21/input.txt");

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }
}
