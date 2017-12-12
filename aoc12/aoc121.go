package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func connectsTo(node string, programs map[string][]string, seen map[string]bool) int {
	elem, ok := programs[node]
	sum := 1
	seen[node] = true
	delete(programs, node)

	if ok {
		for _, child := range elem {
			if !seen[child] {
				fmt.Printf("Node %s is connected to %s\n", node, child)
				sum += connectsTo(child, programs, seen)
			}
		}
	}

	return sum
}

func first_key(programs map[string][]string) string {
	for key, _ := range programs {
		return key
	}
	panic("shouldn't get here")
}

func main() {
	fmt.Println("Hello")
	dat, err := ioutil.ReadFile("input.txt")
	check(err)

	programs := map[string][]string{"0": []string{}}
	lines := strings.Split(string(dat), "\n")

	for _, line := range lines {
		// fmt.Println(">>>" + line)
		parts := strings.Split(line, " <-> ")
		root, elements := parts[0], parts[1]
		connections := strings.Split(elements, ", ")
		// fmt.Printf("%d: %s ==> %s\n", i, root, strings.Join(connections, "/"))
		//        for _, connection := range connections {
		elem, ok := programs[root]
		if ok {
			programs[root] = append(elem, connections...)
		} else {
			programs[root] = connections
		}
		// }
	}

	seen := make(map[string]bool)
	groups := 0
	nodes := len(programs)
	rootCount := connectsTo("0", programs, seen)
	for len(seen) != nodes {
		key := first_key(programs)
		groups++
		fmt.Printf("%s connectsTo %d other nodes\n", key, connectsTo(key, programs, seen))
	}
	fmt.Printf("Nodes %d, and group 0 had %d\n", nodes, rootCount)
	fmt.Printf("Groups %d\n", groups)
}
