import sys

"""
Generate table data to put into Latex
Takes three filenames as parameters
"""

def main():
    filename1 = sys.argv[1]
    filename2 = sys.argv[2]
    filename3 = sys.argv[3]
    with open(filename1) as file1, open(filename2) as file2, open(filename3) as file3:
        contents1 = list(map(lambda line: line.split(' '), file1.readlines()))
        contents2 = list(map(lambda line: line.split(' '), file2.readlines()))
        contents3 = list(map(lambda line: line.split(' '), file3.readlines()))
    
    data1 = {int(line[0]): (int(line[1]), int(line[2])) for line in contents1}
    data2 = {int(line[0]): (int(line[1]), int(line[2])) for line in contents2}
    data3 = {int(line[0]): (int(line[1]), int(line[2])) for line in contents3}

    keys = list(set(data1.keys()).union(set(data2.keys())).union(set(data3.keys())))
    keys.sort()
    for i,key in enumerate(keys):
        print(key, '&', end=' ')
        print(data1[key][0], '&', data1[key][1], '&', end=' ')
        print(data2[key][0], '&', data2[key][1], '&', end=' ')
        print(data3[key][0], '&', data3[key][1], end=' ')
        print('\\\\')
        

if __name__ == '__main__':
    main()
