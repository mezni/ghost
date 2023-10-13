package model

type Subsrciber struct {
	Id               string `json:"id"`
	SubsrciberID     string `json:"subsrciberID"`
	SubsrciberLastTS string `json:"timestamp"`
}

type ClientIpAddress struct {
	Id                string `json:"id"`
	ClientIpAddress   string `json:"subsrciberID"`
	ClientIpAddressTS string `json:"timestamp"`
}
