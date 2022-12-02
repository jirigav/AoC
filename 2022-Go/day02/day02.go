package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {

	var payoff1 = map[string]int{
		"A X": 4,
		"A Y": 8,
		"A Z": 3,
		"B X": 1,
		"B Y": 5,
		"B Z": 9,
		"C X": 7,
		"C Y": 2,
		"C Z": 6,
	}

	var payoff2 = map[string]int{
		"A X": 3,
		"A Y": 4,
		"A Z": 8,
		"B X": 1,
		"B Y": 5,
		"B Z": 9,
		"C X": 2,
		"C Y": 6,
		"C Z": 7,
	}

	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	score1 := 0
	score2 := 0
	for fileScanner.Scan() {
		strat := fileScanner.Text()
		score1 += payoff1[strat]
		score2 += payoff2[strat]
	}
	readFile.Close()
	fmt.Println("Part1:", score1)
	fmt.Println("Part2:", score2)
}
