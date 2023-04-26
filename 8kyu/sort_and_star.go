package main

import (
	"fmt"
	"sort"
	"strings"
)

func TwoSort(arr []string) string {
	sort.Strings(arr)
	var str string
	for idx, char := range arr[0] {
		str += string(char)
		if idx < len(arr[0])-1 {
			str += "***"
		}
	}
	return str
}

func TwoSortBestPractice(arr []string) string {
	sort.Strings(arr)
	chars := strings.Split(arr[0], "")
	return strings.Join(chars, "***")
}

func main() {
	result := TwoSortBestPractice([]string{"bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"})
	fmt.Println(result)
}
