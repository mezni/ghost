package main

import (
	"log"

	"github.com/mezni/generator/dbclient"
	"github.com/mezni/generator/service"
)

var appName = "generator"

func main() {
	log.Println("Starting " + appName)
	initializeBoltClient()
	service.StartWebServer("6777")
}

func initializeBoltClient() {
	log.Println("Init DB")
	service.DBClient = &dbclient.BoltClient{}
	service.DBClient.InitDB("generator.db")
	service.DBClient.Seed()

}
