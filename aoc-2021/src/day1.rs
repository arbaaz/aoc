const INPUT: &'static str = include_str!("../inputs/1.txt");

pub(crate) fn run() {
    let output1 = parse1(INPUT);
    println!("Day 1: output1: {}", output1);
    
    let output2 = parse2(INPUT);
    println!("Day 1: output2: {}", output2);

}

fn parse1(input: &str) -> usize {
    let lines: Vec<u16> = input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect();

    let count = lines.array_windows().filter(|[n1, n2]| (n2 > n1)).count();
    count
}

fn parse2(input: &str) -> usize {
    let lines: Vec<u16> = input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect();

    let y: Vec<u16> = lines.array_windows().map(|[n1, n2, n3]| (n1+n2+n3)).collect();
    let count = y.array_windows().filter(|[n1, n2]| (n2 > n1)).count();
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";
    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 7)
    }

    #[test]
    fn second(){
      assert_eq!(parse2(INPUT), 5)
    }
}
