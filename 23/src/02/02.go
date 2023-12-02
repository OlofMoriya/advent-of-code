package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	f "02/file"
)

func main() {
	input_strings := f.FileToStrings("23", "02", false)
    possible := 0
    total_power := 0
    for i := range input_strings {
        if input_strings[i] == "" { continue }
        
        game, good := check_row(input_strings[i])
//        println("game: ", game, "possible: ", good)
        if good{
            possible += game
        }

        game, max := check_row_min(input_strings[i])
        total_power += (max.Red * max.Blue * max.Green) 
    }

    fmt.Println("possible: ", possible)
    fmt.Println("power: ", total_power)
}

type Set struct {
    Red     int;
    Blue    int;
    Green   int;
}

func check_row_min(line string) (int, Set) {
    fmt.Println(line)
    red := 0
    blue := 0
    green := 0

    values := strings.Split(line, ":");
    game_number_string := strings.Split(values[0], " ")
    game_number, err := strconv.Atoi(game_number_string[1])
    if err != nil { 
        fmt.Println("error parsing game number: ", err)
    }
    sets := strings.Split(values[1], ";")
    for set_i := range sets {
        set_string := sets[set_i];
        
        // fmt.Println("set: ", set_string)
        set := parse_set(set_string)

        fmt.Println("parsed set:", set)

        red = max(red, set.Red)
        blue = max(blue, set.Blue)
        green = max(green, set.Green)
    }

    return game_number, Set{Red: red, Green: green, Blue: blue}
}
func check_row(line string) (int, bool) {
    fmt.Println(line)
    possible := true
    values := strings.Split(line, ":");
    game_number_string := strings.Split(values[0], " ")
    game_number, err := strconv.Atoi(game_number_string[1])
    if err != nil { 
        fmt.Println("error parsing game number: ", err)
    }
    sets := strings.Split(values[1], ";")
    for set_i := range sets {
        set_string := sets[set_i];
        
        // fmt.Println("set: ", set_string)
        set := parse_set(set_string)

        fmt.Println("parsed set:", set)

        if set.Red > 12 || set.Green > 13 || set.Blue > 14 {
            possible = false
            break
        }
    }

    return game_number, possible
}

func parse_set(set_string string) Set {
    colors := strings.Split(set_string, ",")
    red := 0
    green := 0
    blue := 0

        for color_i := range colors {
            parts := strings.Split(colors[color_i], " ")
            //fmt.Println("number: ", parts[1], "color:", parts[2])
            count, err := strconv.Atoi(parts[1])
            if err != nil {
                fmt.Println("error parsing number", err)
                os.Exit(1)
            }
            switch {
                case strings.Contains(colors[color_i], "green"):
                    green += count
                case strings.Contains(colors[color_i], "red"):
                    red += count
                case strings.Contains(colors[color_i], "blue"):
                    blue += count
            }
        }
        return Set{Red: red, Green: green, Blue: blue}
}
