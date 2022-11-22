use clap::{Parser};
use::std::time::{Duration,Instant};

mod sorenson;
mod eratosthenes;
mod pritchard2;
mod pritchard2bv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    ///primes below
    n: usize,
    #[arg(short, long, default_value_t = false)]
    ///time it
    t: bool,
    #[arg(short, long, default_value_t = false)]
    ///print primes
    p: bool,
}

fn main() {
    let args = Args::parse();
    let start=Instant::now();
    //let duration=Duration::new(5,0);

    //for p in eratosthenes_bv(args.n) {
    //for p in sorenson::rolling().take_while(|&p|p<args.n) {
    //for p in eratosthenes::sieve(args.n) {
    for p in pritchard2::sieve(args.n) {
        if args.p {
            println!("{}", p);
        }   
    }
    if args.t {
        println!("That took {:?} ", Instant::now()-start);
    }
}
