package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func HighAndLow(in string) string {
	strSlice := strings.Fields(in)

	intSlice := make([]int, 0, len(strSlice))
	for _, str := range strSlice {
		convertedInt, err := strconv.Atoi(str)
		if err != nil {
			return ""
		}
		intSlice = append(intSlice, convertedInt)
	}
	sort.Ints(intSlice)

	fmt.Println(intSlice)

	return fmt.Sprintf("%d %d", intSlice[len(intSlice)-1], intSlice[0])
}

func main() {
	result := HighAndLow("1 2 3")
	fmt.Println(result)
}
