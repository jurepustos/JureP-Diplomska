import itertools
from operator import itemgetter
import os
import sys
from unittest import result

"""
Generate table data
"""


def process_files(*args):
    total = 0
    contents = []
    for filename in args:
        with open(filename) as file:
            contents.extend([[int(n) for n in line.split(' ')] for line in file.readlines()])
    # sort by time elapsed
    contents.sort(key=itemgetter(2))

    time_intervals = range(0, 60001, 100)
    for interval in time_intervals:
        interval_group = [line for line in contents if interval <= int(line[2]) < interval+100]
        total += len(interval_group)
        print(round(interval/1000, 1), total)


def main():
    results_files = sys.argv[1:]
    process_files(*results_files)


if __name__ == '__main__':
    main()
