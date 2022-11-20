#[allow(unused)]
fn part_1(input: &str) -> i64 {
    todo!()
}

#[allow(unused)]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod unit_tests {
    #[allow(unused)]
    use super::*;
}

#[cfg(test)]
mod acceptance_tests {

    use super::*;

    const EXAMPLE: &str = "";

    const INPUT: &str = include_str!("day08_input.txt");

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_1(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(part_1(input), expected);
    }

    #[rstest]
    #[ignore]
    #[case::example(EXAMPLE, 0)]
    #[ignore]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(part_2(input), expected);
    }
}
