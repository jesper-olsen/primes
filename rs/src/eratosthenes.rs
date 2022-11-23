//! Eratosthenes - calculate all primes below some integer N
//! See <https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes>

/// Calc all primes below n
/// ```
/// for p in sieve(10) {
///     println!(p);
/// }
/// ```
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

/// same as sieve(), but more memory efficient  
/// - uses bitvectors for storing the 'array of bool'
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
                l[y / nbits] &= !(1 << y % nbits); // set false
            }
        }
    }

    (2..n).filter(move |i| l[i / nbits] & 1 << i % nbits != 0)
}

#[test]
fn test_eratosthenes() {
    let l: Vec<usize> = sieve(10).collect();
    assert_eq!(l, vec![2, 3, 5, 7]);
}
#[test]
fn test_eratosthenes_bv() {
    let l: Vec<usize> = sieve_bv(10).collect();
    assert_eq!(l, vec![2, 3, 5, 7]);
}
