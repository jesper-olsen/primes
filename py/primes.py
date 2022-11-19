import argparse
import math 
import sys

def prime_sieve(n):
    """Sieve of Eratosthenes - calc all primes below n 
       Complexity: Time O(nloglog(n)), Space O(n)
    """
    l=[True]*n
    l[0]=False
    l[1]=False
    for i,x in enumerate(l):
        if x:
            yield i
            for y in range(i*i,n,i):  #note - composite numbers cleared multiple times 
                l[y]=False

def prime_sieve_bv(n):
    """same as prime_sieve - but use bitvector instead of list of bool; 
    """
    #l=sum((2**i for i in range(n))) # bit vector - set all 
    l=sum((1<<i for i in range(n))) # bit vector - set all 
    l&=~1<<0
    l&=~1<<1
    for i in range(n):
        if l&(1<<i):  
            yield i
            for y in range(i*i,n,i):  #note - composite numbers cleared multiple times 
                l&=~(1<<y)

def seg_prime_sieve(n):
    """Segmented sieve of Eratosthenes - calc all primes up to n
       Complexity: Time O(nloglog(n)), Space O(sqrt(n))
    """
    delta=math.ceil(math.sqrt(n))    #segment size - at least sqrt(n)
    primes=list(prime_sieve(delta+1))
    yield from primes          
    for left in range(delta+1,n+1,delta):
        right=min(left+delta-1,n)
        l=[True]*(right-left+1)
        for p in primes:
            for q in range(left+(p-left%p)%p,right+1,p): #multiples of p
                l[q-left]=False
        yield from [i+left for i,x in enumerate(l) if x]


if __name__=="__main__":
    parser=argparse.ArgumentParser("Calculate all primes up to some integer")
    parser.add_argument('-n', type=int, default=100, help="upper end of interval")
    args=parser.parse_args()

    N=args.n

    #primes=prime_sieve(N)
    primes=prime_sieve_bv(N)
    #primes=seg_prime_sieve(N)
    for p in primes:
        print(p)
        
