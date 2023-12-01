package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Pair struct {
	A int
	B int
}

type Threes struct {
	A int
	B int
	C int
}

func parseByteSliceToLines(data []byte) []string {
	lines := strings.Split(string(data), "\n")

	return lines
}

func main() {
	file, err := os.ReadFile("./20/input/20_01")
	if err != nil {
		fmt.Println("bad file", err)
		os.Exit(1)
	}

	lines := parseByteSliceToLines(file)
    fmt.Println("lines: ", lines)
	numbers := []int{}
	for i := range lines {
        if lines[i] == "" {continue}
		number, err := strconv.Atoi(lines[i])
		if err != nil {
			fmt.Println("error in parsing: ", err)
		}
        numbers = append(numbers, number)
	}

	var pair *Pair
	var threes *Threes

	for i := range numbers {
        for j := i; j < len(numbers); j++ {
            // loop body remains unchanged
            for x := j; x < len(numbers); x++ {
               if numbers[i] + numbers[j] + numbers[x] == 2020 {
                    threes = &Threes{A:numbers[i], B: numbers[j], C: numbers[x]} 
                    break
               } 
            }
            fmt.Println("adding: ", i, j, numbers[i]+numbers[j])
			if numbers[i]+numbers[j] == 2020 {
                fmt.Println("Found sum: ", i, j)
				pair = &Pair{A: numbers[i], B: numbers[j]}
				break
			}
		}
		if pair != nil && threes != nil{
			break
		}
	}

	fmt.Println("product is: ", pair.A*pair.B)
	fmt.Println("larger product is: ", threes.A*threes.B*threes.C)
	fmt.Println(threes)
}
