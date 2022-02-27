

const INPUT: &'static str = include_str!("../inputs/2.txt");

pub(crate) fn run() {
    let output1 = parse1(INPUT);
    println!("Day 2: output1: {}", output1);

    let output2 = parse2(INPUT);
    println!("Day 2: output2: {}", output2);
}

fn parse1(input: &str) -> usize {
    let mut depth  = 0usize;
    let mut hor_pos= 0usize;

    for line in input.lines(){
        let cmd:Command = line.parse().unwrap();
        match cmd {
            Command::Forward(num)  => hor_pos += num,
            Command::Down(num) => depth += num,
            Command::Up(num) => depth -= num,
        }
    }

    depth * hor_pos
}


fn parse2(input: &str) -> usize {
    let mut depth  = 0usize;
    let mut hor_pos= 0usize;
    let mut aim = 0usize;

    for line in input.lines(){
        let cmd:Command = line.parse().unwrap();
        match cmd {
            Command::Forward(num)  => {
                hor_pos += num;
                depth += aim * num
            },
            Command::Down(num) => aim += num,
            Command::Up(num) => aim -= num,
        }
    }

    depth * hor_pos
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl std::str::FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s:  &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap();
        let pos = tail.parse().unwrap();

        match head {
            "forward" => Ok(Command::Forward(pos)),
            "down" => Ok(Command::Down(pos)),
            "up" => Ok(Command::Up(pos)),
            _ => panic!("unreachable")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 150)
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 900)
    }
   
}
