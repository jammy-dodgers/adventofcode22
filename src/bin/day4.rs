fn main() {
    let input = include_str!("day4_input.txt").trim();
    let mut fully_overlap = 0;
    let mut partial_overlap = 0;

    for line in input.lines() {
        let mut assgs = 
        line.split(',').map(|assg| 
                assg.split('-')
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)); // advent of code teaching bad habits with all these unwraps :P
        let (mut assg1, mut assg2) = (assgs.next().unwrap(), assgs.next().unwrap());
        
        let (assg1l, assg1r) = (assg1.next().unwrap(), assg1.next().unwrap());
        let (assg2l, assg2r) = (assg2.next().unwrap(), assg2.next().unwrap());

        if (assg1l <= assg2l && assg1r >= assg2r) ||
            (assg2l <= assg1l && assg2r >= assg1r) {
            fully_overlap += 1;
        }
        if (assg1r >= assg2l && assg2l >= assg1l) || 
            (assg2r >= assg1l && assg1l >= assg2l) {
            partial_overlap += 1;
        }
    }
    println!("Full overlap {}", fully_overlap);
    println!("Any overlap {}", partial_overlap);
}