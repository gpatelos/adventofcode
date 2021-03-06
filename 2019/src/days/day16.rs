#![allow(unused_variables, dead_code)]
use crate::util::iter::IteratorExtensions;
use itertools::unfold;
use num_bigint::BigInt;
use num_traits::One;
use std::convert::TryFrom;

const INPUT: &str = include_str!("../../static/day16.txt");
// const INPUT: &str = "03036732577212944063491565474664";
// const INPUT: &str = "40002357";

const NUM_PHASES: usize = 100;
const OUTPUT_NUM_DIGITS: usize = 8;
// const OUTPUT_NUM_DIGITS: usize = 4;
const OFFSET_NUM_DIGITS: usize = 7;
// const OFFSET_NUM_DIGITS: usize = 1;
const NUM_REPEATS: usize = 10_000;
// const NUM_REPEATS: usize = 1;

type Value = isize;
type Digit = isize;
const BASE_PATTERN: [Digit; 4] = [0, 1, 0, -1];

// Notes: print(np.matrix([[0 if col - row < 0 else math.comb(n + col - row - 1, col - row) for col in range(4, 8)] for row in range(4, 8)]))
// A8 = np.matrix([[1, 0, -1, 0, 1, 0, -1, 0], [0, 1, 1, 0, 0, -1, -1, 0], [0, 0, 1, 1, 1, 0, 0, 0], [0, 0, 0, 1, 1, 1, 1, 0], [0, 0, 0, 0, 1, 1, 1, 1], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 0, 0, 1]])

pub fn main() {
    // let answer1 = solve1(INPUT);
    // println!("{}", answer1);
    let answer2 = solve2(INPUT);
    println!("{}", answer2);
}

fn solve1(input: &str) -> String {
    let input = input.trim();
    let input = digits(input);
    let output = fft(&input, NUM_PHASES);
    println!("{:?}", &output[output.len() - 8..]);
    let output_digits = &output[..OUTPUT_NUM_DIGITS];
    // let first_8_digits = &output[..8];
    // first_8_digits
    output_digits
        .iter()
        .map(|digit| digit.to_string())
        .collect()
}

fn solve2(input: &str) -> String {
    let input = input.trim();

    let offset = input[..OFFSET_NUM_DIGITS]
        .parse::<usize>()
        .expect("Invalid number in input");

    if offset < input.len() / 2 {
        panic!("Algorithm doesn't work for first half of FFT");
    }

    let init_digits = digits(input);
    let num_digits = init_digits.len();
    let all_digits = init_digits
        .into_iter()
        .cycle()
        .take(NUM_REPEATS * num_digits)
        .collect::<Vec<_>>();

    let input = digits(input);

    efficient_fft(&all_digits, offset, NUM_PHASES, OUTPUT_NUM_DIGITS)
}

fn digits(value: &str) -> Vec<Digit> {
    value
        .chars()
        .map(|c| {
            let digit = c.to_digit(10).expect("Invalid digit");
            Digit::try_from(digit).expect("Failed conversion")
        })
        .collect::<Vec<_>>()
}

fn fft(input: &[Digit], num_phases: usize) -> Vec<Digit> {
    itertools::iterate(input.to_vec(), |digits| next_fft(&digits))
        .nth(num_phases)
        .expect("Infinite application of itertools::iterate ended prematurely")
}

fn next_fft(input: &[Digit]) -> Vec<Digit> {
    (0..input.len()).map(|i| calc_elem(input, i)).collect()
}

fn calc_elem(input: &[Digit], index: usize) -> Digit {
    let pattern = get_pattern(index);
    let sum: Digit = input.iter().zip(pattern).map(|(d, p)| d * p).sum();
    (sum % 10).abs()
}

fn get_pattern(index: usize) -> impl Iterator<Item = Digit> {
    BASE_PATTERN
        .iter()
        .copied()
        .cycle()
        .duplicate_values(index + 1)
        .skip(1)
}

fn efficient_fft(digits: &[Digit], offset: usize, num_phases: usize, num_digits: usize) -> String {
    let mut sums = vec![0; num_digits];
    for (binom_index, binom) in binoms_mod(num_phases - 1, 10)
        .enumerate()
        .take(digits.len() - offset)
    {
        for (sum_index, sum) in sums.iter_mut().enumerate() {
            let digit_index = offset + binom_index + sum_index;
            if let Some(digit) = digits.get(digit_index).map(|d| *d as Digit) {
                *sum = (*sum + binom * digit) % 10;
            }
        }
    }
    sums.iter().map(|d| d.to_string()).collect()
}

