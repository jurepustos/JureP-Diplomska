#!/bin/bash
probs=("005" "01" "02" "03" "04")

# $1: small/medium/large/xlarge
# $2..: solver/pure_dlx/reduce_dlx 

run_tests () {
    mkdir ./results/$1;
    for p in "${probs[@]}"; do
        python3 run_vc.py ./run_$2.sh ./instances/$1/$p > ./results/$1/vc_$2_${p}_$1_results.txt
    done
}

for algo in "${@:2}"; do
    run_tests $1 $algo
done