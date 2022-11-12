import math 

def prime_sieve(n):
    """Sieve of Eratosthenes - calc all primes below n 
       Complexity: Time O(nloglog(n)), Space O(n)
    """
    l=[True]*n
    l[0]=False
    l[1]=False
    #q=math.ceil(math.sqrt(len(l)))
    #for i,x in enumerate(l[:q]): #slower?!
    for i,x in enumerate(l):
        if x:
            yield i
            for y in range(i*i,N,i):  #note - composite numbers cleared multiple times 
                l[y]=False
    
def seg_prime_sieve(n):
    """Segmented sieve of Eratosthenes - calc all primes up to n
       Complexity: Time O(nloglog(n)), Space O(sqrt(n))
    """
    delta=math.ceil(math.sqrt(n))    #segment size - at least sqrt(n)
    primes=list(prime_sieve(delta+1))
    yield from primes          
    new_primes=[]      
    for left in range(delta+1,n+1,delta):
        right=min(left+delta-1,n)
        l=[True]*(right-left+1)
        for p in primes:
            for q in range(left+(p-left%p)%p,right+1,p): #multiples of p
                l[q-left]=False
        yield from [i+left for i,x in enumerate(l) if x]

if __name__=="__main__":
    N=10**7    
    print(f"{N=}")

    #primes=seg_prime_sieve(N)
    primes=prime_sieve(N)
    primes=list(primes)
    print(f"#primes {len(primes)}")
    if len(primes)<10:
        print(primes)
    else:
        print(primes[:5], "...", primes[-5:])
        
