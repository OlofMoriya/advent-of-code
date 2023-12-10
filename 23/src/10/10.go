package main

import (
	"fmt"
	"os"
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

type Step struct {
    X int
    Y int
    Distance int
    Start bool
    Character string
}

func inList(list []string, test string) bool {
    for _,v := range list {
        if v == test {
            return true
        }
    }
    return false
}

func theOther(a []int, b []int, test [] int) (int, int) {
    if test[0] == a[0] && test[1] == a[1] {
        return b[0], b[1]
    }
    return a[0], a[1]

}

func get_out(prev_x int, prev_y int, x int, y int, char string) (int, int) {
    switch char {
        case "F": 
           return theOther([]int{x+1, y}, []int{x, y+1}, []int{prev_x, prev_y}) 
        case "-": 
           return theOther([]int{x+1, y}, []int{x-1, y}, []int{prev_x, prev_y}) 
        case "7": 
           return theOther([]int{x, y + 1}, []int{x-1, y}, []int{prev_x, prev_y}) 
        case "|": 
           return theOther([]int{x, y + 1}, []int{x, y -1}, []int{prev_x, prev_y}) 
        case "L": 
            nx, ny := theOther([]int{x, y - 1}, []int{x + 1, y}, []int{prev_x, prev_y}) 
            fmt.Println("Hit L", nx, ny, "is next, come from")
            return nx,ny
        case "J": 
           return theOther([]int{x, y - 1}, []int{x - 1, y}, []int{prev_x, prev_y}) 
        default: 
           fmt.Println("error probably went wrong in the previous step from", prev_x, prev_y, "to", x, y)
           os.Exit(0)
    }

    return 0,0
}


func isConnected(current Step, next Step) bool {
    switch next.Y - current.Y {
        case -1:
            switch  next.X - current.X {
                case -1:
                    return false
                case 0: 
                    if inList([]string{"|", "7", "F"}, next.Character) {
                        return true
                    }
                    return false
                case 1:
                    return false
                default: 
                    fmt.Println("Error in isConnected X")
                    fmt.Println(current.X, current.Y, "checking next step", string(next.Character), next.X, next.Y, "diffs", current.X- next.X, current.Y - next.Y)
                    return false
            }
        case 0:
            switch  next.X - current.X {
                case -1:
                    if inList([]string{"-", "L", "F"}, next.Character) {
                        return true
                    }
                    return false
                case 0: 
                    return false
                case 1:
                    if inList([]string{"-", "J", "7"}, next.Character) {
                        return true
                    }
                    return false
                default: 
                    fmt.Println("Error in isConnected X")
                    fmt.Println(current.X, current.Y, "checking next step", string(next.Character), next.X, next.Y, "diffs", current.X- next.X, current.Y - next.Y)
                    return false
            }
        case 1:
            switch  next.X - current.X {
                case -1:
                    return false
                case 0: 
                    if inList([]string{"|", "L", "J"}, next.Character) {
                        return true
                    }
                    return false
                case 1:
                    return false
                default: 
                    fmt.Println("Error in isConnected X")
                    fmt.Println(current.X, current.Y, "checking next step", string(next.Character), next.X, next.Y, "diffs", current.X- next.X, current.Y - next.Y)
                    return false
            }
        default: 
            fmt.Println("Error in isConnected Y")
            fmt.Println(current.X, current.Y, "checking next step", string(next.Character), next.X, next.Y, "diffs", current.X- next.X, current.Y - next.Y)
            return false
    }
}

func main() {
	lines := FileToStrings("23", "10", false)

    fmt.Println(lines)

    //find S
    var startStep *Step

    // println("start", string(startStep.Character), "X", startStep.X, "Y", startStep.Y)
    // for _,c := range connected {
    //     println("connected", string(c.Character), "X", c.X, "Y", c.Y)
    // }
    
    Find_s:
    for y := range lines {
        for x := range lines[y] {
            if lines[y][x] == 'S' {
                startStep = &Step{X:x, Y:y, Distance: 0, Character: string(lines[y][x])}
                break Find_s
            }
        }
    }

    didLoop := false
    var prev_step *Step 
    current_step := *startStep
    fmt.Println("start/current", string(current_step.Character))
    steps := []Step{}
    steps = append(steps, *startStep)

    min_y := max(0, current_step.Y - 1)
    min_x := max(0, current_step.X - 1)
    max_y := min(current_step.Y + 1, len(lines) -1)
    max_x := min(current_step.X + 1, len(lines[current_step.Y])-1)
    // fmt.Println("from xmin", min_x, "to xmax", max_x)
    // fmt.Println("from ymin", min_y, "to ymay", max_y)
    var connection *Step
    FIND_S: 
    for yy := min_y; yy <= max_y; yy++ {
        for xx := min_x; xx <= max_x; xx++ {
            if xx != current_step.X && yy != current_step.Y {
                continue
            }
            // fmt.Println("x", xx, "y", yy)
            next := Step{Y: yy, X: xx, Distance: current_step.Distance + 1, Character: string(lines[yy][xx])}
            if isConnected(current_step, next) && (prev_step == nil || (next.X != prev_step.X || next.Y != prev_step.Y)){
                fmt.Println("found connected, moving to next", current_step, next)
                steps = append(steps, next)
                connection = &next
                prev_step = &Step{X:current_step.X, Y: current_step.Y, Distance: current_step.Distance, Character: current_step.Character}
                break FIND_S
            }
        }
    }
    x := connection.X
    y := connection.Y
    prev_x := startStep.X
    prev_y := startStep.Y
    step_count := 2
    for !didLoop {
        t_x := x 
        t_y := y 
        x, y = get_out(prev_x, prev_y, x, y, string(lines[y][x]))
        prev_x = t_x
        prev_y = t_y
        steps = append(steps, Step{X:x, Y:y, Character: string(lines[y][x]), Distance: step_count})
        step_count ++

        if lines[y][x] == 'S'{
            didLoop = true
        }
    }

    distance := (steps[len(steps)-1].Distance + 1) / 2
    fmt.Println("max dist", distance)

    fmt.Println()
    for _,s := range steps {

        fmt.Print(s.Character)
    }
    // fmt.Println()
    // for _,s := range steps {
    //
    //     fmt.Print(s.Distance)
    // }
    //walk S until back to S

    //check where half the distance was
}
