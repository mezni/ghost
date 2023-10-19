package model

type InRequest struct {
	IntervalStartDate string `json:"intervalStartDate"`
	IntervalMinutes   int    `json:"intervalMinutes"`
	TrxCount          int    `json:"trxCount"`
}

type Event struct {
	Timestamp           string `json:"timestamp"`
	Type                string `json:"type"`
	AppName             string `json:"appName"`
	AppInstance         string `json:"appInstance"`
	AppPID              string `json:"appPID"`
	TransactionStart    string `json:"transactionStart"`
	TransactionEnd      string `json:"transactionEnd"`
	ClientIPAddress     string `json:"clientIPAddress"`
	ClientPort          string `json:"clientPort"`
	ServerIPAddress     string `json:"serverIPAddress"`
	ServerPort          string `json:"serverPort"`
	IpProtocol          string `json:"ipProtocol"`
	BytesToClient       string `json:"bytesToClient"`
	BytesFromClient     string `json:"bytesFromClient"`
	BytesFromServer     string `json:"bytesFromServer"`
	BytesToServer       string `json:"bytesToServer"`
	SubsrciberID        string `json:"subsrciberID"`
	ApplicationProtocol string `json:"applicationProtocol"`
	ApplicationName     string `json:"applicationName"`
	Domain              string `json:"domain"`
	DeviceType          string `json:"deviceType"`
	TransactionDuration string `json:"transactionDuration"`
	ContentType         string `json:"contentType"`
	LostBytesClient     string `json:"lostBytesClient"`
	LostBytesServer     string `json:"lostBytesServer"`
	SrttMsClient        string `json:"srttMsClient"`
	SrttMsServer        string `json:"srttMsServer"`
	SubscriberID        string `json:"subscriberID"`
}

type AppInfo struct {
	Timestamp         string
	Type              string
	AppName           string
	AppInstance       string
	AppPID            string
	TransportProtocol string
	AppProtocol       string
	Domain            string
	Device            string
	Content           string
	ApplicationName   string
}

type TrxInfo struct {
	StartDate       string
	EndDate         string
	Duration        string
	BytesToClient   string
	BytesFromClient string
	BytesFromServer string
	BytesToServer   string
	LostBytesClient string
	LostBytesServer string
	SrttMsClient    string
	SrttMsServer    string
	SubscriberID    string
}

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
