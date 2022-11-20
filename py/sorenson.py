import argparse
import math 

def rolling_sorenson():
    """Sorenson's rolling sieve - yield next prime each time it is called.
       Complexity: O(nloglogn) time, O(sqrt(n)log(n)) bits,
                   O(logn/loglogn) incremental   

       See algorithm description in paper:
       "Two Compact Incremental Prime Sieves", Jonathan P. Sorenson, Journal of Computation and Mathematics, 2015
       https://arxiv.org/abs/1503.02592
    """     
    start=100

    primes=[ 2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 
            31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
            73, 79, 83, 89, 97]

    yield from primes[:start]

    r=math.floor(math.sqrt(start))+1
    s=r*r
    delta=r+2   

    T=[] #array of stacks
    for i in range(delta+1):
        T+=[[]]
    for p in [p for p in primes if p<=r-1]:
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

    for i,p in enumerate(rolling_sorenson()):
        if p>N: break  
        print(p)
