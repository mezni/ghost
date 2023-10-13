package main

import (
	"fmt"
	"log"

	"github.com/mezni/generator/dbclient"
	"github.com/mezni/generator/service"
)

var appName = "generator"

func main() {
	fmt.Printf("Starting %v\n", appName)
	initializeBoltClient()
	service.StartWebServer("6777")
}

func initializeBoltClient() {
	log.Println("Init ")
	service.DBClient = &dbclient.BoltClient{}
	service.DBClient.OpenBoltDb()
	service.DBClient.Seed()
}
