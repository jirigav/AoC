package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func findSE(grid [][]rune) ([]int, []int) {
	var s []int
	var e []int
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[i]); j++ {
			if grid[i][j] == 'S' {
				s = []int{i, j, 0}
				grid[i][j] = 'a'
			}
			if grid[i][j] == 'E' {
				e = []int{i, j}
				grid[i][j] = 'z'
			}
		}
	}
	return s, e
}

func neighbours(grid [][]rune, current []int, visited [][]bool) [][]int {
	n := [][]int{{1, 0, 1}, {0, 1, 1}, {-1, 0, 1}, {0, -1, 1}}
	var result [][]int
	for i := 0; i < len(n); i++ {
		for j := 0; j < len(n[i]); j++ {
			n[i][j] += current[j]
		}
		if n[i][0] >= 0 && n[i][0] < len(grid) &&
			n[i][1] >= 0 && n[i][1] < len(grid[0]) &&
			(grid[n[i][0]][n[i][1]]-grid[current[0]][current[1]]) < 2 &&
			!visited[n[i][0]][n[i][1]] {

			result = append(result, n[i])
			visited[n[i][0]][n[i][1]] = true
		}
	}
	return result
}

func bfs(grid [][]rune, queue [][]int, target []int) int {
	var visited [][]bool
	for i := 0; i < len(grid); i++ {
		visited = append(visited, make([]bool, len(grid[i])))
	}
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current[0] == target[0] && current[1] == target[1] {
			return current[2]
		}
		queue = append(queue, neighbours(grid, current, visited)...)
	}
	return -1
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var grid [][]rune
	queue := [][]int{}
	for fileScanner.Scan() {
		row := strings.Split(fileScanner.Text(), "")
		var r_row []rune
		for j, i := range row {
			r_row = append(r_row, rune(i[0]))
			if i[0] == 'a' {
				queue = append(queue, []int{len(grid), j, 0})
			}
		}
		grid = append(grid, r_row)
	}
	readFile.Close()

	s, e := findSE(grid)

	fmt.Println("Part1:", bfs(grid, [][]int{s}, e))
	fmt.Println("Part2:", bfs(grid, queue, e))

}
