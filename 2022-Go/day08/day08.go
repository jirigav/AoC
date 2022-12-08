package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func score_direction(grid [][]int, x, y, start, stop, step int, row bool) (int, bool) {
	direction_score := 0
	for i := start; step*i < step*stop; i += step {
		direction_score++
		if (row && grid[x][y] <= grid[i][y]) || (!row && grid[x][y] <= grid[x][i]) {
			return direction_score, false
		}
	}
	return direction_score, true
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var grid [][]int
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), "")
		var grid_row []int
		for _, n := range line {
			n_int, _ := strconv.Atoi(n)
			grid_row = append(grid_row, n_int)
		}
		grid = append(grid, grid_row)
	}

	visible_count := 0
	max_score := 0
	l := len(grid)
	for i := 0; i < l; i++ {
		for j := 0; j < l; j++ {

			d1, e1 := score_direction(grid, i, j, i+1, l, 1, true)
			d2, e2 := score_direction(grid, i, j, i-1, -1, -1, true)
			d3, e3 := score_direction(grid, i, j, j+1, l, 1, false)
			d4, e4 := score_direction(grid, i, j, j-1, -1, -1, false)

			if e1 || e2 || e3 || e4 {
				visible_count++
			}
			score := d1 * d2 * d3 * d4
			if score > max_score {
				max_score = score
			}
		}
	}
	fmt.Println("Part1:", visible_count)
	fmt.Println("Part2:", max_score)
}