fn binoms(k: usize) -> impl Iterator<Item = BigInt> {
    let mut n = k;
    let mut number: BigInt = One::one();
    unfold(0, move |divisor| {
        let return_number = number.clone();
        n += 1;
        number *= n;
        *divisor += 1;
        number /= *divisor;
        Some(return_number)
    })
}

fn binoms_mod(k: usize, mod_divisor: usize) -> impl Iterator<Item = Digit> {
    binoms(k).map(move |x| mod_bigint(&x, mod_divisor))
    // let mut n = k;
    // let mut number: BigInt = One::one();
    // unfold(0, move |divisor| {
    //     let return_number = mod_bigint(&number, mod_divisor);
    //     n += 1;
    //     number *= n;
    //     *divisor += 1;
    //     number /= *divisor;
    //     Some(return_number)
    // })
}

fn mod_bigint(num: &BigInt, divisor: usize) -> Digit {
    let (_, digits) = (num % divisor).to_u32_digits();
    *digits.last().unwrap_or(&0) as Digit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_pos() {
        assert_eq!(digits("123"), vec![1, 2, 3]);
        assert_eq!(digits("7"), vec![7]);
        assert_eq!(digits("1403"), vec![1, 4, 0, 3]);
    }

    #[test]
    fn digits_zero() {
        assert_eq!(digits("0"), vec![0]);
    }

    #[test]
    fn digits_neg() {
        assert_eq!(digits("123"), vec![1, 2, 3]);
    }

    #[test]
    fn pattern_first() {
        let mut pattern = get_pattern(0);
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
    }

    #[test]
    fn pattern_second() {
        let mut pattern = get_pattern(1);
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
    }

    #[test]
    fn detailed_example() {
        let input = digits("12345678");
        let phase1 = next_fft(&input);
        assert_eq!(phase1, digits("48226158"));
        let phase2 = next_fft(&phase1);
        assert_eq!(phase2, digits("34040438"));
        let phase3 = next_fft(&phase2);
        assert_eq!(phase3, digits("03415518"));
        let phase4 = next_fft(&phase3);
        assert_eq!(phase4, digits("01029498"));
    }

    #[test]
    fn example1() {
        const NUM_PHASES: usize = 100;
        let input = digits("80871224585914546619083218645595");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("24176176");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn example2() {
        const NUM_PHASES: usize = 100;
        let input = digits("19617804207202209144916044189917");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("73745418");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn example3() {
        const NUM_PHASES: usize = 100;
        let input = digits("69317163492948606335995924319873");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("52432133");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn test_binoms() {
        let actual = binoms(2).take(10).collect::<Vec<_>>();
        let expected = to_vec_of_bigint(&[1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        if actual != expected {
            println!(
                "actual: {:?}",
                actual.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
            println!(
                "expected: {:?}",
                expected.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
        }
        assert_eq!(actual, expected);

        let actual = binoms(3).take(10).collect::<Vec<_>>();
        let expected = to_vec_of_bigint(&[1, 4, 10, 20, 35, 56, 84, 120, 165, 220]);
        if actual != expected {
            println!(
                "actual: {:?}",
                actual.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
            println!(
                "expected: {:?}",
                expected.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_binoms_mod() {
        let actual = binoms_mod(2, 10).take(10).collect::<Vec<_>>();
        // let expected = to_vec_of_bigint(&[1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        let expected = vec![1, 3, 6, 0, 5, 1, 8, 6, 5, 5];
        if actual != expected {
            println!(
                "actual: {:?}",
                actual.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
            println!(
                "expected: {:?}",
                expected.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
        }
        assert_eq!(actual, expected);

        let actual = binoms_mod(3, 10).take(10).collect::<Vec<_>>();
        // let expected = to_vec_of_bigint(&[1, 4, 10, 20, 35, 56, 84, 120, 165, 220]);
        let expected = vec![1, 4, 0, 0, 5, 6, 4, 0, 5, 0];
        if actual != expected {
            println!(
                "actual: {:?}",
                actual.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
            println!(
                "expected: {:?}",
                expected.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            );
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve2("03036732577212944063491565474664"), "84462026");
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve2("02935109699940807407585447034323"), "78725270");
    }

    #[test]
    fn part2_example3() {
        assert_eq!(solve2("03081770884921959731165446850517"), "53553731");
    }

    fn to_vec_of_bigint(slice: &[usize]) -> Vec<BigInt> {
        slice.iter().map(|x| BigInt::from(*x)).collect::<Vec<_>>()
    }
}
