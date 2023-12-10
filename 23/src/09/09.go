package main

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


type Sequence struct {
    Original []int
    Changes [][]int
}

func allValuesZero(list []int) bool {
    for _,v := range list {
        if v != 0 {
            return false
        }
    }
    return true
}

func main() {
	lines := FileToStrings("23", "09", false)

    sequences := []Sequence{}
    for _,line := range lines {
        original_strings := strings.Split(line, " ")
        original := []int{}

        for _,v := range original_strings {
            val,_ := strconv.Atoi(v)
            original = append(original, val)
        }
        sequence := Sequence{Original: original}
        allZero := false
        current_set := original
        for !allZero {
            change_set := []int{}
            for i,v := range current_set[1:] {
                change_set = append(change_set, v - current_set[i])
            }

            sequence.Changes = append(sequence.Changes, change_set)
            allZero = allValuesZero(change_set)
            current_set = change_set
        }
        sequences = append(sequences, sequence)
        // fmt.Println("sequence", sequence)
    }
    
    sum := 0
    for _,seq := range sequences {
        next_change := 0
        for i := len(seq.Changes) - 2; i >= 0; i-- {
            // fmt.Println("changes backwards", seq.Changes[i])
            next_change = next_change + seq.Changes[i][len(seq.Changes[i])-1]
        }

        next_value := seq.Original[len(seq.Original) -1] + next_change
        // fmt.Println("next value of", seq.Original, "is", next_value)
        sum += next_value
    }

    n_sum := 0
    for _,seq := range sequences {
        next_change := 0
        fmt.Println("seq", seq)
        for i := len(seq.Changes) - 2; i >= 0; i-- {
            // fmt.Println("changes backwards", seq.Changes[i])
            next_change = seq.Changes[i][0] - next_change 
            fmt.Println(next_change, seq.Changes[i])
        }

        next_value :=   seq.Original[0] - next_change 
        fmt.Println(next_value, seq.Original)
        // fmt.Println("next value of", seq.Original, "is", next_value)

        n_sum += next_value
    }

    fmt.Println("sum is", sum)
    fmt.Println("n_sum is", n_sum)

}
