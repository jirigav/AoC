package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func copy_2d[A any](s [][]A) [][]A {
	var result [][]A
	for _, sl := range s {
		temp := make([]A, len(sl))
		copy(temp, sl)
		result = append(result, temp)
	}
	return result
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func add_sand(grid [][]rune, x, y int) (bool, int, int) {
	for y++; ; y++ {
		if y >= len(grid) {
			return false, 0, 0
		}
		if grid[y][x] == ' ' {
			continue
		}
		if grid[y][x-1] == ' ' {
			x = x - 1
		} else if grid[y][x+1] == ' ' {
			x = x + 1
		} else {
			grid[y-1][x] = 'o'
			return true, x, y - 1
		}
	}
}

func part1(grid [][]rune, x, y int) int {
	b := true
	i := 0
	for ; b; i++ {
		b, _, _ = add_sand(grid, x, y)
	}
	return i - 1
}

func part2(grid [][]rune, x, y int) int {
	i := 1
	for ; ; i++ {
		_, xx, yy := add_sand(grid, x, y)

		if xx == x && yy == y {
			break
		}
	}
	return i
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	max_y := 0
	min_x := 999
	max_x := 0

	var xss [][]int
	var yss [][]int

	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " -> ")
		var xs []int
		var ys []int
		for _, coordinates := range line {
			c := strings.Split(coordinates, ",")

			x, _ := strconv.Atoi(c[0])
			y, _ := strconv.Atoi(c[1])

			max_y = max(max_y, y)
			max_x = max(max_x, x)
			min_x = min(min_x, x)

			xs = append(xs, x)
			ys = append(ys, y)
		}

		xss = append(xss, xs)
		yss = append(yss, ys)
	}
	readFile.Close()

	a := (min_x+max_x)/2 - max_y
	var grid [][]rune
	for i := 0; i < max_y+5; i++ {
		var row []rune
		for j := 0; j < (max_x - min_x + 2*max_y); j++ {
			row = append(row, ' ')
		}
		grid = append(grid, row)
	}

	for k := 0; k < len(xss); k++ {
		for i := 1; i < len(xss[k]); i++ {
			for x := min(xss[k][i-1], xss[k][i]); x <= max(xss[k][i-1], xss[k][i]); x++ {
				for y := min(yss[k][i-1], yss[k][i]); y <= max(yss[k][i-1], yss[k][i]); y++ {
					grid[y][x-a] = '#'
				}
			}
		}
	}

	grid2 := copy_2d(grid)

	for x := 0; x < len(grid2[0]); x++ {
		grid2[max_y+2][x] = '#'
	}

	fmt.Println("Part1:", part1(grid, 500-a, 0))
	fmt.Println("Part2:", part2(grid2, 500-a, 0))

}
