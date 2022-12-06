fn main() {
    let input: Vec<char> = Vec::from_iter(
        include_str!("day6_input.txt")
        .trim().chars());

    let mut buf_4 = CircularBuffer::<char>::new(4,'\0');
    let mut buf_14 = CircularBuffer::<char>::new(14,'\0');
    
    for i in 0_usize..3 {
        buf_4.push(input[i]);
    }
    for i in 3..input.len() {
        buf_4.push(input[i]);
        let cur_str = buf_4.into_str();
        if cur_str.chars().all(|c| cur_str.matches(c).count() <= 1) {
            println!("Found packet marker; size 4: {}", i + 1);
            break;
        }
    }

    for i in 0_usize..13 {
        buf_14.push(input[i]);
    }
    for i in 13..input.len() {
        buf_14.push(input[i]);
        let cur_str = buf_14.into_str();
        if cur_str.chars().all(|c| cur_str.matches(c).count() <= 1) {
            println!("Found packet marker; size 14: {}", i + 1);
            break;
        }
    }
}

struct CircularBuffer<T> where T: Clone {
    inner: Vec<T>,
    pos: usize,
    size: usize
}
impl<T> CircularBuffer<T> where T: Clone {
    fn new(size: usize, default: T) -> Self {
        CircularBuffer::<T> {
            inner: vec![default; size],
            pos: 0,
            size
        }
    }
    fn at(&self, i: usize) -> Option<&T> {
        if i < self.size {
            Some(&self.inner[i])
        } else {
            None
        }
    }
    fn push(&mut self, n: T) {
        self.inner[self.pos] = n;
        self.pos += 1;
        self.pos %= self.size
    }
}
impl CircularBuffer<char> {
    fn into_str(&self) -> String {
        let mut s = String::new();
        for i in 0..self.size {
            s.push(self.inner[(self.pos + i) % self.size]);
        }
        s
    }
}