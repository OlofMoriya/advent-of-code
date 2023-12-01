package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Policy struct {
    Min int
    Max int
    Letter string 
}

func validate_positions(password string, policy Policy) bool {
    match := 0
    if string(password[policy.Min - 1]) == policy.Letter {
        match += 1
    }
    if string(password[policy.Max - 1]) == policy.Letter {
        match += 1
    }
    return match == 1

}
func validate(password string, policy Policy) bool {
    letter_count := 0
    for index := range password {
        if string(password[index]) == policy.Letter {
            letter_count += 1
        }
    }
    return (letter_count >= policy.Min && letter_count <= policy.Max)
}
 
func parse_line(line string) (*Policy, *string, error) {
    sections := strings.Split(line, " ") 
    min_str, max_str, found := strings.Cut(sections[0], "-")
    if !found {
        return nil, nil, errors.New("no min and max")
    }
    min, err := strconv.Atoi(min_str)
    if err != nil {
        return nil, nil, errors.New("can't parse min")
    }
    max, err := strconv.Atoi(max_str)
    if err != nil {
        return nil, nil, errors.New("can't parse max")
    }
    letter := string(sections[1][0])
    password := sections[2]
    return &Policy{Min:min, Max: max, Letter: letter}, &password, nil
}

func main(){
    input, err := os.ReadFile("./20/input/20_02")
    if err != nil {
        fmt.Println(err)
    }
    string_input := string(input)
    lines := strings.Split(string_input, "\n")
    valid_count := 0
    valid_count_alt := 0
    for i := range lines {
        if lines[i] == "" {
            continue
        }
        policy, pass, err := parse_line(lines[i])
        if err != nil {
            fmt.Println("error in parsing line", err)
        }
        if validate(*pass, *policy) {
            fmt.Println("match", policy, *pass)
            valid_count += 1
        }
        if validate_positions(*pass, *policy) {
            fmt.Println("match pos", policy, *pass)
            valid_count_alt += 1
        }
    }
    fmt.Println("valid passwords", valid_count)
    fmt.Println("valid passwords pos", valid_count_alt)
}
