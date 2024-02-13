package main

import (
	"fmt"
	"math"
)

func RowSumOddNumbers(n int) int {
	return int(math.Pow(float64(n), 3.0))
}

func main() {
	result := RowSumOddNumbers(13)
	fmt.Println(result)
}
