package main

import "testing"

func TestIsValidIp(t *testing.T) {
	ip := "0.34.82.53"

	result := Is_valid_ip(ip)
	if result == false {
		t.Errorf("Is_valid_ip(%s) == %t. Want %t", ip, false, true)
	}
}
