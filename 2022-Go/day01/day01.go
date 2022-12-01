package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var totalCalories []int
	var calories int = 0
	var intCal int
	for fileScanner.Scan() {
		intCal, err = strconv.Atoi(fileScanner.Text())

		if err == nil {
			calories = calories + intCal
		} else {
			totalCalories = append(totalCalories, calories)
			calories = 0
		}
	}
	readFile.Close()
	sort.Sort(sort.IntSlice(totalCalories))
	fmt.Println("Part1:", totalCalories[len(totalCalories)-3])
	fmt.Println("Part2:", totalCalories[len(totalCalories)-1]+
		totalCalories[len(totalCalories)-2]+totalCalories[len(totalCalories)-3])

}
