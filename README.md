Prime sieves - calculate all primes below a given integer n. Implementations in Python3 and Rust.

Table shows time in seconds on my laptop for different values of n - 100.000 to 10.000.000.000.

| #primes / n            |    | 9592 / 10\*\*5 | 78498 / 10\*\*6 | 664579 / 10\*\*7 | 5761455 / 10\*\*8 | 50847534 / 10\*\*9 | 455052511 / 10\*\*10 |
| ---------------------- | -- | -------------- | --------------- | ---------------- | ----------------- | ------------------ | -------------------- |
| Eratostenes            | py | 0.142          | 0.26            | 2.3              | 23.9              |                    |                      |
| Eratosthenes BitVec    | py | 1.176          | 110.5           |                  |                   |                    |                      |
| Segmented Eratosthenes | py | 0.35           | 0.533           | 2.7              | 23.76             |                    |                      |
| Pritchard1             | py | 0.396          | 1.281           | 30.406           |                   |                    |                      |
| Pritchard2             | py | 0.333          | 0.438           | 2.282            | 16.736            |                    |                      |
| Rolling Sorenson       | py | 0.393          | 1.228           | 10.217           | 101.13            |                    |                      |
|                        |    |                |                 |                  |                   |                    |                      |
| Eratostenes            | rs | 0.035          | 0.133           | 0.812            | 6.656             | 67.61              | 708.34               |
| Eratostenes BitVec     | rs | 0.056          | 0.11            | 0.837            | 6.553             | 70.3               | 674.99               |
| Segmented Eratosthenes | rs | 0.022          | 0.114           | 0.71             | 6.721             | 54.819             |                      |
| Pritchard2             | rs | 0.043          | 0.12            | 1.115            | 6.632             | 58.472             | 713.41               |
| Pritchard2 BitVec      | rs | 0.04           | 0.112           | 0.689            | 5.921             | 52.746             | 728.16               |
| Rolling Sorenson       | rs | 0.028          | 0.157           | 1.191            | 10.075            | 97.4               | 1241.34              |
