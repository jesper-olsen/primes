use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=100)]
    n: usize,
}

//Sieve of Eratosthenes - calc all primes below n
//fn prime_sieve(n:usize) -> Vec<usize> {
fn prime_sieve(n:usize) -> impl Iterator<Item=usize> {
    let mut l=vec![true;n];
    l[0]=false;
    l[1]=false;
    let q=f64::sqrt(l.len() as f64) as usize + 1;
    //let q=l.len();
    for i in 0..q {
        if l[i] {
            for y in (i*i..n).step_by(i) {
                l[y]=false;
            }
        }
    }

    l.into_iter()
    .enumerate()
    .filter(|(_a,b)| *b)
    .map(|(a,_b)|a)
    //.collect::<Vec<usize>>()
}

fn main() {
    let args=Args::parse();

    //let n = usize::pow(10,8);
    for p in prime_sieve(args.n) {  
        println!("{}", p);
    }   
}
