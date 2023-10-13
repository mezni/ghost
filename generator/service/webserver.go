package service

import (
	"log"
	"net/http"
	"time"
)

func StartWebServer(port string) {

	srv := http.Server{
		Addr:         ":" + port,
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
		IdleTimeout:  120 * time.Second,
	}

	r := NewRouter()
	http.Handle("/", r)

	log.Println("Starting HTTP service at " + port)
	err := srv.ListenAndServe()
	if err != nil {
		log.Println("An error occured starting HTTP listener at port " + port)
		log.Println("Error: " + err.Error())
	}

}
