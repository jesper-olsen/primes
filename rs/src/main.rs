use clap::Parser;
//use::std::time::{Duration,Instant};
use ::std::time::Instant;

#[allow(dead_code)]
mod eratosthenes;
#[allow(dead_code)]
mod pritchard2;
#[allow(dead_code)]
mod pritchard2bv;
#[allow(dead_code)]
mod sorenson;

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

/// Several different prime sieves implemented here
fn main() {
    let args = Args::parse();
    let start = Instant::now();
    //let duration=Duration::new(5,0);

    //for p in eratosthenes_bv(args.n) {
    //for p in eratosthenes::sieve(args.n) {
    //for p in pritchard2::sieve(args.n) {
    //for p in pritchard2bv::sieve(args.n) {
    for p in sorenson::sieve(args.n) {
        println!("{}", p);
    }
    eprintln!("That took {:?} ", Instant::now() - start);
}
