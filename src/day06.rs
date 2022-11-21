mod parser;
mod part_1;
mod part_2;

type Input = Vec<u64>;
type Output = u64;

pub fn part_1(input: &str) -> anyhow::Result<Output> {
    let input = parser::parse(input)?;
    Ok(part_1::run(input))
}

pub fn part_2(input: &str) -> anyhow::Result<Output> {
    let input = parser::parse(input)?;
    Ok(part_2::run(input))
}

#[cfg(test)]
mod tests {
    use crate::day01::part_1;

    use super::*;

    const EXAMPLE: &str = "";

    const INPUT: &str = include_str!("day06/input.txt");

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()).unwrap(), expected);
    }

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()).unwrap(), expected);
    }
}
