use advent_of_code_2022::read_input;
use std::collections::HashMap;
use itermore::prelude::*;

const INPUT: &str = "src/day_03/input";

pub fn run() {
    part1();
    part2();
}



fn part1() {

    let input = read_input(INPUT);
    let mut count = 0;

    let letter_scores = ('a'..='z')
    .chain('A'..='Z')
    .enumerate()
    .map(|(idx, c)| (c, idx + 1))
    .collect::<HashMap<char, usize>>();


    for line in input.lines(){
        let chars =line.chars().count();
        let (compartment_1, compartment_2) = line.split_at(chars/2);
        println!("1: {}, 2: {}", compartment_1, compartment_2);

        let common_char = compartment_1
                .chars()
                .find(|c| compartment_2.contains(*c))
                .unwrap();

       count += letter_scores.get(&common_char).unwrap();

        // for char in compartment_1.chars(){
        //     if compartment_2.chars().into_iter().any(|v| v==char){
        //         println!("esto: {}", char)
        //     }
        // }
    }

    println!("Part1: {}", count);
}



fn part2(){
    let input = read_input(INPUT);

    let letter_scores = ('a'..='z')
    .chain('A'..='Z')
    .enumerate()
    .map(|(idx, c)| (c, idx + 1))
    .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| {
                    b.contains(*a_char)
                        && c.contains(*a_char)
                })
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    println!("Part2: {}", result);
}