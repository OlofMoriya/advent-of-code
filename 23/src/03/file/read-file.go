package file

import (
	"fmt"
	"os"
	"strconv"
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


func FileToInts(year string, day string, test bool) []int {
    suffix := ""
    if test {
        suffix = "_test"
    }

    bytes, err := os.ReadFile("/Users/olofmoriya/versioned/personal/advent-of-code/" + year +"/input/" + year + "_" + day + suffix)

    if err != nil {
        fmt.Println("Failed to read file", err)
        os.Exit(1)
    }
    
    strings := strings.Split(string(bytes), "\n")
    nums := []int{}
    for i := range strings {
        if strings[i] != "" {
            num, err := strconv.Atoi(strings[i])
            if err != nil {
                fmt.Println("error parsing to number", err)
            }
            nums = append(nums, num)
        }
    }
    return nums
}
