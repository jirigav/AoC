package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func str_to_int(s string) int {
	i, _ := strconv.Atoi(s)
	return i
}

func is_numeric(s string) bool {
	_, err := strconv.ParseInt(s, 10, 64)
	return err == nil
}

func items(s string) []string {
	depth_map := map[byte]int{'[': 1, ']': -1}
	var result []string
	depth := 0
	item := ""
	for i := 1; i < len(s)-1; i++ {
		if s[i] == ',' && depth == 0 {
			result = append(result, item)
			item = ""
			continue
		}
		item += string(s[i])
		depth += depth_map[s[i]]
	}
	if item != "" {
		result = append(result, item)
	}
	return result
}

func compare(first, second string) int {
	if is_numeric(first) && is_numeric(second) {
		return str_to_int(second) - str_to_int(first)
	}
	if is_numeric(first) {
		return compare("["+first+"]", second)
	}
	if is_numeric(second) {
		return compare(first, "["+second+"]")
	}

	items1 := items(first)
	items2 := items(second)
	min := len(items2)
	if len(items1) < len(items2) {
		min = len(items1)
	}
	for i := 0; i < min; i++ {
		c := compare(items1[i], items2[i])
		if c != 0 {
			return c
		}
	}
	return len(items2) - len(items1)
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
		line := fileScanner.Text()
		if line != "" {
			lines = append(lines, line)
		}
	}
	readFile.Close()

	result1 := 0
	for i := 0; i < len(lines); i += 2 {
		if compare(lines[i], lines[i+1]) >= 0 {
			result1 += (i + 2) / 2
		}
	}

	packet1 := "[[2]]"
	packet2 := "[[6]]"
	pos1 := 1
	pos2 := 1

	for _, line := range lines {
		if compare(line, packet1) >= 0 {
			pos1 += 1
		}
		if compare(line, packet2) >= 0 {
			pos2 += 1
		}
	}
	if compare(packet1, packet2) >= 0 {
		pos2 += 1
	} else {
		pos1 += 1
	}

	fmt.Println("Part1:", result1)
	fmt.Println("Part2:", pos1*pos2)

}
