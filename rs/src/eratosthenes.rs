//! [Eratosthenes prime sieve](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
//! - calculate all primes below some integer N.

/// Calc all primes below n
/// ```
/// for p in sieve(10) {
///     println!("{p}");
/// }
/// ```
pub fn sieve(n: usize) -> impl Iterator<Item = usize> {
    let mut is_prime = vec![true; n];
    let sqrt_n = (n as f64).sqrt() as usize + 1;

    for i in 2..sqrt_n {
        if is_prime[i] {
            for y in (i * i..n).step_by(i) {
                is_prime[y] = false;
            }
        }
    }

    (2..n).filter(move |&i| is_prime[i])
}

// -- sieve_bv --------------------------------------------------------------

/// same as sieve(), but more memory efficient  
/// - uses bitvectors for storing the 'array of bool'
pub fn sieve_bv(n: usize) -> impl Iterator<Item = usize> {
    let nbits = usize::BITS as usize;
    let mut is_prime: Vec<usize> = vec![usize::MAX; n.div_ceil(nbits)];
    // set_bit k:
    // l[k/usize::BITS] |= 1<<k%usize::BITS

    // clear_bit k:
    // l[k/usize::BITS] &= !(1<<k%usize::BITS)

    // is_set k
    // l[k/usize::BITS] & (1<<k%usize::BITS) != 0

    for i in 2..n.isqrt() + 1 {
        if is_prime[i / nbits] & 1 << i % nbits != 0 {
            for y in (i * i..n).step_by(i) {
                is_prime[y / nbits] &= !(1 << y % nbits); // set false
            }
        }
    }

    (2..n).filter(move |i| is_prime[i / nbits] & 1 << i % nbits != 0)
}

// -- sieve_segmented -------------------------------------------------------

/// Same as sieve(), but segmented to optimize for CPU L1 cache.
/// Space complexity is bounded to O(sqrt(N)) for the base primes
/// and a fixed ~32KB block for the sieve window.
pub fn sieve_segmented(n: usize) -> impl Iterator<Item = usize> {
    SegSieveIter::new(n)
}

struct SegSieveIter {
    n: usize,
    delta: usize,
    base_primes: Vec<usize>,
    is_prime: Vec<bool>,
    low: usize,
    high: usize,
    current_idx: usize,
}

impl SegSieveIter {
    fn new(n: usize) -> Self {
        // Handle edge cases where n <= 2 (no primes strictly below 2)
        if n < 3 {
            return Self {
                n,
                delta: 32768,
                base_primes: vec![],
                is_prime: vec![],
                low: n,
                high: n,
                current_idx: 0,
            };
        }

        // Lock delta to ~32KB to fit comfortably inside the L1 data cache.
        // Tying this to sqrt(n) causes cache thrashing for very large n.
        let delta = 32768;

        let sqrt_n = n.isqrt();
        // We need base primes up to and including sqrt(n)
        let base_primes: Vec<usize> = sieve(sqrt_n + 1).collect();

        let mut iter = Self {
            n,
            delta,
            base_primes,
            is_prime: Vec::with_capacity(delta),
            low: 2,
            high: 1, // triggers immediate calculation of the first segment
            current_idx: 0,
        };

        iter.sieve_next_segment();
        iter
    }

    fn sieve_next_segment(&mut self) {
        self.low = self.high + 1;

        // n is exclusive, so if low >= n we are completely done
        if self.low >= self.n {
            return;
        }

        self.high = std::cmp::min(self.low + self.delta - 1, self.n - 1);
        let len = self.high - self.low + 1;

        self.is_prime.clear();
        self.is_prime.resize(len, true);

        for &p in &self.base_primes {
            // Calculate the first multiple of p that is >= self.low
            let mut first_multiple = (self.low + p - 1) / p * p;

            // Skip multiples that were already handled by smaller base primes
            if first_multiple < p * p {
                first_multiple = p * p;
            }

            if first_multiple <= self.high {
                for y in (first_multiple..=self.high).step_by(p) {
                    self.is_prime[y - self.low] = false;
                }
            }
        }
        self.current_idx = 0;
    }
}

impl Iterator for SegSieveIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.low >= self.n {
                return None;
            }

            while self.current_idx < self.is_prime.len() {
                let is_p = self.is_prime[self.current_idx];
                let p = self.low + self.current_idx;
                self.current_idx += 1;

                if is_p {
                    return Some(p);
                }
            }

            // Reached the end of the current segment, compute the next
            self.sieve_next_segment();
        }
    }
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
#[test]
fn test_eratosthenes_segmented() {
    let l: Vec<usize> = sieve_segmented(10).collect();
    assert_eq!(l, vec![2, 3, 5, 7]);
}
