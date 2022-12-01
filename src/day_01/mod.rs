use advent_of_code_2022::read_input;

const INPUT: &str = "src/day_01/input";

pub fn run() {
    part2();
}

fn part1() -> Vec<i32> {

    let input = read_input(INPUT);

    let mut elf = Vec::new();
    let mut cal_count = 0; 

    for line in input.lines(){
        if line.is_empty(){
            elf.push(cal_count);
            cal_count = 0;
            continue;
        }
        cal_count = cal_count + line.parse::<i32>().unwrap();

    }

    println!("Part1: {}", elf.iter().max().unwrap());
    return elf
}


fn part2(){
    let mut elf = part1();
    elf.sort();
    elf.reverse();
    println!("Part2: {}",elf[0]+elf[1]+elf[2]);
}