use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    ///primes below
    n: usize,
}

//Sieve of Eratosthenes - calc all primes below n
pub fn sieve(n: usize) -> impl Iterator<Item = usize> {
    let mut l = vec![true; n];
    let q = f64::sqrt(l.len() as f64) as usize + 1;

    for i in 2..q {
        if l[i] {
            for y in (i * i..n).step_by(i) {
                l[y] = false;
            }
        }
    }

    (2..n).filter(move |&i| l[i])
}

// same as sieve(), but with BitVec
pub fn sieve_bv(n: usize) -> impl Iterator<Item = usize> {
    let nbits = usize::BITS as usize;
    let mut l: Vec<usize> = vec![usize::MAX; n / nbits + 1];
    // set_bit k:
    // l[k/usize::BITS] |= 1<<k%usize::BITS

    // clear_bit k:
    // l[k/usize::BITS] &= !(1<<k%usize::BITS)

    // is_set k
    // l[k/usize::BITS] & (1<<k%usize::BITS) != 0

    let q = f64::sqrt(n as f64) as usize + 1;
    for i in 2..q {
        if l[i / nbits] & 1 << i % nbits != 0 {
            for y in (i * i..n).step_by(i) {
                l[y / nbits] &= !(1 << y % nbits);
            }
        }
    }

    (2..n).filter(move |i| l[i / nbits] & 1 << i % nbits != 0)
}

fn main() {
    let args = Args::parse();

    for p in sieve_bv(args.n) {
    //for p in sieve(args.n) {
        println!("{}", p);
    }
}
