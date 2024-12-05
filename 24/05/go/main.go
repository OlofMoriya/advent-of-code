package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func FileToStrings(year string, day string, test bool) []byte {
	suffix := ""
	if test {
		suffix = "_test"
	}

	bytes, err := os.ReadFile("/Users/olofmoriya/versioned/personal/advent-of-code/" + year + "/" + day + "/" + day + suffix)

	if err != nil {
		fmt.Println("Failed to read file", err)
		os.Exit(1)
	}
	return bytes
}

func main() {
	bytes := FileToStrings("24", "05", false)
	sections := strings.Split(string(bytes), "\n\n")
	tests := strings.Split(sections[0], "\n")
	updates := strings.Split(sections[1], "\n")

	ok := []int{}
	notOk := []int{}

updatesloop:
	for _, update := range updates {
		if update == "" {
			continue
		}
		pages := strings.Split(update, ",")

		for _, test := range tests {
			testPages := strings.Split(test, "|")
			first := indexOf(testPages[0], pages)
			second := indexOf(testPages[1], pages)
			if first > second && first > -1 && second > -1 {
				fixed := fixUpdate(pages, tests)
				middlePage := pages[int(len(fixed)/2)]
				middleNum, err := strconv.Atoi(middlePage)
				if err != nil {
					fmt.Println(err)
				}
				notOk = append(notOk, middleNum)

				continue updatesloop
			}
		}

		middlePage := pages[int(len(pages)/2)]
		middleNum, err := strconv.Atoi(middlePage)
		if err != nil {
			fmt.Println(err)
		}
		ok = append(ok, middleNum)
	}

	total := 0
	for _, num := range ok {
		total += num
	}
	fmt.Println("total", total)

	totalNot := 0
	for _, num := range notOk {
		totalNot += num
	}
	fmt.Println("totalNot", totalNot)
}

func fixUpdate(pages []string, tests []string) []string {
	for _, test := range tests {
		testPages := strings.Split(test, "|")
		first := indexOf(testPages[0], pages)
		second := indexOf(testPages[1], pages)
		if first > second && first > -1 && second > -1 {
			testPages = swapIndexes(pages, first, second)
			return fixUpdate(pages, tests)
		}
	}

	return pages
}

func swapIndexes(slice []string, first int, second int) []string {
	temp := slice[first]
	slice[first] = slice[second]
	slice[second] = temp
	return slice
}

func indexOf(element string, data []string) int {
	for k, v := range data {
		if element == v {
			return k
		}
	}
	return -1 // not found
}
