package main

import "fmt"

func Between(a, b int) []int {
	result := make([]int, 0)
	for i := a; i <= b; i++ {
		result = append(result, i)
	}
	return result
}

func main() {
	result := Between(6, 9)
	fmt.Println(result)
}
