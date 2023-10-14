package model

type Subscriber struct {
	Id              string `json:"id"`
	SubscriberID    string `json:"subsrciberID"`
	SubscriberEndTS string `json:"subscriberEndTS"`
}

type ClientIp struct {
	Id      string `json:"id"`
	IpID    string `json:"ipID"`
	IpEndTS string `json:"ipEndTS"`
}
