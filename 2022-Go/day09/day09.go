package main

import (
	"bufio"
	"fmt"
	"math"
	"math/cmplx"
	"os"
	"strconv"
	"strings"
)

func step(h complex128, t complex128) complex128 {
	step := h - t
	if math.Abs(real(step)) > 1 {
		step -= complex(real(step)/2, 0)
	}
	if math.Abs(imag(step)) > 1 {
		step -= complex(0, imag(step)/2)
	}
	return step
}

func tail_visited_count(directions []string, steps []int, rope_len int) int {
	v := map[string]complex128{"L": -1 + 0i, "R": 1 + 0i, "U": 0 - 1i, "D": 0 + 1i}

	rope := make([]complex128, rope_len)
	visited := make(map[complex128]struct{})
	var member struct{}
	visited[rope[rope_len-1]] = member

	for i := 0; i < len(steps); i++ {
		for j := 0; j < steps[i]; j++ {
			rope[0] += v[directions[i]]
			for k := 1; k < rope_len; k++ {
				if cmplx.Abs(rope[k]-rope[k-1]) >= 2 {
					rope[k] += step(rope[k-1], rope[k])
				}
			}
			visited[rope[rope_len-1]] = member
		}
	}
	return len(visited)
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var directions []string
	var steps []int
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " ")
		directions = append(directions, line[0])
		step, _ := strconv.Atoi(line[1])
		steps = append(steps, step)
	}

	fmt.Println("Part1:", tail_visited_count(directions, steps, 2))
	fmt.Println("Part2:", tail_visited_count(directions, steps, 10))
}
