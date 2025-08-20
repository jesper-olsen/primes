use ::std::time::Instant;
use clap::{Parser, ValueEnum};

#[allow(dead_code)]
mod eratosthenes;
#[allow(dead_code)]
mod pritchard2;
#[allow(dead_code)]
mod pritchard2bv;
#[allow(dead_code)]
mod sorenson;

/// Available prime sieving algorithms
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Algorithm {
    Eratosthenes,
    EratosthenesBV,
    Pritchard2,
    Pritchard2BV,
    Sorenson,
}

impl std::str::FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eratosthenes" => Ok(Algorithm::Eratosthenes),
            "eratosthenesbv" => Ok(Algorithm::EratosthenesBV),
            "pritchard2" => Ok(Algorithm::Pritchard2),
            "pritchard2bv" => Ok(Algorithm::Pritchard2BV),
            "sorenson" => Ok(Algorithm::Sorenson),
            _ => Err(format!("Unknown algorithm: {s}")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    ///primes below
    n: usize,

    /// Algorithm to use
    #[arg(short, long, default_value = "eratosthenes")]
    algorithm: Algorithm,

    #[arg(short, long, default_value_t = false)]
    ///time it
    t: bool,
}

/// Several different prime sieves implemented here
fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let iter: Box<dyn Iterator<Item = usize>> = match args.algorithm {
        Algorithm::Eratosthenes => Box::new(eratosthenes::sieve(args.n)),
        Algorithm::EratosthenesBV => Box::new(eratosthenes::sieve_bv(args.n)),
        Algorithm::Pritchard2 => Box::new(pritchard2::sieve(args.n)),
        Algorithm::Pritchard2BV => Box::new(pritchard2bv::sieve(args.n)),
        Algorithm::Sorenson => Box::new(sorenson::sieve(args.n).into_iter()),
    };

    for p in iter {
        println!("{p}");
    }
    eprintln!("That took {:?} ", Instant::now() - start);
}
