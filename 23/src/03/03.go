package main

import (
	f "03/file"
	"fmt"
	"strconv"

	// "strconv"
	"regexp"
)

func find_connected_gears(start_y int, start_x int, input_strings []string) []Gear {
    y_b := max(start_y-1, 0)
    y_e := min(start_y+1, len(input_strings)-1)
    x_b := max(start_x-1, 0)
    x_e := min(start_x+1, len(input_strings[start_y])-1)

    gears := []Gear{}
    
    
    for y := y_b; y <=y_e; y++ {
        for x := x_b; x <=x_e; x++ {
            c := string(input_strings[y][x])

            if c == "*" {
                gears = append(gears, Gear{Y: y, X: x, Numbers: []int{}})
            }
        }
        
    }
    return gears
}

func check_connected(start_y int, start_x int, input_strings []string) (bool, string) {
    y_b := max(start_y-1, 0)
    y_e := min(start_y+1, len(input_strings)-1)
    x_b := max(start_x-1, 0)
    x_e := min(start_x+1, len(input_strings[start_y])-1)
    // fmt.Println("-----") 
    // fmt.Println("y range for y_start:",start_y, y_b, y_e)
    // fmt.Println("x range for x_start:",start_x, x_b, x_e)

    is_connected := false
    engine_part := ""
    
    r, _ := regexp.Compile("[^0-9.]")
    
    Exit:
    for y := y_b; y <=y_e; y++ {
        for x := x_b; x <=x_e; x++ {
            c := string(input_strings[y][x])

            found := r.FindString(c)
            if found != "" {
                is_connected = true
                engine_part = found
                break Exit
            }
        }
        
    }
    return is_connected, engine_part
}

type Gear struct {
    X int
    Y int
    Numbers []int
}

func main() {
	input_strings := f.FileToStrings("23", "03", false)
    total := 0
    gears := []Gear{}
    for y := range input_strings {
        fmt.Println("cs: ", input_strings[y])
        current_number_string := ""
        current_number_connected := false
        current_engine_parts := []Gear{} 
        for x := range input_strings[y] {
            c := string(input_strings[y][x])
            found_digit, err := regexp.MatchString("[0-9]", c)
            if err != nil {
                fmt.Println("error in regexp", err)
            }
            if found_digit {
                current_number_string += c
                found_part, _ := check_connected(y, x, input_strings)
                current_number_connected = current_number_connected || found_part

                //Find the connected gears
                connected_gear_locations := find_connected_gears(y,x, input_strings)
                fmt.Println("following gears are connected to the current_number_string", current_number_string, ":", connected_gear_locations)

                for connected_gear_locations_i := range connected_gear_locations {
                    connected_gear_location := connected_gear_locations[connected_gear_locations_i]
                    current_engine_parts = connected_gear_locations
                    already_found_gear := false
                    for gear_i := range gears{
                        gear := gears[gear_i]
                        fmt.Println("checking if should add gear (y,x) == (y,x): ", gear.Y, gear.X, connected_gear_location.Y ,connected_gear_location.X )
                        if gear.Y == connected_gear_location.Y && gear.X == connected_gear_location.X {
                            fmt.Println("already in list")
                            already_found_gear = true
                        }
                    }
                    if !already_found_gear {
                        gears = append(gears, connected_gear_location)
                    }
                }
            } 
            if !found_digit || x == len(input_strings[y])-1{
                if current_number_string != "" {
                    //fmt.Println(current_number_string, current_number_connected)
                    num, err := strconv.Atoi(current_number_string)
                    if err != nil {
                        fmt.Println("x"+current_number_string+"x")
                        fmt.Println("error parsing", current_number_string, err)
                    }
                    if current_number_connected { 
                        for i := range current_engine_parts {
                            gear := find_gear(current_engine_parts[i], gears)
                            gear.Numbers = append(gear.Numbers, num)
                            
                            fmt.Println("setting number in gear", num)
                            fmt.Println("gear", gear)
                            fmt.Println("gears", gears)
                        }
                        fmt.Println("found digit", num)
                        total += num

                    }
                }
                current_engine_parts = []Gear{}
                current_number_string = ""
                current_number_connected = false
            }
        }
    }

    total_gears := 0
    for i := range gears {
        if len(gears[i].Numbers) > 1 {
            fmt.Println("numbers to add",gears[i].Numbers)
            prod := 1  
            for num_i := range gears[i].Numbers {
                prod = prod * gears[i].Numbers[num_i]
            }
            fmt.Println("product", prod)
            total_gears += prod
        }
    }

    fmt.Println("total:", total)
    fmt.Println("total gears:", total_gears)
}

func find_gear(locate Gear, gears []Gear) *Gear {
    for i := range gears {
        if gears[i].X == locate.X && gears[i].Y == locate.Y {
            return &gears[i]
        }
    }   
    return nil
}
