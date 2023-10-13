package main

import (
        "fmt"
        "github.com/mezni/generator/service"        
)

var appName = "generator"

func main() {
        fmt.Printf("Starting %v\n", appName)
        service.StartWebServer("6777")           
}
