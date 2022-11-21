use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    ///primes below
    n: usize,
}

//Sieve of Eratosthenes - calc all primes below n
//fn prime_sieve(n:usize) -> Vec<usize> {
fn prime_sieve(n: usize) -> impl Iterator<Item = usize> {
    let mut l = vec![true; n];
    l[0] = false;
    l[1] = false;
    let q = f64::sqrt(l.len() as f64) as usize + 1;
    for i in 0..q {
        if l[i] {
            for y in (i * i..n).step_by(i) {
                l[y] = false;
            }
        }
    }

    l.into_iter()
        .enumerate()
        .filter(|(_a, b)| *b)
        .map(|(a, _b)| a)
    //.collect::<Vec<usize>>()
}

fn prime_sieve_bv(n: usize) -> impl Iterator<Item = usize> {
    let nbits = usize::BITS as usize;
    let mut l: Vec<usize> = vec![usize::MAX; n / nbits + 1];
    // set_bit k:
    // l[k/usize::BITS] |= 1<<k%usize::BITS

    // clear_bit k:
    // l[k/usize::BITS] &= !(1<<k%usize::BITS)

    // is_set k
    // l[k/usize::BITS] & (1<<k%usize::BITS) != 0

    l[0 / nbits] &= !(1 << 0 % nbits);
    l[1 / nbits] &= !(1 << 1 % nbits);

    let q = f64::sqrt(n as f64) as usize + 1;
    for i in 0..q {
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

    for p in prime_sieve_bv(args.n) {
        println!("{}", p);
    }
}
