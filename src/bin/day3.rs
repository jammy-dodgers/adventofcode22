fn main() {
    let mut input = include_str!("day3_input.txt").trim().lines();
    let mut running_total = 0;
    let mut badge_total = 0;

    while let (Some(elf1), Some(elf2), Some(elf3)) = (input.next(), input.next(), input.next()) {
        for line in [elf1, elf2, elf3] {
            let mid = line.len() / 2;
            let left = &line[..mid];
            let right = &line[mid..];
            let mut matching = Vec::from_iter(left.chars().filter(|x| right.contains(*x)));
            matching.sort();
            matching.dedup();
            running_total += matching.iter().map(|c| to_value(*c)).sum::<i32>();
        }

        badge_total += to_value(
            *Vec::from_iter(
                elf1.chars()
                    .filter(|x| elf2.contains(*x) && elf3.contains(*x)),
            )
            .first()
            .unwrap(),
        );
    }

    println!("Part 1: {}", running_total);
    println!("Part 2: {}", badge_total);
}

fn to_value(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 'A' as i32 + 27
    }
    // A-Z 27-52
    else {
        c as i32 - 'a' as i32 + 1
    } // a-z 1-26
}
