#!/bin/bash

cat "../../input/22_01_test" | sed -z 's/\n/+/g' | sed -z 's/++/\n/g' | sed -z 's/.$/\n/' | bc | sort -n | tail -n 1

cat "../../input/22_01_test" | sed -z 's/\n/+/g' | sed -z 's/++/\n/g' | sed -z 's/.$/\n/' | bc | sort -n | tail -n 3 |sed -z 's/\n/+/g' | sed -z 's/.$/\n/' | bc
