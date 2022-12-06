package main

import (
	"fmt"
	"os"
)

func string_to_set(str string) map[rune]struct{} {
	set := make(map[rune]struct{})
	var member struct{}
	for _, b := range str {
		set[b] = member
	}
	return set
}

func distinct_runes_endindex(str string, n int) int {
	for i := 0; i < len(str)-n; i++ {
		if len(string_to_set(str[i:i+n])) == n {
			return i + n
		}
	}
	return -1
}

func main() {
	input, err := os.ReadFile("input")
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println("Part1:", distinct_runes_endindex(string(input), 4))
	fmt.Println("Part2:", distinct_runes_endindex(string(input), 14))
}
