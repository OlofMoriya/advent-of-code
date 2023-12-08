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

    bytes, err := os.ReadFile("/Users/olofmoriya/versioned/personal/advent-of-code/" + year +"/input/" + year + "_" + day + suffix)

    if err != nil {
        fmt.Println("Failed to read file", err)
        os.Exit(1)
    }
    
    all_strings := strings.Split(string(bytes), "\n")
    all_strings = all_strings[:len(all_strings)-1]
    return all_strings
}


func main() {
	lines := FileToStrings("23", "06", true)

    fmt.Println(lines)
}
