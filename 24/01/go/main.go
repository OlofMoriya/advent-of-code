package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func FileToStrings(year string, day string, test bool) []string {
	suffix := ""
	if test {
		suffix = "_test"
	}

	bytes, err := os.ReadFile("/Users/olofmoriya/versioned/personal/advent-of-code/" + year + "/" + day + "/" + day + suffix)

	if err != nil {
		fmt.Println("Failed to read file", err)
		os.Exit(1)
	}

	all_strings := strings.Split(string(bytes), "\n")
	all_strings = all_strings[:len(all_strings)-1]
	return all_strings
}

// num, err := strconv.Atoi(strings[i])

func main() {
	inputLines := FileToStrings("24", "01", false)
	left := []int{}
	right := []int{}
	for _, s := range inputLines {
		parts := strings.Fields(s)

		leftN, err := strconv.Atoi(parts[0])
		rightN, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Panic("not parsable", err)
		}

		left = append(left, leftN)
		right = append(right, rightN)
		// fmt.Println(left)
		// fmt.Println(right)

	}
	dict := make(map[int]int)
	sum := 0
	for _, num := range right {
		// dict[num] will return 0 if it hasn't yet been initialised
		dict[num] = dict[num] + 1
	}
	for _, num := range left {
		// fmt.Println("num", num, " count", dict[num])
		sum += (num * dict[num])
	}
	fmt.Println("sum", sum)
}
