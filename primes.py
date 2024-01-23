# Python3
# Count number of primes from 1 to 10^8 using sieve of Eratosthenes

import math
import time


def sieve(n):
    # Create a boolean array "prime[0..n]" and initialize
    # all entries it as true. A value in prime[i] will
    # finally be false if i is Not a prime, else true.
    prime = [True for i in range(n + 1)]
    prime[0] = False
    prime[1] = False
    p = 2
    while p * p <= n:
        # If prime[p] is not changed, then it is a prime
        if prime[p] == True:
            # Update all multiples of p
            for i in range(p * p, n + 1, p):
                prime[i] = False
        p += 1
    # Print all prime numbers
    # for p in range(2, n):
    #    if prime[p]:
    #        print(p)
    return prime


def main():
    start_time = time.time()
    n = 100000000
    res = sieve(n)
    print("Number of primes from 1 to 10^8 is: ", res.count(True))
    print("Time elapsed: ", time.time() - start_time)
    total = 0
    for i in range(n):
        if res[i]:
            total += i
    print("Sum of primes from 1 to 10^8 is: ", total)


main()
