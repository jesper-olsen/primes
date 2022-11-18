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

def rolling_sorenson():
    start=100

    primes=[ 2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 
            31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
            73, 79, 83, 89, 97]
    n=0
    while n<len(primes[:start]):
        yield primes[n]
        n+=1

    r=math.floor(math.sqrt(start))+1
    s=r*r
    delta=r+2   

    T=[] #array of stacks
    for i in range(delta+1):
        T+=[[]]
    for p in prime_sieve(r-1):
        j=(p-(start%p))%p
        T[j]+=[p]
    pos=0
    n=start
    while True:
        isPrime=True
        while not T[pos]==[]:
            p=T[pos].pop()
            T[(pos+p)%delta]+=[p]
            isPrime=False
        if n==s:
            if isPrime:
                T[(pos+r)%delta]+=[r]
                isPrime=False
            r=r+1
            s=r*r
        n=n+1
        pos=(pos+1)%delta
        if pos==0:
            delta+=2
            T+=[[],[]]
        if isPrime: yield n-1

if __name__=="__main__":
    parser=argparse.ArgumentParser("Calculate all primes up to some integer")
    parser.add_argument('-n', type=int, default=200, help="upper end of interval")
    args=parser.parse_args()

    N=args.n

    primes=sieve_of_pritchard(N)
    for p in primes:
        print(p)
        
