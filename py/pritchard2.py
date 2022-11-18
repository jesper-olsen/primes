import argparse

"""
    Sieve of Pritchard 
    - see https://rosettacode.org/wiki/Sieve_of_Pritchard
"""
class Wheel:
    def __init__(self, n):
        self.n=n
        self.w=[1]+[0]*(n//4+4)
        self.d=[False]*(n+1)
        self.w_end=0
        self.w_end_max=0
        self.length=2

    def extend(self, n):
        """roll wheel"""    
        i=0
        j=self.w_end
        x=self.length+1
        while x<=n:
            j+=1
            self.w[j]=x
            self.d[x]=False
            i+=1
            x=self.length+self.w[i]
        self.length=n
        self.w_end=j
        if self.w_end>self.w_end_max:
            self.w_end_max=self.w_end   

    def delete(self,p):   
        i=0
        x=p
        while x<=self.length:
            self.d[x]=True
            i+=1
            x=p*self.w[i]
        self.imaxf=i-1

    def compress(self):
        if self.length<self.n:
            to=self.w_end
        else:
            to=self.imaxf
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

def sieve_of_pritchard(N):
    """generate primes up to N"""
    W=Wheel(N)
    yield 2
    p=3
    while p*p<=N:
        yield p
        if W.length<N:
            W.extend(min(p*W.length,N))
        W.delete(p)   
        W.compress()
        p=W.w[1]
        if p==0: break  
    if W.length<N:
        W.extend(N)
    for i in range(1,W.w_end+1):
        if W.w[i]!=0 and not W.d[W.w[i]]: yield W.w[i]

if __name__=="__main__":
    parser=argparse.ArgumentParser("Calculate all primes up to some integer")
    parser.add_argument('-n', type=int, default=100, help="upper end of interval")
    args=parser.parse_args()

    N=args.n
    for p in sieve_of_pritchard(N):
        print(p)    
