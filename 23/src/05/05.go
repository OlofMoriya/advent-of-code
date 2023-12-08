package main

import (
	f "05/file"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input_strings := f.FileToStrings("23", "05", false)

    seeds := []int{}
    seed_line := input_strings[0]
    seed_strings := strings.Split(seed_line, " ")[1:]
    for i:= 0; i<len(seed_strings); i+=2 {
        fmt.Println("create from:", seed_strings[i], ", count:", seed_strings[i+1])
        fmt.Print(".")
        seed_int, err := strconv.Atoi(seed_strings[i])
        if err != nil {
            fmt.Println("error in parsing seed: ", err)
        }
        seed_count, err := strconv.Atoi(seed_strings[i+1])
        if err != nil {
            fmt.Println("error in parsing seed: ", err)
        }
        for j:=seed_int;j < seed_int+seed_count; j++{
            seeds = append(seeds, j) 
        }
    }

    // fmt.Println("seeds: ", seeds)
    fmt.Println("seed count:", len(seeds))

    seed_soil_map, end_index := make_map(input_strings, 3)
    // fmt.Println("seed_soil_map", seed_soil_map[0])
    soil_fert_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("soil_fert_map", soil_fert_map[0])
    fer_water_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("fer_water_map", fer_water_map[0])
    water_light_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("water_light_map", water_light_map[0])
    light_temp_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("light_temp_map", light_temp_map[0])
    temp_hum_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("temp_hum_map", temp_hum_map[0])
    hum_loc_map, end_index := make_map(input_strings, end_index+2)
    // fmt.Println("hum_loc_map", hum_loc_map[0])

    min_loc := -1
    min_hum := 0
    min_temp := 0
    min_light := 0
    min_seed_loc := 0
    for _,seed := range seeds {
        soil := getValueOrDefault(seed_soil_map, seed) 
        // fmt.Println("soil from seed", soil, seed)
        fert := getValueOrDefault(soil_fert_map, soil) 
        // fmt.Println("fert from seed", fert)
        water := getValueOrDefault(fer_water_map, fert) 
        // fmt.Println("water from seed", water)
        light := getValueOrDefault(water_light_map, water) 
        // fmt.Println("light from seed", light)
        temp := getValueOrDefault(light_temp_map, light) 
        // fmt.Println("temp from seed", temp)
        hum := getValueOrDefault(temp_hum_map, temp) 
        // fmt.Println("hum from seed", hum)
        loc := getValueOrDefault(hum_loc_map, hum) 
        // fmt.Println("seed to loc", seed, loc)

        // fmt.Println(loc, hum, temp, light, water, fert, soil, seed)

        if loc < min_loc || min_loc == -1 {
            min_loc = loc
            min_seed_loc = seed
            min_hum = hum
            min_temp = temp
            min_light = light
        }

    }
    fmt.Println("min loc", min_loc, min_seed_loc)
    fmt.Println("hum", min_hum)
    fmt.Println("temp", min_temp)
    fmt.Println("light", min_light)
}

func getValueOrDefault(m []Map, key int) int {
    for _,v := range m {
        if key >= v.From && key <= v.To {
            return key + v.Diff
        }
    }
    return key
}

type Map struct {
    From int
    To int
    Diff int
}


func make_map(input_strings []string, start_index int) ([]Map, int) {
    source_dest_map_strings := []string{}
    end_index := start_index
    for line_i := start_index; line_i < len(input_strings); line_i ++ {
        if input_strings[line_i] == "" {
            end_index = line_i
            break
        }
        source_dest_map_strings = append(source_dest_map_strings, input_strings[line_i])
    }

    //fmt.Println("seed soil map lines", source_dest_map_strings)

    // source_desp_map := map[int]int{} 
    source_dest_list := []Map{}
    for _, source_dest := range source_dest_map_strings {
        parts := strings.Split(source_dest, " ")
        dest,err := strconv.Atoi(parts[0])
        if err != nil {
            fmt.Println("could not parse", parts[0])
        }
        source,err := strconv.Atoi(parts[1])
        if err != nil {
            fmt.Println("could not parse", parts[1])
        }
        count,err := strconv.Atoi(parts[2])
        if err != nil {
            fmt.Println("could not parse", parts[2])
        }
        
        // fmt.Println("found", dest, source, count)
        source_dest_list = append(source_dest_list, Map{From: source, To:source+count, Diff: dest-source})
        // for i := source; i < source + count; i++ {
        //     source_desp_map[i]=dest-source+i
        // }
    }
    return source_dest_list, end_index
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
