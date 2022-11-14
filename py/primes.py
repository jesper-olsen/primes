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
            for y in range(i*i,n,i):  #note - composite numbers cleared multiple times 
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


class SieveOfPritchard:
   """ Paul Pritchard's Wheel Sieve 
       Complexity: Time O(n/loglog(n)), Space O(n)
       See "Explaining the Wheel Sieve", Paul Pritchard, Acta Informatica 17, 477-485 (1982).
   """  
   def __init__(self, n):
       self.n=n
       self.w=[1]*(n//4+5)
       self.d=[True]*(n+1)
       self.length=2
       self.w_end=0

   def extend(self, to):
       i=0
       j=self.w_end
       x=self.length+1
       while x<=to:
           j+=1
           self.w[j]=x
           self.d[x]=False
           i+=1
           x=self.length+self.w[i]
       self.length=to
       self.w_end=j

   def delete(self, p):
       i=0
       x=p
       while x<=self.length:
           self.d[x]=True
           i+=1
           x=p*self.w[i]
       return i-1

   def compress(self, to):
       j=0
       for i in range(1,to+1):
           if not self.d[self.w[i]]:
               j+=1
               self.w[j]=self.w[i]
       if to==self.w_end:
           self.w_end=j
       else:
           for k in range(j+1,to+1):
               self.w[k]=0

   def generate(self):
       yield 2
       p=3
       while p*p<=self.n:
           yield p
           if self.length<self.n:
               self.extend(min(p*self.length,self.n))
           imaxf=self.delete(p)
           if self.length<self.n:
               self.compress(self.w_end)
           else:
               while self.d[self.w[imaxf]]:
                   imaxf+=1
               self.compress(imaxf)
           p=self.w[1]
       if self.length<self.n:
           self.extend(self.n)
       for i in range(1,self.w_end+1):
           if self.w[i]!=0 and not self.d[self.w[i]]:
               yield self.w[i]

def sieve_of_pritchard(n):
    s=SieveOfPritchard(n)
    yield from s.generate()

if __name__=="__main__":
    N=10**7    

    #primes=seg_prime_sieve(N)
    #primes=prime_sieve(N)
    primes=sieve_of_pritchard(N)
    for p in primes:
        print(p)
        
