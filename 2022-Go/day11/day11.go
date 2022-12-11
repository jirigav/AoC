package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type monkey struct {
	items      []int
	operation  []string
	test       int
	case_true  int
	case_false int
	inspected  int
}

func last_num(line string) int {
	line_split := strings.Split(line, " ")
	n, _ := strconv.Atoi(line_split[len(line_split)-1])
	return n
}

func create_monkey(lines []string) monkey {
	items_str := strings.Split(strings.Split(lines[1], ":")[1], ",")
	var items []int
	for _, i := range items_str {
		it, _ := strconv.Atoi(i[1:])
		items = append(items, it)
	}
	return monkey{items: items, operation: strings.Split(lines[2], " ")[5:],
		test: last_num(lines[3]), case_true: last_num(lines[4]),
		case_false: last_num(lines[5]), inspected: 0}
}

func worry_num(x string, worry int) int {
	if x == "old" {
		return worry
	}
	y, _ := strconv.Atoi(x)
	return y
}

func op(operation []string, worry int, m int) int {
	a := worry_num(operation[0], worry)
	b := worry_num(operation[2], worry)

	if operation[1] == "+" {
		return (a + b) % m
	} else {
		return (a * b) % m
	}
}

func round(monkeys []monkey, m int, round1 bool) {
	for i := 0; i < len(monkeys); i++ {
		for _, it := range monkeys[i].items {
			monkeys[i].inspected++
			worry := it
			worry = op(monkeys[i].operation, worry, m)
			if round1 {
				worry /= 3
			}
			if worry%monkeys[i].test == 0 {
				monkeys[monkeys[i].case_true].items = append(monkeys[monkeys[i].case_true].items, worry)
			} else {
				monkeys[monkeys[i].case_false].items = append(monkeys[monkeys[i].case_false].items, worry)
			}
		}
		monkeys[i].items = make([]int, 0)
	}
}

func comp_result(monkeys []monkey) int {
	var inspections []int
	for i := 0; i < len(monkeys); i++ {
		inspections = append(inspections, monkeys[i].inspected)
	}
	sort.Sort(sort.IntSlice(inspections))

	return inspections[len(inspections)-1] * inspections[len(inspections)-2]
}

func part1(monkeys []monkey, m int) int {
	for i := 0; i < 20; i++ {
		round(monkeys, m, true)
	}
	return comp_result(monkeys)
}

func part2(monkeys []monkey, m int) int {
	for i := 0; i < 10000; i++ {
		round(monkeys, m, false)
	}
	return comp_result(monkeys)
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var lines []string
	for fileScanner.Scan() {
		lines = append(lines, fileScanner.Text())
	}
	var monkeys []monkey
	m := 1
	for i := 0; i < len(lines); i += 7 {
		monkey := create_monkey(lines[i:])
		m *= monkey.test
		monkeys = append(monkeys, monkey)
	}

	monkeys2 := make([]monkey, len(monkeys))
	copy(monkeys2, monkeys)
	fmt.Println("Part1:", part1(monkeys, m))
	fmt.Println("Part2:", part2(monkeys2, m))
}
