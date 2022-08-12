from math import ceil, floor, inf
from operator import itemgetter
import sys

def main():
    filenames = sys.argv[1:]
    contents = []
    for filename in filenames:
        with open(filename) as file:
            contents.append([line.split(' ') for line in file.readlines()])

    keys = {int(line[0]) for content in contents for line in content}

    data_rows = {key: [[int(line[1])
                    for line in content if int(line[0]) == key] 
                for content in contents]
            for key in keys}

    avg_rows = [(key, [(round(sum(values) / len(values)) if len(values) >= 7 else '-') for values in row]) 
                for key, row in data_rows.items()]
    avg_rows.sort(key=itemgetter(0))
    for key, avg in avg_rows:
        print(key, *avg, sep=' & ', end=' \\\\\n')
        

if __name__ == '__main__':
    main()



