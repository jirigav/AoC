package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func item_priority(item rune) int {
	if item > 'Z' {
		return int(item-'a') + 1
	}
	return int(item-'A') + 27
}
func main() {

	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var rucksacks []string
	for fileScanner.Scan() {
		rucksacks = append(rucksacks, fileScanner.Text())
	}

	result1 := 0
	for i := 0; i < len(rucksacks); i++ {
		rucksack := rucksacks[i]
		for _, item := range rucksack[:len(rucksack)/2] {
			if strings.ContainsRune(rucksack[len(rucksack)/2:], item) {
				result1 += item_priority(item)
				break
			}
		}
	}
	result2 := 0
	for i := 0; i < len(rucksacks); i += 3 {
		for _, item := range rucksacks[i] {
			if strings.ContainsRune(rucksacks[i+1], item) &&
				strings.ContainsRune(rucksacks[i+2], item) {
				result2 += item_priority(item)
				break
			}
		}
	}
	readFile.Close()
	fmt.Println("Part1:", result1)
	fmt.Println("Part2:", result2)
}
