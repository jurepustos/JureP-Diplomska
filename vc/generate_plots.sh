#!/bin/bash
for algo in "${@:1}"; do
    python3 vc_plot.py results/vc_${algo}_sparse_005_results.txt results/vc_${algo}_sparse_01_results.txt > results/vc_${algo}_sparse_summary.txt
done

for algo in "${@:1}"; do
    python3 vc_plot.py results/vc_${algo}_dense_02_results.txt results/vc_${algo}_dense_03_results.txt results/vc_${algo}_dense_04_results.txt results/vc_${algo}_dense_05_results.txt results/vc_${algo}_dense_06_results.txt > results/vc_${algo}_dense_summary.txt
done

