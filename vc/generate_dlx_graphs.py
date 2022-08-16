
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
        print(f'#{graph.number_of_nodes()} {len(edges)}', file=file)
        for x,y in edges:
            print(x, y, file=file)
        file.flush()


def generate_small(instances_folder: str):
    probs = [0.05, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6]
    
    sizes = [n for n in range(10,51) for _ in range(10)]

    for p in probs:
        format_p = str(p).replace('.', '')

        os.makedirs(os.path.join(instances_folder, 'small', format_p), exist_ok=True)

        for i,n in enumerate(sizes):
            graph = generate_graph(n, p)
            filename = os.path.join(instances_folder, 'small', format_p, f'small_graph_{p}_{i}.input')
            save_graph(graph, filename)


def generate_medium(instances_folder: str):
    probs = [0.2]
    
    sizes = [n for n in range(51,71) for _ in range(10)]

    for p in probs:
        format_p = str(p).replace('.', '')

        os.makedirs(os.path.join(instances_folder, 'medium', format_p), exist_ok=True)

        for i,n in enumerate(sizes):
            graph = generate_graph(n, p)
            filename = os.path.join(instances_folder, 'medium', format_p, f'medium_graph_{p}_{i}.input')
            save_graph(graph, filename)


def generate_large(instances_folder: str):
    probs = [0.4, 0.5, 0.6]
    
    sizes = [n for n in range(71,101) for _ in range(10)]

    for p in probs:
        format_p = str(p).replace('.', '')

        os.makedirs(os.path.join(instances_folder, 'large', format_p), exist_ok=True)

        for i,n in enumerate(sizes):
            graph = generate_graph(n, p)
            filename = os.path.join(instances_folder, 'large', format_p, f'large_graph_{p}_{i}.input')
            save_graph(graph, filename)


def main():
    size = sys.argv[1]
    instances_folder = sys.argv[2]

    if size == 'small':
        generate_small(instances_folder)
    if size == 'medium':
        generate_medium(instances_folder)
    if size == 'large':
        generate_large(instances_folder)


if __name__ == '__main__':
    main()

