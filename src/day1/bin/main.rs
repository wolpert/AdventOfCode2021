use std::io;

#[derive(Default, Debug)]
pub struct Depth {
    last_value: i64,
    increasing_count: i64,
    done: bool,
    init: bool,
}

impl Depth {
    pub fn new() -> Self {
        Depth {
            last_value: 0,
            increasing_count: 0,
            done: false,
            init: false,
        }
    }
    pub fn process(&mut self) {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).expect("error");
        if bytes_read > 0 {
            let value: i64 = line.trim().parse().unwrap();
            if !self.init {
                self.init = true;
            } else if value > self.last_value {
                self.increasing_count += 1;
            }
            self.last_value = value;
        } else {
            self.done = true;
        }
    }
}

fn main() {
    let mut depth = Depth::new();

    while !depth.done {
        depth.process();
    }
    println!("Increased {:?}", depth);
}
