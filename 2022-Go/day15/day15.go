package main

import (
	"bufio"
	"fmt"
	"os"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func part1(xs, ys, ds []int, b int) int {
	min_x := 0
	max_x := 0
	for i := 0; i < len(xs); i++ {
		if min_x > (xs[i] - ds[i]) {
			min_x = xs[i] - ds[i]
		}
		if max_x < (xs[i] + ds[i]) {
			max_x = xs[i] + ds[i]
		}
	}
	count := -b
	y := 2000000
	for x := min_x; x <= max_x; x++ {
		for i := 0; i < len(xs); i++ {
			if abs(xs[i]-x)+abs(ys[i]-y) <= ds[i] {
				count++
				break
			}
		}
	}
	return count
}

func part2(xs, ys, ds []int) int {
	for x := 0; x <= 4000000; x++ {
		for y := 0; y <= 4000000; y++ {
			found := false
			for i := 0; i < len(xs); i++ {
				if abs(xs[i]-x)+abs(ys[i]-y) <= ds[i] {
					y += ds[i] - (abs(xs[i]-x) + abs(ys[i]-y))
					found = true
					break
				}
			}
			if !found {
				return x*4000000 + y
			}

		}
	}
	return 0
}

func main() {
	readFile, err := os.Open("input")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var xs []int
	var ys []int
	var ds []int
	b := 0
	bxs := make(map[int]bool)
	for fileScanner.Scan() {
		var sx, sy, bx, by int
		fmt.Sscanf(fileScanner.Text(),
			"Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d",
			&sx, &sy, &bx, &by)
		if by == 2000000 && !bxs[bx] {
			bxs[bx] = true
			b++
		}
		d := abs(sx-bx) + abs(sy-by)
		xs = append(xs, sx)
		ys = append(ys, sy)
		ds = append(ds, d)
	}
	readFile.Close()

	fmt.Println("Part1:", part1(xs, ys, ds, b))
	fmt.Println("Part2:", part2(xs, ys, ds))

}
