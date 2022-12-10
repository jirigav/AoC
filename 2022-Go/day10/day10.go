package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func one_cycle(pos, result, cycle *int, register int) {
	if *pos >= register && *pos < register+3 {
		print("#")
	} else {
		print(".")
	}
	*pos++
	if (*cycle)%40 == 0 {
		println()
		*pos = 1
	}

	if (*cycle-20)%40 == 0 {
		*result += *cycle * register
	}
	*cycle++
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	cycle := 1
	register := 1
	result := 0
	pos := 1
	fmt.Println("Part2:")
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " ")
		one_cycle(&pos, &result, &cycle, register)

		if line[0] == "addx" {
			one_cycle(&pos, &result, &cycle, register)
			n, _ := strconv.Atoi(line[1])
			register += n
		}
	}

	fmt.Println("Part1:", result)
}
