#!/bin/bash
for algo in "${@:1}"; do
    python3 run_queens.py ./run_$algo.sh > ./results/queens_${algo}_results.txt
done