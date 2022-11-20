use clap::Parser;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=100)]
    ///primes below
    n: usize,
}

// Sorenson's rolling sieve - yield next prime each time it is called.
// Complexity: O(nloglogn) time, O(sqrt(n)log(n)) bits,
//             O(logn/loglogn) incremental
// See algorithm description in paper:
// "Two Compact Incremental Prime Sieves", Jonathan P. Sorenson, Journal of Computation and Mathematics, 2015
// https://arxiv.org/abs/1503.02592

struct RollingSorenson {
    n: usize,
    r: usize,
    s: usize,
    delta: usize,
    pos: usize,
    t: Vec<Vec<usize>>,
    primes: VecDeque<usize>,
}

impl RollingSorenson {
    pub fn new()->Self {
        let start=100;
        let r=f64::sqrt(start as f64) as usize + 1;
        let mut rs=RollingSorenson {
            n: start,
            r: r,
            delta: r+2,
            s: r*r,
            t: Vec::new(), // vec of stacks
            pos: 0,
            primes: VecDeque::from([ 2,  3,  5,  7, 11, 13, 17, 19, 23, 29,
                                    31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
                                    73, 79, 83, 89, 97]),
        };

        for _ in 0..rs.delta+1 {
            rs.t.push(Vec::new());
        }

        for p in &rs.primes {
            if *p>r-1 {
                break;
            }
            let j=(p-(start%p))%p;
            rs.t[j].push(*p);
        }
        rs
    }

    fn next(self: &mut Self) -> usize {
        if !self.primes.is_empty() {
            return self.primes.pop_front().unwrap();
        }   

        loop {
            let mut is_prime=true;
            while !self.t[self.pos].is_empty() {
                let p=self.t[self.pos].pop().unwrap();
                self.t[(self.pos+p)%self.delta].push(p);
                is_prime=false;
            }
            if self.n==self.s {
                if is_prime {
                    self.t[(self.pos+self.r)%self.delta].push(self.r);
                    is_prime=false;
                }   
                self.r+=1;
                self.s=self.r*self.r;
            }   
            self.n+=1;
            self.pos=(self.pos+1)%self.delta;
            if self.pos==0 {
                self.delta+=2;
                self.t.push(Vec::new());
                self.t.push(Vec::new());
            }
            if is_prime {
                return self.n-1;
            }   
        }   
    }
}

fn main() {
    let args=Args::parse();

    let mut rs=RollingSorenson::new();
    loop {
        let p=rs.next();
        if p>=args.n {
            break;
        }
        println!("{}",p);
    }   
}
