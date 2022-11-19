
Calculate all primes below a given integer.  Parallel implementations in Python and Rust.

| #primes / n            |    | 9592 / 10\*\*5 | 78498 / 10\*\*6 | 664579 / 10\*\*7 | 5761455 / 10\*\*8 | 50847534 / 10\*\*9 |
| ---------------------- | -- | -------------- | --------------- | ---------------- | ----------------- | ------------------ |
| Eratostenes            | py |                | 0.26            | 2.3              | 23.9              |                    |
| Eratosthenes BitVec    | py | 42.05          |                 |                  |                   |                    |
| Segmented Eratosthenes | py | 0.35           | 0.533           | 2.7              | 23.76             |                    |
| Pritchard1             | py | 0.396          | 1.281           | 30.406           |                   |                    |
| Pritchard2             | py | 0.333          | 0.438           | 2.282            | 16.736            |                    |
| Rolling Sorenson       | py | 0.393          | 1.228           | 10.217           | 101.13            |                    |
|                        |    |                |                 |                  |                   |                    |
| Eratostenes            | rs | 0.389          | 0.636           | 1.125            | 7.517             | 66.74              |
| Eratostenes BitVec     | rs | 0.392          | 0.462           | 1.168            | 6.621             | 59.174             |
