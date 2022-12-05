fn main() {
    let mut input = include_str!("day5_input.txt").trim().lines();
    let input = input.by_ref();
    let mut beginning_state: Vec<&str> = input.take_while(|x| !x.trim().is_empty()).collect();
    beginning_state.reverse();
    let rest: Vec<&str> = input.collect();
    let mut crates_1 = construct_stacks(beginning_state.into_iter());
    let mut crates_2 = crates_1.clone();
    
    for line in rest {
        parse_instruct(line, &mut crates_1, false);
        parse_instruct(line, &mut crates_2, true);
    }

    let mut output_1 = String::new();
    for stack in crates_1 {
        output_1.push(*stack.last().unwrap())
    }
    let mut output_2 = String::new();
    for stack in crates_2 {
        output_2.push(*stack.last().unwrap())
    }
    
    println!("Part 1 {}", output_1);
    println!("Part 2 {}", output_2);
}

fn parse_instruct(instruction: &str, vec: &mut Vec<Vec<char>>, is_upgraded: bool) {
    let mut nums = instruction.split(" ").map(str::parse::<usize>).filter(|x| x.is_ok()).map(Result::unwrap);
    let move_amount = nums.next().unwrap_or(0);
    let move_src = nums.next().unwrap() - 1;
    let move_dest = nums.next().unwrap() - 1;
    if !is_upgraded {
        for _ in 0..move_amount {
            let c = vec[move_src].pop().unwrap();
            vec[move_dest].push(c)
        }
    } else {
        let mut n = vec![];
        for _ in 0..move_amount {
            n.push(vec[move_src].pop().unwrap());
        }
        for _ in 0..move_amount {
            vec[move_dest].push(n.pop().unwrap());
        }
    }
}

fn construct_stacks<'a>(stuff: impl Iterator<Item = &'a str>) -> Vec<Vec<char>> {
    let mut iter = stuff.into_iter();
    let number_of_stacks = iter.next().unwrap().split_ascii_whitespace().count();
    let mut out_vec: Vec::<Vec<char>> = vec![vec![]; number_of_stacks];
    while let Some(line) = iter.next() {
        const OFFSET: usize = 4;
        for i in 0..number_of_stacks {
            let crate_char = line.chars().skip((i*OFFSET)+1).next().unwrap();
            if !crate_char.is_whitespace() {
                out_vec[i].push(crate_char)
            }
        }
    }
    out_vec
}