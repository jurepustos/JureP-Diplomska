from math import inf
import sys


def parse_int(x):
    try:
        return int(x)
    except:
        return '-'

def main():
    filename1 = sys.argv[1]
    filename2 = sys.argv[2]
    with open(filename1) as file1, open(filename2) as file2:
        contents1 = list(map(lambda line: line.split(' '), file1.readlines()))
        contents2 = list(map(lambda line: line.split(' '), file2.readlines()))

    data1 = {int(line[0]): parse_int(line[1]) for line in contents1}
    data2 = {int(line[0]): parse_int(line[1]) for line in contents2}

    keys = list(set(data1.keys()).union(set(data2.keys())))
    keys.sort()
    for i,key in enumerate(keys):
        print(key, '&', data1.get(key, None), '&', data2.get(key, None), '&' if i % 2 == 0 else '\\\\')


if __name__ == '__main__':
    main()



