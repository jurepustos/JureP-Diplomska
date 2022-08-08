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

def generate_graph(n: int, p: float) -> networkx.Graph:
    graph = networkx.gnp_random_graph(n, p)
    return graph


def save_graph(graph: networkx.Graph, folder: str, filename: str):
    edges = [(x,y) for x,y in graph.edges if x < y]
    file_path = os.path.join(folder, filename) 
    with open(file_path, mode='w+') as file:
        print(f'{graph.number_of_nodes()} {len(edges)}')
        for x,y in edges:
            print(x, y, file=file)
        file.flush()


def main():
    instances_folder = sys.argv[1]
    sparse_folder = os.path.join(instances_folder, 'sparse')
    dense_folder = os.path.join(instances_folder, 'dense')

    sparse_probs = [0.05, 0.1]
    sparse_folders = dict()
    for p in sparse_probs:
        sparse_folders[p] = os.path.join(sparse_folder, str(p).replace('.', ''))

    dense_probs = [0.2, 0.3, 0.4, 0.5, 0.6]
    dense_folders = dict()
    for p in dense_probs:
        dense_folders[p] = os.path.join(dense_folder, str(p).replace('.', ''))

    for folder in sparse_folders.values():
        os.makedirs(folder, exist_ok=True)
    for folder in dense_folders.values():
        os.makedirs(folder, exist_ok=True)

    
    sizes = list(sorted(random.randint(10,200) for _ in range(200)))

    # sparse graphs
    for p,folder in sparse_folders.items():
        for i,n in enumerate(sizes):
            graph = generate_graph(n, p)
            filename = f'sparse_graph_{p}_{i}.input'
            save_graph(graph, sparse_folders[p], filename)

    # dense graphs
    for p,folder in dense_folders.items():
        for i,n in enumerate(sizes):
            graph = generate_graph(n, p)
            filename = f'dense_graph_{p}_{i}.input'
            save_graph(graph, dense_folders[p], filename)


if __name__ == '__main__':
    main()
