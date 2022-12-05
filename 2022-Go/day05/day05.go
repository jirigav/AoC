package main

import (
	"bufio"
	"fmt"
	"os"
)

func pop[A any](s []A, n int) (popped, original []A) {
	popped = s[len(s)-n:]
	original = s[:len(s)-n]
	return
}

func reverse[A any](s *[]A) {
	for i := 0; i < len(*s)/2; i++ {
		(*s)[i], (*s)[len(*s)-i-1] = (*s)[len(*s)-i-1], (*s)[i]
	}
}

func copy_2d[A any](s [][]A) [][]A {
	var result [][]A
	for _, sl := range s {
		temp := make([]A, len(sl))
		copy(temp, sl)
		result = append(result, temp)
	}
	return result
}

func get_stacks(lines []string) [][]rune {
	stacks := make([][]rune, (len(lines[0])+2)/4)
	for i := len(lines) - 1; i >= 0; i-- {
		for j := 0; 1+4*j < len(lines[i]); j++ {
			if lines[i][1+4*j] != ' ' {
				stacks[j] = append(stacks[j], rune(lines[i][1+4*j]))
			}
		}
	}
	return stacks
}

func solve(stacks [][]rune, part int, lines []string) {

	for _, line := range lines {
		var a, f, t int
		fmt.Sscanf(line, "move %d from %d to %d", &a, &f, &t)
		var temp []rune
		temp, stacks[f-1] = pop(stacks[f-1], a)
		if part == 1 {
			reverse(&temp)
		}
		stacks[t-1] = append(stacks[t-1], temp...)
	}

	result := ""
	for _, stack := range stacks {
		result += string(stack[len(stack)-1])
	}
	fmt.Printf("Part%d: %s\n", part, result)
}

func main() {

	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var lines []string
	empty_line := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		if line == "" {
			empty_line = len(lines)
		}
		lines = append(lines, line)
	}

	stacks := get_stacks(lines[:empty_line-1])

	var stacks_copy [][]rune
	copy(stacks_copy, stacks)
	solve(copy_2d(stacks), 1, lines[empty_line+1:])
	solve(stacks, 2, lines[empty_line+1:])
}
