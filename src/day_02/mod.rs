use advent_of_code_2022::read_input;
use core::str::FromStr;
use std::cmp::Ordering;


const INPUT: &str = "src/day_02/input";

pub fn run() {
    part1();
    part2();
}

#[derive(PartialEq, Copy, Clone)]
enum Move{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{

        match s {
            "A"|"X"=>Ok(Move::Rock),
            "B"|"Y"=>Ok(Move::Paper),
            "C"|"Z"=>Ok(Move::Scissors),
            _ => Err("Move uknown".to_string())
        }
    }   
}

impl PartialOrd for Move {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock
            && other == &Move::Scissors
        {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}


fn part1() {

    let input = read_input(INPUT);
    let mut count = 0;

    for line in input.lines(){

        let play: Vec<Move> = line.split_whitespace().map(|s| s.parse::<Move>().unwrap()).collect();

        let part = match play[0].partial_cmp(&play[1]) {
            Some(Ordering::Equal) => {
                3 + play[1] as i32
            }
            Some(Ordering::Less) => 6 + play[1] as i32,
            Some(Ordering::Greater) => {
                0 + play[1] as i32
            }
            None => {
                panic!("moves should be comparable")
            }
        };

        count +=part
    }

    println!("Part1: {}", count);
}


fn part2(){

    let input = read_input(INPUT);
    let mut count = 0;

    for line in input.lines(){

        let play: Vec<Move> = line.split_whitespace().map(|s| s.parse::<Move>().unwrap()).collect();

        let part = match line.split_whitespace().last().unwrap() {
            "X" => {
                let our_move = match play[0] {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                };
                0 + our_move as i32
            }
            "Y" => 3 + play[0] as i32,
            "Z" => {
                let our_move = match play[0] {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                };
                6 + our_move as i32
            }
            _ => {
                panic!("Unexpected response");
            }
        };

        count +=part
   
    }
    println!("Part2: {}", count);
}