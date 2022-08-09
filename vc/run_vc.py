from concurrent.futures import ThreadPoolExecutor
import os
import string
import sys
import subprocess

"""
Run the specified VC solver on all test cases and output the timing results.
Takes two parameters: 
1. the executable to run as the solver
2. directory containing test inputs
The executable should take just one parameter: filename for the input file
"""

def run_program(program: str, filename: str) -> bytes:
    print('Starting', filename, file=sys.stderr)
    proc = subprocess.Popen([program, filename], stdout=subprocess.PIPE)
    proc.wait()
    output = proc.stdout.read()
    return output

def main():
    program = sys.argv[1]
    instances_folder = sys.argv[2]
    instances = os.listdir(instances_folder)
    with ThreadPoolExecutor(max_workers=14) as executor:
        param_sets = [os.path.join(instances_folder, instance) for instance in instances]
        for output in executor.map(lambda file: run_program(program, file), param_sets):
            try:
                n, m, time = output.decode().split(' ')
                print(n, m, time.strip())
            except ValueError:
                print(n, m, '-')


if __name__ == '__main__':
    main()
