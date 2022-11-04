import math 
#Sieve of Eratosthenes - calc all primes below n 
def prime_sieve(N):
    l=[True]*N
    l[0]=False
    l[1]=False
    #q=math.ceil(math.sqrt(len(l)))
    #for i,x in enumerate(l[:q]): #slower?!
    for i,x in enumerate(l):
        if x:
            for y in range(i*i,N,i):
                l[y]=False
    return [i for i,x in enumerate(l) if x]


#N=31
N=10**7    
print("N=",N)
primes=prime_sieve(N)
print("#primes", len(primes))
if len(primes)<10:
    print(primes)
else:
    print(primes[:5], "...", primes[-5:])
    
