package main

import "fmt"

func Digitize(n int) []int {
	if n == 0 {
		return []int{0}
	}
	slc := []int{}
	for n > 0 {
		slc = append(slc, n%10)
		n /= 10
	}
	return slc
}

func main() {
	result := Digitize(35231)
	fmt.Println(result)
}
