package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type node struct {
	name   string
	size   int
	parent *node
	childs []*node
}

func make_tree(lines []string) node {
	root := node{name: "/", size: 0, parent: nil, childs: make([]*node, 0)}
	current_node := &root
	for i := 0; i < len(lines); i++ {
		switch lines[i] {
		case "$ cd /":
			current_node = &root
		case "$ cd ..":
			current_node = current_node.parent
		case "$ ls":
			if len(current_node.childs) > 0 {
				i += len(current_node.childs)
				continue
			}

			for i++; i < len(lines) && lines[i][0] != '$'; i++ {
				split := strings.Split(lines[i], " ")
				size := 0
				if split[0] != "dir" {
					size, _ = strconv.Atoi(split[0])
				}
				newNode := node{name: split[1], size: size,
					parent: current_node, childs: make([]*node, 0)}
				current_node.childs = append(current_node.childs, &newNode)
			}
			i--
		default:
			split := strings.Split(lines[i], " ")
			for _, node := range current_node.childs {
				if split[2] == node.name {
					current_node = node
					break
				}
			}
		}
	}
	return root
}

func count_sizes(n *node) int {
	if len(n.childs) == 0 {
		return n.size
	}
	size := 0
	for _, c := range n.childs {
		size += count_sizes(c)
	}
	n.size = size
	return size
}

func part1(n *node) int {
	if len(n.childs) == 0 {
		return 0
	}
	val := 0
	if n.size <= 100000 {
		val += n.size
	}
	for _, c := range n.childs {
		val += part1(c)
	}

	return val
}

func part2(n *node, min int) int {
	if len(n.childs) == 0 {
		return 0
	}
	val := 0
	if n.size >= min {
		val = n.size
	}
	for _, c := range n.childs {
		v := part2(c, min)
		if v != 0 {
			if v < val {
				val = v
			}
		}
	}
	return val
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
		lines = append(lines, line)
	}

	root := make_tree(lines)
	count_sizes(&root)
	fmt.Println("Part1:", part1(&root))
	fmt.Println("Part2:", part2(&root, 30000000-(70000000-root.size)))
}
