
import os
import random
import sys
import networkx
from typing import List, Tuple

"""
Generate random small (5 <= n <= 30) Erdos graphs 
and save them as files in the specified directory.
Takes one command line parameter: directory to store generated graphs
WIll create directory if it doesn't exist. Will overwrite any conflicting filename
"""

def generate_graph(n: int, p: float) -> networkx.Graph:
    graph = networkx.gnp_random_graph(n, p)
    return graph


def save_graph(graph: networkx.Graph, filename: str):
    edges = [(x,y) for x,y in graph.edges if x < y] 
    with open(filename, mode='w+') as file:
        print(f'{graph.number_of_nodes()} {len(edges)}')
        for x,y in edges:
            print(x, y, file=file)
        file.flush()


def main():
    instances_folder = sys.argv[1]

    probs = [0.05, 0.1]
    
    small_sizes = list(sorted(random.randint(5,30) for _ in range(200)))
    medium_sizes = list(sorted(random.randint(31,50) for _ in range(200)))

    for p in probs:
        format_p = str(p).replace('.', '')

        try:
            os.makedirs(os.path.join(instances_folder, 'small', format_p))
            os.makedirs(os.path.join(instances_folder, 'medium', format_p))
        except:
            pass

        for i,n in enumerate(small_sizes):
            graph = generate_graph(n, p)
            filename = os.path.join(instances_folder, 'small', format_p, f'graph_{p}_{i}.input')
            save_graph(graph, filename)
        
        for i,n in enumerate(medium_sizes):
            graph = generate_graph(n, p)
            filename = os.path.join(instances_folder, 'medium', format_p, f'graph_{p}_{i}.input')
            save_graph(graph, filename)


if __name__ == '__main__':
    main()

