from math import ceil, floor, inf
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

    data_rows = []
    for key in keys:
        data_rows.append(f'{key} & {data1[key]} & {data2[key]}')

    third = ceil(len(data_rows) / 3)
    for i in range(third):
        print(data_rows[i], '&')
        print(data_rows[i + third], '&')
        try:
            print(data_rows[i + 2*third], '\\\\')
        except:
            print('& & \\\\')

    print()


if __name__ == '__main__':
    main()



