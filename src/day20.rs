pub fn part_1(input: &str) -> anyhow::Result<u64> {
    Ok(input.parse()?)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    Ok(input.parse()?)
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "";

    const INPUT: &str = include_str!("day20_input.txt");

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part_1(input.trim()).unwrap(), expected);
    }

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part_2(input.trim()).unwrap(), expected);
    }
}
