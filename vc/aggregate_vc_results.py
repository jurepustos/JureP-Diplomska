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
        lines = map(lambda line: line.split(' '), file.readlines())
    data = [(int(line[0]), parse_int(line[2])) for line in lines]
    data.sort(key=itemgetter(0))
    # for n, time in data:
    #     print(n, time)

    time_groups = [(n, [time for _, time in group if time is not None]) for n, group in groupby(data, key=itemgetter(0))]
    averages = [(n, 10 - len(times), sum(times) / len(times)) for n, times in time_groups]
    for n, timeouts, time in averages:
        print(n, timeouts, int(time))

    

if __name__ == '__main__':
    main()
