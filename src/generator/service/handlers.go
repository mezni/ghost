package service

import (
	"encoding/json"
	"math/rand"
	"net/http"
	"strconv"
	"time"

	"github.com/mezni/generator/dbclient"
	"github.com/mezni/generator/model"
)

const timeFormat = "2006-01-02 15:04:05"
const trxDurationMax = 3600

var DBClient dbclient.IBoltClient

func GetEvents(w http.ResponseWriter, r *http.Request) {

	w.Header().Set("Content-Type", "application/json")
	var inRequest model.InRequest
	err := json.NewDecoder(r.Body).Decode(&inRequest)

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	intervalStartDate := inRequest.IntervalStartDate
	intervalMinutes := inRequest.IntervalMinutes
	trxCount := inRequest.TrxCount

	events, _ := generateEvents(intervalStartDate, intervalMinutes, trxCount)
	json.NewEncoder(w).Encode(events)
}

func generateEvents(intervalStartDate string, intervalMinutes int, trxCount int) ([]model.Event, error) {
	var events []model.Event

	startDate, err := time.Parse(timeFormat, intervalStartDate)
	if err != nil {
		return events, err
	}
	batchDate := strconv.Itoa(int(startDate.Unix()))
	for i := 0; i < trxCount; i++ {
		trxInfo := generateTrxInfo(startDate, intervalMinutes)
		appInfo := generateAppInfo(startDate)
		event := model.Event{}
		event.TransactionStart = trxInfo.StartDate
		event.TransactionEnd = trxInfo.EndDate
		event.TransactionDuration = trxInfo.EndDate
		event.BytesToClient = trxInfo.BytesToClient
		event.BytesFromClient = trxInfo.BytesFromClient
		event.BytesFromServer = trxInfo.BytesFromServer
		event.BytesToServer = trxInfo.BytesToServer
		event.LostBytesClient = trxInfo.LostBytesClient
		event.LostBytesServer = trxInfo.LostBytesServer
		event.SrttMsClient = trxInfo.SrttMsClient
		event.SrttMsServer = trxInfo.SrttMsServer

		ipc, _ := DBClient.GetClientIpByKey(strconv.Itoa(rand.Intn(10000)))
		event.ClientIPAddress = ipc.IpID
		event.ClientPort = strconv.Itoa(rand.Intn(46000) + 1024)

		ips, _ := DBClient.GetClientIpByKey(strconv.Itoa(rand.Intn(10000)))
		event.ServerIPAddress = ips.IpID
		event.ServerPort = strconv.Itoa(443)

		event.Timestamp = batchDate
		event.IpProtocol = appInfo.TransportProtocol
		event.ApplicationProtocol = appInfo.AppProtocol
		event.ApplicationName = appInfo.ApplicationName
		event.Domain = appInfo.Domain
		event.DeviceType = appInfo.Device
		event.ContentType = appInfo.Content

		event.Type = appInfo.Type
		event.AppName = appInfo.AppName
		event.AppInstance = appInfo.AppInstance
		event.AppPID = appInfo.AppPID
		event.SubscriberID = trxInfo.SubscriberID

		events = append(events, event)
	}

	return events, nil
}

func generateAppInfo(startDate time.Time) model.AppInfo {
	var refTraProtocol = []string{"TCP", "UDP"}
	var refAppProtocol = []string{"https", "quic"}
	var refAppName = []string{"Youtube", "Facebook", "Google APIs", "Tiktok", "-"}
	var refDomain = []string{"youtubei.googleapi.com", "graph.facebook.com", "196.204.5.48",
		"142.250.185.106", "i.yting.com"}
	var refDevice = []string{"Samsung S22", "Samsung A54", "Iphone 14", "Iphone 14 pro", "Pixel"}
	var refContent = []string{"Web", "Video", "Text", "-"}

	var appInfo model.AppInfo
	appInfo.TransportProtocol = refTraProtocol[rand.Intn(len(refTraProtocol))]
	appInfo.AppProtocol = refAppProtocol[rand.Intn(len(refAppProtocol))]
	appInfo.ApplicationName = refAppName[rand.Intn(len(refAppName))]
	appInfo.Domain = refDomain[rand.Intn(len(refDomain))]
	appInfo.Device = refDevice[rand.Intn(len(refDevice))]
	appInfo.Content = refContent[rand.Intn(len(refContent))]

	appInfo.Type = "AllIPMessages"
	appInfo.AppName = "TraficServerElement"
	appInfo.AppInstance = strconv.Itoa(int(startDate.Unix()))[3:7]
	appInfo.AppPID = strconv.Itoa(rand.Intn(55000) + 1000)

	return appInfo
}

func generateTrxInfo(startDate time.Time, intervalMinutes int) model.TrxInfo {
	var trxInfo model.TrxInfo
	startDateTS := int(startDate.Unix())
	endDate := startDate.Add(time.Minute * time.Duration(intervalMinutes))
	endDateTS := int(endDate.Unix())
	trxDuration := rand.Intn(trxDurationMax)
	trxEndDateTS := int(rand.Intn(int(endDateTS-startDateTS))) + int(startDateTS)
	trxStartDateTS := int(startDateTS) - trxDuration

	trxInfo.StartDate = strconv.Itoa(trxStartDateTS)
	trxInfo.EndDate = strconv.Itoa(trxEndDateTS)
	trxInfo.Duration = strconv.Itoa(trxDuration)

	bytesToClient := rand.Intn(10000)
	bytesFromClient := rand.Intn(10000)
	trxInfo.BytesToClient = strconv.Itoa(bytesToClient)
	trxInfo.BytesFromClient = strconv.Itoa(bytesFromClient)
	trxInfo.BytesFromServer = strconv.Itoa(bytesFromClient)
	trxInfo.BytesToServer = strconv.Itoa(bytesToClient)

	trxInfo.LostBytesClient = strconv.Itoa(rand.Intn(512))
	trxInfo.LostBytesServer = strconv.Itoa(rand.Intn(512))
	trxInfo.SrttMsClient = strconv.Itoa(rand.Intn(512))
	trxInfo.SrttMsServer = strconv.Itoa(rand.Intn(512))

	s, _ := DBClient.GetSubscriberByKey(strconv.Itoa(rand.Intn(10000)))

	trxInfo.SubscriberID = s.SubscriberID
	return trxInfo
}
