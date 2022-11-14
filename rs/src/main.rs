//Sieve of Eratosthenes - calc all primes below n
fn prime_sieve(n:usize) -> Vec<usize> {
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

    l.iter()
    .enumerate()
    .filter(|(_a,b)| **b)
    .map(|(a,_b)|a)
    .collect::<Vec<usize>>()
}

fn main() {
    let n = usize::pow(10,9);
    let l=prime_sieve(n);
    println!("n: {}, #primes: {}",n,l.len());
    println!("{:?}",l[0..5].to_vec());
}
