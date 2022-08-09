from concurrent.futures import ThreadPoolExecutor
import os
import string
import sys
import subprocess

"""
Run the specified Queens solver from n=5 to n=80 and output the timing results.
Takes one parameter: 
1. the executable to run as the solver
The executable should take just one parameter: the chessboard size
"""

def run_program(program: str, n: int) -> bytes:
    print(f'Starting n={n}', file=sys.stderr)
    proc = subprocess.Popen([program, str(n)], stdout=subprocess.PIPE)
    proc.wait()
    output = proc.stdout.read()
    return output


def main():
    program = sys.argv[1]
    with ThreadPoolExecutor(max_workers=14) as executor:
        param_sets = [n for n in range(5, 81, 5) for _ in range(10)]
        for output in executor.map(lambda n: run_program(program, n), param_sets):
            try:
                n, duration = output.decode().split(' ')
                print(n, duration.strip())
            except ValueError:
                print('timeout', file=sys.stderr)


if __name__ == '__main__':
    main()