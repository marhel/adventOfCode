from collections import defaultdict
from math import gcd

with open('input.txt', 'r') as content_file:
    data = content_file.read()

def aoc13_2():
    # d is {depth:range}, eg:
    d = eval("{" + data.strip().replace('\n', ',') + "}")

    neq = defaultdict(list)  # of the form {b:[a1,a2...]} where delay != a_i (mod b)
    for depth in d.keys():
        neq[d[depth] * 2 - 2] += [(-depth) % (d[depth] * 2 - 2)]
    moduli = sorted(neq.keys())

    prev_lcm = 1
    lcm = 1
    residues = [0]  # mod 1
    for m in moduli:
        g = gcd(m, lcm)  # simple Euclidean algorithm
        prev_lcm = lcm
        lcm = lcm * m // g  # new modulus
        residues = [x for i in residues 
                    for x in range(i, lcm, prev_lcm) 
                    if x % m not in neq[m]]
    # print(sorted(residues)[0], "(mod", lcm, ")")  # the smallest residue
    return sorted(residues)[0], lcm

if __name__ == '__main__':
    import timeit
    duration = timeit.timeit("aoc13_2()", number=1000, setup="from __main__ import aoc13_2")
    print(f"1000 runs in {duration:.3f}s")
