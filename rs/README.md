# primes

```
 % cargo build --release
```

This builds targets the following targets:
 * primes - Eratostenes with bit vector
 * pritchard2 - Pritchard's wheel sieve
 * pritchard2bv - Pritchard's wheel sieve with bit vector
 * sorenson - Sorenson's rolling sieve - the only incremental algorithm in the list.

Execute each of the targets like so

```
 % target/release/sorenson -n 100
2
3
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
```


