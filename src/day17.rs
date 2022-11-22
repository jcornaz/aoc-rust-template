type Output = u64;

pub fn part_1(input: &str) -> Result<Output, String> {
    input.parse::<Output>().map_err(|e| e.to_string())
}

pub fn part_2(input: &str) -> Result<Output, String> {
    input.parse::<Output>().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day01::part_1;

    use super::*;

    const EXAMPLE: &str = r#"

    "#;

    const INPUT: &str = include_str!("day17/input.txt");

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()).unwrap(), expected);
    }

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()).unwrap(), expected);
    }
}
