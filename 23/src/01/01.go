package main

import (
	f "01/file"
	"fmt"
	"strconv"
	"strings"
)
func match(sub string) int {
    digit := 0
    switch {
    case strings.Contains(sub, "1") || strings.Contains(sub, "one"):
        digit = 1
    case strings.Contains(sub, "2") || strings.Contains(sub, "two"):
        digit = 2
    case strings.Contains(sub, "3") || strings.Contains(sub, "three"):
        digit = 3
    case strings.Contains(sub, "4") || strings.Contains(sub, "four"):
        digit = 4
    case strings.Contains(sub, "5") || strings.Contains(sub, "five"):
        digit = 5
    case strings.Contains(sub, "6") || strings.Contains(sub, "six"):
        digit = 6
    case strings.Contains(sub, "7") || strings.Contains(sub, "seven"):
        digit = 7
    case strings.Contains(sub, "8") || strings.Contains(sub, "eight"):
        digit = 8
    case strings.Contains(sub, "9") || strings.Contains(sub, "nine"):
        digit = 9
    }
    return digit
}

func main() {
	strings_ := f.FileToStrings("23", "01", false)
    total := 0
	for i := range strings_ {

		s := strings_[i]
		if s == "" {
			break
		}
		firstdigit := 0
		for j := range s {
			sub := s[:j+1]
            firstdigit = match(sub)
            if firstdigit != 0 {
                break
            }
		}

		lastdigit := 0
		for j := range s {
			sub := s[len(s)-j-1 :]
            lastdigit = match(sub)
            if lastdigit != 0 {
                break
            }
		}

        test := fmt.Sprintf("%d%d", firstdigit, lastdigit)
        digits, err := strconv.Atoi(test);
        
        if err != nil {fmt.Println("error", err); return} 
        
        total += digits

	}
    fmt.Println("total for part two: ", total)
}
