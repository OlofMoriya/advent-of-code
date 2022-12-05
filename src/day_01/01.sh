#!/bin/bash

cat "./input/22_01_test" | sed -z 's/\n/+/g' | sed -z 's/++/\n/g' | bc

