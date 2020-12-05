package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("05.txt")
	if err != nil {
		fmt.Println("Failed to read file")
		return
	}
	lines := strings.Split(string(content), "\n")
	seatIds := []int{}
	for _, line := range lines {
		seatIds = append(seatIds, getSeatID(line))
	}
	sort.Ints(seatIds)

	// Part one
	minID, maxID := seatIds[0], seatIds[len(seatIds)-1]
	fmt.Println("Highest seat ID:", maxID)

	// Part two
	i := 0
	for id := minID; id < maxID; id++ {
		if seatIds[i] != id {
			fmt.Println("My seat ID:", id)
			break
		}
		i++
	}
}

func getSeat(letters string, left int, right int) int {
	mid := left + (right-left)/2
	if len(letters) == 0 {
		return mid
	} else if letters[0] == 'F' || letters[0] == 'L' {
		return getSeat(letters[1:], left, mid-1)
	} else {
		return getSeat(letters[1:], mid+1, right)
	}
}

func getSeatID(letters string) int {
	seatRow := getSeat(letters[:7], 0, 127)
	seatCol := getSeat(letters[7:], 0, 7)
	return seatRow*8 + seatCol
}
