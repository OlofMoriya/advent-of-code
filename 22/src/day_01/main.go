package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// Read input from a file named "input.txt" in the current directory.
	data, err := os.ReadFile("../../input/22_01")
	if err != nil {
		// Print any errors and exit.
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}


    lines := parseByteSliceToLines(data)

    sums := []int{}

    current_sum := 0
	for line := range lines {

		if lines[line] == "" {

			sums = append(sums, current_sum)
			current_sum = 0
			continue
		}
		line_int, err := strconv.Atoi(lines[line])
		if err != nil {
			fmt.Println("error parsing {}", lines[line])
			os.Exit(1)
		}
		current_sum += line_int
	}


    sort.Slice(sums, func(i, j int) bool {
       return sums[i] > sums[j] // Compare the values at index i and j for correct sorting.
    })

    fmt.Println("Largest sum is ", sums[0])
    fmt.Println("Three largest sum is ", sums[0] + sums[1] + sums[2])
}

func parseByteSliceToLines(data []byte) []string {
	// Convert []byte to string then split by new lines.
	lines := strings.Split(string(data), "\n")
	return lines
}
