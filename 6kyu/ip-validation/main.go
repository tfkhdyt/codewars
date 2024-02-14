package main

import (
	"net"
)

func Is_valid_ip(ip string) bool {
	if res := net.ParseIP(ip); res == nil {
		return false
	}
	return true
}
