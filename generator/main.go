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
	service.DBClient.InitDB("generator.db")
	service.DBClient.Seed()
	// v,_:=service.DBClient.GetSubscriberByKey("3")
	// fmt.Printf("The answer is: %s\n", v)
	// v,_:=service.DBClient.GetClientIpByKey("3")
	// fmt.Printf("The answer is: %s\n", v)
}
