package main

import "fmt"

func SetAlarm(employed, vacation bool) bool {
	return employed && !vacation
}

func main() {
	result1 := SetAlarm(true, true)
	fmt.Println(result1)
	result2 := SetAlarm(false, true)
	fmt.Println(result2)
	result3 := SetAlarm(false, false)
	fmt.Println(result3)
	result4 := SetAlarm(true, false)
	fmt.Println(result4)
}
