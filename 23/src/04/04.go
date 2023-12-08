package main

import (
	f "04/file"
	"fmt"
	"strings"
)

func main() {
	input_strings := f.FileToStrings("23", "04", false)
    games := make([]int, len(input_strings))
    for i := range games {
		games[i] = 1
	}
    total := 0
    for i := range input_strings {
        sections := strings.Split(input_strings[i], ": ")
        card_title := sections[0]
        parts := strings.Split(sections[1], " | ")
        winning := strings.Split(parts[0], " ")
        numbers := strings.Split(parts[1], " ")

        score := 0
        matches := 0

        
        for n_i := range numbers {
            if numbers[n_i] == "" {
                continue
            }

            val := numbers[n_i]
            filtered := filter(winning, func(n string) bool {
                if n == val {
                    return true
                }
                return false
            })
            if len(filtered) > 0 {
                matches += 1
                if score == 0 {
                    score = 1
                } else {
                    score = score * 2
                }
            }
        }

        fmt.Println("card", card_title, "winning", winning, "numbers", numbers, "score", score)
        total += score

        if score > 0 {
            for s := i+1; s <= i + matches ; s++ {
                if s < len(games) {
                    games[s] = games[s] + games[i]
                    // fmt.Println("matches", matches, "adding", games[i], "to game i" , s, "new value for game:", games[s])
                }
            }
        }
        
    }
    sum := 0
    for _, number := range games {
        sum += number
    }

    fmt.Println("cards:", sum)
    fmt.Println("total is:", total)
}

func filter(numbers []string, condition func(string) bool) []string {
    result := []string{}
    for _, number := range numbers {
        if condition(number) {
            result = append(result, number)
        }
    }
    return result
}
