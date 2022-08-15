#!/bin/bash
probs=("005" "01")

run_tests () {
    for p in "${probs[@]}"; do
        python3 run_vc.py ./run_$1.sh ./instances/small/small/$p > ./results/small/vc_small_$1_${p}_results.txt
    done

    for p in "${probs[@]}"; do
        python3 run_vc.py ./run_$1.sh ./instances/small/medium/$p > ./results/small/vc_medium_$1_${p}_results.txt
    done
}

for algo in "${@:1}"; do
    run_tests $algo
done