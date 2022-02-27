const INPUT: &'static str = include_str!("../inputs/3.txt");

pub(crate) fn run() {
    let output1 = parse1(INPUT);
    println!("Day 3: output1: {}", output1);
}

fn parse1(input: &str) -> usize {
    let mut len = None;
    let numbers: Vec<_> = input
        .lines()
        .map(|line| {
            if len.is_none() {
                len = Some(line.len());
            }
            usize::from_str_radix(line, 2).unwrap()
        })
        .collect();

    let mut counters = vec![0; len.unwrap()];

    let line_count = numbers.len();
    for mut num in numbers {
        let mut idx = 0;
        dbg!(num);
        while num != 0 {
            if (num & 1) == 1 {
                counters[idx] += 1
            }
            dbg!(format!("{:05b}", num));
            dbg!(num & 1);
            idx += 1;
            num >>= 1;
        }
    }

    let gamma = counters
        .iter()
        .map(|count| if *count >= (line_count / 2) { 1 } else { 0 })
        .rfold(0, |acc, x| (acc << 1) | x);
    
    let epsilon = counters
        .iter()
        .map(|count| if *count <= (line_count / 2) { 1 } else { 0 })
        .rfold(0, |acc, x| (acc << 1) | x);


    dbg!(gamma* epsilon);
    gamma* epsilon
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 198)
    }

    // #[test]
    // fn second() {
    //     assert_eq!(parse2(INPUT), 900)
    // }
}
