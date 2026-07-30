use std::time::Instant;
use clap::{Parser, ValueEnum};
use std::io::{self, BufWriter, Write};

mod eratosthenes;
mod pritchard2;
mod pritchard2bv;
mod sorenson;

/// Available prime sieving algorithms
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Algorithm {
    Eratosthenes,
    EratosthenesBV,
    EratosthenesSegmented,
    Pritchard2,
    Pritchard2BV,
    Sorenson,
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

    #[arg(short, long, default_value_t = true)]
    ///time it
    time: bool,
}

/// Several different prime sieves implemented here
fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    macro_rules! run_sieve {
        ($iter:expr) => {
            for p in $iter {
                writeln!(handle, "{}", p).unwrap();
            }
        };
    }

    match args.algorithm {
        Algorithm::Eratosthenes => run_sieve!(eratosthenes::sieve(args.n)),
        Algorithm::EratosthenesBV => run_sieve!(eratosthenes::sieve_bv(args.n)),
        Algorithm::EratosthenesSegmented => run_sieve!(eratosthenes::sieve_segmented(args.n)),
        Algorithm::Pritchard2 => run_sieve!(pritchard2::sieve(args.n)),
        Algorithm::Pritchard2BV => run_sieve!(pritchard2bv::sieve(args.n)),
        Algorithm::Sorenson => run_sieve!(sorenson::sieve(args.n)),
    };

    if args.time {
        eprintln!("That took {:?} ", Instant::now() - start);
    }
}
