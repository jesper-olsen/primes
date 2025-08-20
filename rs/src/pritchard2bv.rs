//! same as pritchard2.rs, but more memory efficient (bit vectors instead of 'array of bool').
//! See <https://en.wikipedia.org/wiki/Sieve_of_Pritchard>

struct Wheel {
    n: usize,
    w: Vec<usize>,
    //d: Vec<bool>,
    d: Vec<usize>,
    end: usize,
    end_max: usize,
    length: usize,
    max_index: usize,
}

impl Wheel {
    pub fn new(n: usize) -> Self {
        let mut wheel = Wheel {
            n,
            w: vec![0; n / 4 + 5],
            //d: vec![false;n+1],
            d: vec![usize::MAX; n / usize::BITS as usize + 1],
            end: 0,
            end_max: 0,
            length: 2,
            max_index: 0,
        };
        wheel.w[0] = 1;
        wheel
    }

    pub fn extend(&mut self, n: usize) {
        let mut i = 0;
        let mut j = self.end;
        let mut x = self.length + 1;
        while x <= n {
            j += 1;
            self.w[j] = x;
            //self.d[x]=false;
            self.d[x / usize::BITS as usize] &= !(1 << x % usize::BITS as usize);
            i += 1;
            x = self.length + self.w[i];
        }
        self.length = n;
        self.end = j;
        if self.end > self.end_max {
            self.end_max = self.end;
        }
    }

    pub fn delete(&mut self, p: usize) {
        let mut i = 0;
        let mut x = p;
        while x <= self.length {
            //self.d[x]=true;
            self.d[x / usize::BITS as usize] |= 1 << x % usize::BITS as usize;
            i += 1;
            x = p * self.w[i];
        }
        self.max_index = i - 1;
    }

    pub fn compress(&mut self) {
        let to = if self.length < self.n {
            self.end
        } else {
            self.max_index
        };
        let mut j = 0;
        for i in 1..=to {
            //if !self.d[self.w[i]] {
            if self.d[self.w[i] / usize::BITS as usize] & (1 << self.w[i] % usize::BITS as usize)
                == 0
            {
                j += 1;
                self.w[j] = self.w[i];
            }
        }
        if to == self.end {
            self.end = j;
        } else {
            for k in j + 1..=to {
                self.w[k] = 0;
            }
        }
    }
}

/// Pritchard's wheel sieve
pub fn sieve(n: usize) -> impl Iterator<Item = usize> {
    if n < 2 {
        return Vec::new().into_iter();
    }

    let mut wheel = Wheel::new(n);
    let mut primes = vec![2];
    let mut p = 3;
    while p * p <= n {
        primes.push(p);
        if wheel.length < n {
            wheel.extend(usize::min(p * wheel.length, n));
        }
        wheel.delete(p);
        wheel.compress();
        p = wheel.w[1];
        if p == 0 {
            break;
        }
    }
    if wheel.length < n {
        wheel.extend(n);
    }

    for i in 1..=wheel.end {
        //if wheel.w[i]!=0 && !wheel.d[wheel.w[i]] {
        if wheel.w[i] != 0
            && (wheel.d[wheel.w[i] / usize::BITS as usize]
                & (1 << wheel.w[i] % usize::BITS as usize)
                == 0)
        {
            primes.push(wheel.w[i]);
        }
    }

    primes.into_iter()
}

#[test]
fn test_pritchard2bv() {
    let l: Vec<usize> = sieve(10).collect();
    assert_eq!(l, vec![2, 3, 5, 7]);
}
