

import os
import shutil


def add_graphs(graphs, instances, p):
    for instance_file in instances:
        with open(os.path.join('instances', 'sparse', p,  instance_file)) as file:
            edges = [[int(n) for n in line.split(' ')] for line in file.readlines()]
            n = max(max(x,y) for x,y in edges) + 1
            m = len(edges)
            if (n,m) not in graphs:
                graphs[(n,m)] = [instance_file]
            else:
                graphs[(n,m)].append(instance_file)



def main():
    instances_005 = os.listdir(os.path.join('instances', 'sparse', '005'))
    instances_01 = os.listdir(os.path.join('instances', 'sparse', '01'))
    
    graphs = dict()
    add_graphs(graphs, instances_005, '005')
    add_graphs(graphs, instances_01, '01')

    # for (n, m), instance_files in graphs.items():
        # print(n, m, instance_files)


    solver_sparse_005_file = os.path.join('results', 'vc_solver_sparse_005_results.txt')
    solver_sparse_01_file = os.path.join('results', 'vc_solver_sparse_01_results.txt')


    below_second_results = []
    with open(solver_sparse_005_file) as file:
        lines = [[int(n) for n in line.split(' ')] for line in file.readlines()]
        below_second_results.extend([(line[0], line[1]) for line in lines if line[2] <= 1000])

    easy_instances = []
    for n,m in below_second_results:
        try:
            for instance in graphs[(n,m)]:
                easy_instances.append(os.path.join('instances', 'sparse', '005', instance))
        except:
            pass
    
    below_second_results = []
    with open(solver_sparse_01_file) as file:
        lines = [[int(n) for n in line.split(' ')] for line in file.readlines()]
        below_second_results.extend([(line[0], line[1]) for line in lines if line[2] <= 1000])
    
    easy_instances = []
    for n,m in below_second_results:
        try:
            for instance in graphs[(n,m)]:
                easy_instances.append(os.path.join('instances', 'sparse', '01', instance))
        except:
            pass

    for instance in easy_instances:
        shutil.copy(instance, os.path.join('instances', 'easy'))


if __name__ == '__main__':
    main()
