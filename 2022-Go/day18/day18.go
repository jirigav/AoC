package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func str(x, y, z int) string {
	return fmt.Sprint(x, y, z)
}
func from_str(cube string) (int, int, int) {
	split := strings.Split(cube, " ")
	x, _ := strconv.Atoi(split[0])
	y, _ := strconv.Atoi(split[1])
	z, _ := strconv.Atoi(split[2])
	return x, y, z
}
func neighbours(x, y, z int) []string {
	return []string{str(x-1, y, z), str(x+1, y, z), str(x, y-1, z),
		str(x, y+1, z), str(x, y, z-1), str(x, y, z+1)}
}

// This is not a good way to check if the cube is inside of the lava droplet
func cond(x int) bool {
	return x > -5 && x < 25
}
func reach_0_0_0(x, y, z int, cubes map[string]bool, visited map[string]bool) bool {
	visited[str(x, y, z)] = true
	if x == 0 && y == 0 && z == 0 {
		return true
	}
	for _, n := range neighbours(x, y, z) {
		if !visited[n] && !cubes[n] {
			x, y, z := from_str(n)
			if cond(x) && cond(y) && cond(z) {

				if reach_0_0_0(x, y, z, cubes, visited) {
					return true
				}
			}

		}
	}
	return false
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	ns := make(map[string]int)
	cubes := make(map[string]bool)
	for fileScanner.Scan() {

		line := strings.Split(fileScanner.Text(), ",")
		x, _ := strconv.Atoi(line[0])
		y, _ := strconv.Atoi(line[1])
		z, _ := strconv.Atoi(line[2])
		fmt.Println(x, y, z)
		cubes[str(x, y, z)] = true
		for _, n := range neighbours(x, y, z) {
			ns[n] += 1
		}
	}
	readFile.Close()
	count := 0

	for n, v := range ns {
		if !cubes[n] {
			x, y, z := from_str(n)
			visited := make(map[string]bool)
			if reach_0_0_0(x, y, z, cubes, visited) {
				count += v
			}
		}
	}

	fmt.Println("Part1:", count)
	fmt.Println("Part2:")

}
