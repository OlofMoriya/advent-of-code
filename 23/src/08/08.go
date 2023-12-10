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

    //Third line and after is the direction translation
    for _,line := range lines[2:]{

        pattern := "(\\b[A-Z]+\\b) = \\((\\b[A-Z]+\\b), (\\b[A-Z]+\\b)\\)"

	    re := regexp.MustCompile(pattern)
	    match := re.FindStringSubmatch(line)

        //fmt.Println("match", match)
        maps[match[1]] = Map{Left: match[2], Right: match[3], Current: match[1]}
        fmt.Println("map added", maps[match[1]], "to:", match[1])
    }
    fmt.Println("dir", directions)
    //fmt.Println("maps", maps)

    //Find all start values
    for _,v := range maps {
        if v.Current[2] == 'A' {
            matches, steps := findStepsTo(maps, directions, v.Current, func(s string) bool { return s[2] == 'Z'}, func(s string) bool {return s == v.Current})
            fmt.Println("start value:",v.Current, "matches", matches, "steps", steps)
        }
    }
}

type Match struct {
    Step int
    Name string
    Direction byte
}

func findStepsTo(maps map[string]Map, directions string ,start string, collectMatch func(string) bool, endMatch func(string) bool) ([]Match, int){
    steps := 1
    collectMatches := []Match{}
    current := start
    first := true
    WHILE: 
    for !endMatch(current) || first {
        first = false
        for i,d := range directions {
            var next string
            current_map, _ := maps[current]
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
            if collectMatch(next) {
                fmt.Println("matched", current)
                hasMatchAlready := false
                nextDirectionIndex := i + 1
                if i + 1 == len(directions) {
                    nextDirectionIndex = 0
                }
                for _,m := range collectMatches {
                    if m.Name == next && m.Direction == directions[nextDirectionIndex] {
                        hasMatchAlready = true
                    }
                }
                collectMatches = append(collectMatches, Match{Step: steps, Name: next, Direction:directions[nextDirectionIndex]})
                if hasMatchAlready {
                    break WHILE
                }
            }
           // fmt.Println(current, string(d), " -> ", next)
           if endMatch(next) {
               fmt.Println("end matched", next)
               break WHILE
           }
           current = next
           steps++
        }
    }
    return collectMatches, steps
}
