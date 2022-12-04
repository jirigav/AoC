package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func get_range(s string) (int, int) {
	str_range := strings.Split(s, "-")
	a, _ := strconv.Atoi(str_range[0])
	b, _ := strconv.Atoi(str_range[1])
	return a, b
}

func main() {

	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	result1 := 0
	result2 := 0
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), ",")
		l1, r1 := get_range(line[0])
		l2, r2 := get_range(line[1])

		if (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1) {
			result1++
		}

		if (l1 <= l2 && r1 >= l2) || (l2 <= l1 && r2 >= l1) {
			result2++
		}
	}
	fmt.Println("Part1:", result1)
	fmt.Println("Part2:", result2)
}
