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
    """same as prime_sieve - but use bitvector instead of list of bool; slower, but more memory efficient
    """
    l=sum((2**i for i in range(n))) # bit vector - set all 
    l^=2**0
    l^=2**1
    for i in range(n):
        if l&2**i:           
            yield i
            for y in range(i*i,n,i):  #note - composite numbers cleared multiple times 
                l&=~2**y

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


class Wheel:
    def __init__(self):
        # Wheel 1 
        # w - subset of numbers between 1 and length
        # length - product of first i primes p1*p2*...*pi
        self.w=[1]
        self.length=2

    def __str__(self):
        return f'{self.length=}\n{self.w=}'

    def extend(self,n):
        """Roll the wheel - obtain wheel k+1"""
        i=0
        x=self.w[i]+self.length
        while x<=n:
            self.w+=[x]
            i=i+1
            x=self.w[i]+self.length
        self.length=n

    def delete(self,p):
        """delete multiples of p"""
        self.w=[1]+[x for x in self.w[2:] if x%p!=0]
        
def sieve_of_pritchard(N):
    """ Paul Pritchard's Wheel Sieve 
        Complexity: Time O(n/loglog(n)), Space O(n)
        https://en.wikipedia.org/wiki/Sieve_of_Pritchard
    """  
    W=Wheel()
    yield 2
    p=3
    while p*p<N:
        yield p
        if W.length<N:
            W.extend(min(N,p*W.length))
        W.delete(p) 
        p=W.w[1]
    if W.length<N:
        W.extend(N)
    yield from W.w[1:]

if __name__=="__main__":
    parser=argparse.ArgumentParser("Calculate all primes up to some integer")
    parser.add_argument('-n', type=int, default=10**2, help="upper end of interval")
    args=parser.parse_args()

    N=args.n

    #primes=prime_sieve(N)
    #primes=prime_sieve_bv(N)
    #primes=seg_prime_sieve(N)
    primes=sieve_of_pritchard(N)

    for p in primes:
        print(p)
        
