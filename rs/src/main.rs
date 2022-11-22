use clap::{Parser};
use::std::time::{Duration,Instant};

#[allow(dead_code)]
mod sorenson;
#[allow(dead_code)]
mod eratosthenes;
#[allow(dead_code)]
mod pritchard2;
#[allow(dead_code)]
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
}

fn main() {
    let args = Args::parse();
    let start=Instant::now();
    //let duration=Duration::new(5,0);

    //for p in eratosthenes_bv(args.n) {
    //for p in eratosthenes::sieve(args.n) {
    //for p in pritchard2::sieve(args.n) {
    for p in sorenson::sieve().take_while(|&p|p<args.n) {
        println!("{}", p);
    }
    if args.t {
        println!("That took {:?} ", Instant::now()-start);
    }
}
