package main

import (
	"fmt"
	"os"
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

func main() {
	lines := FileToStrings("24", "04", false)
	matrix := [][]string{}

	for _, l := range lines {
		chars := strings.Split(l, "")
		matrix = append(matrix, chars)
	}
	// fmt.Println(matrix)
	count := 0
	for r, row := range matrix {
		for c, letter := range row {
			if letter == "X" {
				// fmt.Println(r, c, letter)
				count += testForXmas(matrix, r, c)
			}
		}
	}

	fmt.Println("xmas count", count)

	count = 0
	for r, row := range matrix {
		for c, letter := range row {
			if letter == "A" {
				if testForMas(matrix, r, c) {
					count += 1

					// fmt.Println(r, c, letter)
				}
			}
		}
	}
	fmt.Println("x-mas count", count)
}

func testForMas(matrix [][]string, row int, col int) bool {
	if row == 0 || row == len(matrix)-1 || col == 0 || col == len(matrix[row])-1 {
		return false
	}

	if ((matrix[row-1][col-1] == "M" && matrix[row+1][col+1] == "S") ||
		(matrix[row-1][col-1] == "S" && matrix[row+1][col+1] == "M")) &&
		((matrix[row+1][col-1] == "M" && matrix[row-1][col+1] == "S") ||
			(matrix[row+1][col-1] == "S" && matrix[row-1][col+1] == "M")) {
		return true
	}
	return false
}

func testForXmas(matrix [][]string, row int, col int) int {
	mas := []string{"X", "M", "A", "S"}
	found := 0
	//we have X and need to find Mas in any direction
	for i := 1; i <= 3; i++ {
		//right
		// fmt.Println("right", i)
		if col+i >= len(matrix[row]) || matrix[row][col+i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found right")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		//right up
		// fmt.Println("right up", i)
		if row-i < 0 || col+i >= len(matrix[row]) || matrix[row-i][col+i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found right up")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		//right down
		// fmt.Println("right down", i)
		if row+i >= len(matrix) || col+i >= len(matrix[row]) || matrix[row+i][col+i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found right down")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		// up
		// fmt.Println("up", i)
		if row-i < 0 || matrix[row-i][col] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found up")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		// down
		// fmt.Println("down", i)
		if row+i >= len(matrix) || matrix[row+i][col] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found down")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		//left
		// fmt.Println("left", i)
		if col-i < 0 || matrix[row][col-i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found left")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		//left up
		// fmt.Println("left up", i)
		if row-i < 0 || col-i < 0 || matrix[row-i][col-i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found left up")
			found += 1
		}
	}
	for i := 1; i <= 3; i++ {
		//left down
		// fmt.Println("left down", i)
		if row+i >= len(matrix) || col-i < 0 || matrix[row+i][col-i] != mas[i] {
			break
		}

		if i == 3 {
			// fmt.Println("found left down")
			found += 1
		}
	}

	return found
}
