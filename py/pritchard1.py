import argparse
import math 
import sys

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
        self.w=[1]+[x for x in self.w[1:] if x%p!=0]


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
    parser.add_argument('-n', type=int, default=200, help="upper end of interval")
    args=parser.parse_args()

    N=args.n

    primes=sieve_of_pritchard(N)
    for p in primes:
        print(p)
        
