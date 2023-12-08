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


func filter(items []string, condition func(string) bool) []string {
    result := []string{}
    for _, item := range items {
        if condition(item) {
            result = append(result, item)
        }
    }
    return result
}

func first() {
	lines := FileToStrings("23", "06", false)

    times_parts := strings.Split(lines[0], " ")
    times_parts = filter(times_parts, func (input string) bool {
        return input != ""
    })[1:]
    fmt.Println("times", times_parts)
      
    distance_parts := strings.Split(lines[1], " ")
    distance_parts = filter(distance_parts, func (input string) bool {
        return input != ""
    })[1:]
    fmt.Println("distance", distance_parts)

    ways_to_beat_prod := 1

    for i := range times_parts {
        race_time, _ := strconv.Atoi(times_parts[i])
        record_distance, _ := strconv.Atoi(distance_parts[i])
        
        ways_to_beat := 0
        for button_hold := 1; button_hold < race_time; button_hold++ {
            speed := button_hold
            travel_time := race_time - button_hold

            distance := speed * travel_time
            if distance > record_distance {
                ways_to_beat++
            }
        }
        ways_to_beat_prod = ways_to_beat_prod * ways_to_beat
    }
    
    fmt.Println("prod", ways_to_beat_prod)
}
func main() {
	lines := FileToStrings("23", "06", false)

    times_parts := strings.Split(lines[0], " ")
    times_parts = filter(times_parts, func (input string) bool {
        return input != ""
    })[1:]
    time := strings.Join(times_parts, "")
    fmt.Println("times", time)
      
    distance_parts := strings.Split(lines[1], " ")
    distance_parts = filter(distance_parts, func (input string) bool {
        return input != ""
    })[1:]
    distance := strings.Join(distance_parts, "")
    fmt.Println("distance", distance)

    race_time, _ := strconv.Atoi(time)
    record_distance, _ := strconv.Atoi(distance)
    
    ways_to_beat := 0
    for button_hold := 1; button_hold < race_time; button_hold++ {
        speed := button_hold
        travel_time := race_time - button_hold

        distance := speed * travel_time
        if distance > record_distance {
            ways_to_beat++
        }
    }
    
    fmt.Println("prod", ways_to_beat)
}
