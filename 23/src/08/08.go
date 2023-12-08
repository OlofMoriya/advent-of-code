package main

import (
	"fmt"
	"os"
	"strings"
	"regexp"
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

type Map struct {
    Current string
    Left    string
    Right   string
}

func main() {
    lines := FileToStrings("23", "08", false)

    fmt.Println(lines)
    maps := map[string]Map{}

    //first line is the LR instructions 
    directions := lines[0]
    current_nodes := []string{}
    //Third line and after is the direction translation
    for _,line := range lines[2:]{

        pattern := "(\\b[A-Z]+\\b) = \\((\\b[A-Z]+\\b), (\\b[A-Z]+\\b)\\)"

	    re := regexp.MustCompile(pattern)
	    match := re.FindStringSubmatch(line)

        //fmt.Println("match", match)
        maps[match[1]] = Map{Left: match[2], Right: match[3], Current: match[1]}
        if match[1][2] == 'A'{
            current_nodes = append(current_nodes, match[1])
        }
        fmt.Println("map added", maps[match[1]], "to:", match[1])
    }
    fmt.Println("dir", directions)
    //fmt.Println("maps", maps)

    steps := 1

    WHILE: 
    for  !allMatch(current_nodes, func(l string) bool {return l[2] == 'Z'}) {
    fmt.Println("I start at:", current_nodes)
        for _,d := range directions {
            for i,path := range current_nodes {
                var next string
                current_map, _ := maps[path]
                // fmt.Println("current", current_map)
                switch d {
                case 'L':
                    next = current_map.Left
                case 'R':
                    next = current_map.Right
                default:
                    fmt.Println("issue with directions... did not match L or R")
                    os.Exit(0)
                }

                //fmt.Println(current_nodes, string(d), " -> ", next)
                current_nodes[i] = next
            }

           if allMatch(current_nodes, func(l string) bool {return l[2] == 'Z'}) {
               break WHILE
           }
           steps++
        }
    }
    fmt.Println("steps to goal", steps, current_nodes)
}

func allMatch(lines []string, match func(string) bool) bool {
    for _, line := range lines {
        if !match(line) {
            return false
        }
    }
    return true
}
