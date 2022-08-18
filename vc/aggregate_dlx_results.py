from itertools import groupby
from math import inf
from operator import itemgetter
import sys


"""
Aggregates timing results.
Takes one parameter: filename of the timing results.
"""

def parse_int(x):
    try:
        return int(x)
    except:
        return None


def main():
    filename = sys.argv[1]
    with open(filename) as file:
        lines = [[int(n) for n in line.split(' ')] for line in file.readlines()]

    data = {line[0]: [] for line in lines}
    for [n,m,t] in lines:
        data[n].append(m)

    averages = {n: sum(times)/len(times) for n,times in data.items()}
    for n, time in averages.items():
        print(n, round(time/1000, 3))

    

if __name__ == '__main__':
    main()
