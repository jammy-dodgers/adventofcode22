struct Elf {
    calories: i32
}

fn main() {
    let input = include_str!("day1_input.txt").trim();

    let mut elves = Vec::<Elf>::new();
    let mut current_calorie_count = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            elves.push(Elf { calories: current_calorie_count });
            current_calorie_count = 0;
        } else {
            current_calorie_count += line.parse::<i32>().unwrap_or(0)
        }
    }
    elves.sort_by_key(|e| e.calories);
    elves.reverse();
    let top_three_total: i32 = elves.iter().take(3).map(|e| e.calories).sum();

    println!("Most calories held are {}", 
        elves.first().map_or(0, |e| e.calories));
    println!("Top three total calories held are {}", 
        top_three_total);
}