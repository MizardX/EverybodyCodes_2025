use std::num::ParseIntError;

pub struct Day06;

impl crate::Day for Day06 {
    type Input = String;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        Ok(input.to_string())
    }

    fn part_1(input: &Self::Input) -> usize {
        number_of_pairings(input, 1, input.len(), 0)[0]
    }

    fn part_2(input: &Self::Input) -> usize {
        number_of_pairings(input, 1, input.len(), 0)
            .into_iter()
            .sum()
    }

    fn part_3(input: &Self::Input) -> usize {
        number_of_pairings_shortcut(input, 1000, 1000, 1000)
            .into_iter()
            .sum()
    }
}

fn number_of_pairings_shortcut(
    input: &str,
    cycles: usize,
    behind: usize,
    ahead: usize,
) -> [usize; 3] {
    let until_repeat = (ahead + behind + 1).div_ceil(input.len());
    if cycles <= until_repeat * 2 {
        number_of_pairings(input, cycles, behind, ahead)
    } else {
        let first = number_of_pairings(input, until_repeat, behind, ahead);
        let second = number_of_pairings(input, until_repeat + 1, behind, ahead);
        [0, 1, 2].map(|ix| first[ix] + (second[ix] - first[ix]) * (cycles - until_repeat))
    }
}

fn number_of_pairings(input: &str, cycles: usize, behind: usize, ahead: usize) -> [usize; 3] {
    let mut mentors = [0_usize; 3];
    let mut pairs = [0_usize; 3];
    let len = input.len();
    // Pad with 'x' such that we get our first value by the `behind`'th squire
    let mentors_behind = std::iter::repeat_n(b'x', behind + ahead).chain(input.bytes().cycle());
    // Pad with 'x' such that we get our first value by the `ahead`'th position.
    let squires = std::iter::repeat_n(b'x', ahead).chain(input.bytes().cycle().take(len * cycles));
    // Make sure we don't cycle too long, and pad with 'x' at the end
    let mentors_ahead = input
        .bytes()
        .cycle()
        .take(len * cycles)
        .chain(std::iter::repeat_n(b'x', ahead));
    for ((remove, query), add) in mentors_behind.zip(squires).zip(mentors_ahead) {
        if let mentor @ b'A'..=b'C' = add {
            mentors[(mentor - b'A') as usize] += 1;
        }
        if let squire @ b'a'..=b'c' = query {
            pairs[(squire - b'a') as usize] += mentors[(squire - b'a') as usize];
        }
        if let mentor @ b'A'..=b'C' = remove {
            mentors[(mentor - b'A') as usize] -= 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;
    use test_case::test_case;

    #[test_case("ABabACacBCbca" => 5)]
    fn test_part_1(input: &str) -> usize {
        Day06::part_1(&input.to_string())
    }

    #[test_case("ABabACacBCbca" => 11)]
    fn test_part_2(input: &str) -> usize {
        Day06::part_2(&input.to_string())
    }

    #[test_case("AABCBABCABCabcabcABCCBAACBCa", 10, 1 => 34)]
    #[test_case("AABCBABCABCabcabcABCCBAACBCa", 10, 2 => 72)]
    #[test_case("AABCBABCABCabcabcABCCBAACBCa", 1_000, 1_000 => 3_442_321)]
    fn test_part_3(input: &str, dist_limit: usize, cycles: usize) -> usize {
        number_of_pairings_shortcut(input, cycles, dist_limit, dist_limit)
            .into_iter()
            .sum()
    }
}
