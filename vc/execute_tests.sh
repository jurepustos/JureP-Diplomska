#!/bin/bash
sparse_probs=("005" "01")
dense_probs=("02" "03" "04" "05" "06")

for p in "${sparse_probs[@]}"; do
    python3 run_vc.py ./run_solver.sh ./instances/sparse/$p > ./results/vc_solver_sparse_${p}_results.txt
done

for p in "${dense_probs[@]}"; do
    python3 run_vc.py ./run_solver.sh ./instances/dense/$p > ./results/vc_solver_dense_${p}_results.txt
done