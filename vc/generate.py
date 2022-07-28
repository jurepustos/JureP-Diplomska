import os
import random
import sys
import networkx
from typing import List, Tuple


"""
Generate random Erdos graphs and save them as files in the specified directory.
Takes one command line parameter: directory to store generated graphs
WIll create directory if it doesn't exist. Will overwrite any conflicting filename
"""

def main():
    folder = sys.argv[1]
    if not os.path.exists(folder):
        os.makedirs(folder)
    
    for n in range(10, 151, 10):
        edges_samples = random.sample(range(2*n, n*(n-1)), 10)
        for m in edges_samples:
            graph = networkx.gnm_random_graph(n, m)
            edges = [(x,y) for x,y in graph.edges if x < y]
            with open(f'{folder}/graph{n}_{m}.input', mode='w+') as file:
                print(f'{n} {m}')
                for x,y in edges:
                    print(x, y, file=file)
                
                file.flush()


if __name__ == '__main__':
    main()
