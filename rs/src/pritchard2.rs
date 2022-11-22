use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    ///primes below
    n: usize,
}

struct Wheel {
    n: usize,
    w: Vec<usize>,
    d: Vec<bool>,
    w_end: usize,
    w_end_max: usize,
    length: usize,
    imaxf: usize,
}

impl Wheel {
    pub fn new(n: usize) -> Self {
        let mut wheel = Wheel {
            n: n,
            w: vec![0; n / 4 + 5],
            d: vec![false; n + 1],
            w_end: 0,
            w_end_max: 0,
            length: 2,
            imaxf: 0,
        };
        wheel.w[0] = 1;
        wheel
    }

    pub fn extend(self: &mut Self, n: usize) {
        let mut i = 0;
        let mut j = self.w_end;
        let mut x = self.length + 1;
        while x <= n {
            j += 1;
            self.w[j] = x;
            self.d[x] = false;
            i += 1;
            x = self.length + self.w[i];
        }
        self.length = n;
        self.w_end = j;
        if self.w_end > self.w_end_max {
            self.w_end_max = self.w_end;
        }
    }

    pub fn delete(self: &mut Self, p: usize) {
        let mut i = 0;
        let mut x = p;
        while x <= self.length {
            self.d[x] = true;
            i += 1;
            x = p * self.w[i];
        }
        self.imaxf = i - 1;
    }

    pub fn compress(self: &mut Self) {
        let to = if self.length < self.n {
            self.w_end
        } else {
            self.imaxf
        };
        let mut j = 0;
        for i in 1..to + 1 {
            if !self.d[self.w[i]] {
                j += 1;
                self.w[j] = self.w[i];
            }
        }
        if to == self.w_end {
            self.w_end = j;
        } else {
            for k in j + 1..to + 1 {
                self.w[k] = 0;
            }
        }
    }
}

pub fn sieve(n: usize) -> impl Iterator<Item = usize> {
    let mut wheel = Wheel::new(n);
    let mut primes = vec![2];
    let mut p = 3;
    while p * p <= n {
        primes.push(p);
        if wheel.length < n {
            wheel.extend(usize::min(p * wheel.length, n));
        }
        wheel.delete(p);
        wheel.compress();
        p = wheel.w[1];
        if p == 0 {
            break;
        }
    }
    if wheel.length < n {
        wheel.extend(n);
    }

    for i in 1..wheel.w_end + 1 {
        if wheel.w[i] != 0 && !wheel.d[wheel.w[i]] {
            primes.push(wheel.w[i]);
        }
    }

    primes.into_iter()
}

fn main() {
    let args = Args::parse();

    for p in sieve(args.n) {
        println!("{}", p);
    }
}
