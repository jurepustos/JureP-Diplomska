#!/bin/bash
sparse_probs=("005" "01")
dense_probs=("02" "03" "04" "05" "06")

# $1..: vc/pure_dlx/reduce_dlx

run_tests () {
    for p in "${sparse_probs[@]}"; do
        python3 run_vc.py ./run_$1.sh ./instances/sparse/$p > ./results/sparse/vc_$1_sparse_${p}_results.txt
    done

    for p in "${dense_probs[@]}"; do
        python3 run_vc.py ./run_$1.sh ./instances/dense/$p > ./results/dense/vc_$1_dense_${p}_results.txt
    done
}

for algo in "${@:1}"; do
    run_tests $algo
done
