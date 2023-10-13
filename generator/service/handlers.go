package service

import (
	"net/http"

	"github.com/mezni/generator/dbclient"
)

var DBClient dbclient.IBoltClient

func GetEvents(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.Write([]byte("{\"result\":\"OK\"}"))
}
