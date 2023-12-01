package main

import "fmt"
import f "01/file"

func main() {
	strings := f.FileToStrings("23", "01", true)
	nums := f.FileToInts("23", "01", true)
    fmt.Println(nums)
    fmt.Println(strings)
}
