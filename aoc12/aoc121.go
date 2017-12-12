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

func main() {
	fmt.Println("Hello")
	dat, err := ioutil.ReadFile("input.txt")
	check(err)

	programs := map[string][]string{"0": []string{}}
	lines := strings.Split(string(dat), "\n")

	for _, line := range lines {
		fmt.Println(">>>" + line)
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

	fmt.Printf("Nodes %d, connectsTo 0 = %d", len(programs), connectsTo("0", programs, make(map[string]bool)))

	// s := map[int]bool{5: true, 2: true}
	// _, ok := s[6] // check for existence
	// s[8] = true // add element
	// delete(s, 2) // remove element

	// s_union := map[int]bool{}
	// for k, _ := range s1{
	//     s_union[k] = true
	// }
	// for k, _ := range s2{
	//     s_union[k] = true
	// }
	// Intersection

	// s_intersection := map[int]bool{}
	// for k,_ := range s1 {
	//   if s2[k] {
	//     s_intersection[k] = true
	//   }
	// }

}
