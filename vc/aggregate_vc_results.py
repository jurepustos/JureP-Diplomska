from itertools import groupby
from math import inf
from operator import itemgetter
import sys


def parse_int(x):
    try:
        return int(x)
    except:
        return inf


def main():
    filename = sys.argv[1]
    with open(filename) as file:
        lines = map(lambda line: line.split(' '), file.readlines())
    data = [(int(line[0]), parse_int(line[2])) for line in lines]
    data.sort()
    # for n, time in data:
    #     print(n, time)

    time_groups = [(n, [time for _, time in group if time != inf]) for n, group in groupby(data, key=itemgetter(0))]
    averages = [(n, 10 - len(times), sum(times) / len(times)) for n, times in time_groups]
    for n, timeouts, times in averages:
        print(n, timeouts, times)

    

if __name__ == '__main__':
    main()
