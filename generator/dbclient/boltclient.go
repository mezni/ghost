package dbclient

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"strconv"

	"github.com/boltdb/bolt"
	"github.com/mezni/generator/model"
)

type IBoltClient interface {
	InitDB(dbName string)
	Seed()
	GetSubscriberByKey(key string) (model.Subscriber, error)
	GetClientIpByKey(key string) (model.ClientIp, error)
}

type BoltClient struct {
	boltDB *bolt.DB
}

func generateIP() string {
	ip := ""
	for i := 0; i <= 3; i++ {
		ip = ip + strconv.Itoa(rand.Intn(255)) + "."
	}
	ip = ip[0 : len(ip)-1]
	return ip
}

func (bc *BoltClient) InitDB(dbName string) {
	var err error
	bc.boltDB, err = bolt.Open(dbName, 0600, nil)
	if err != nil {
		log.Fatal(err)
	}
}

func (bc *BoltClient) Seed() {
	bc.createBucket("SUBSCRIBERS")
	bc.createBucket("CLIENTIPADD")
	bc.seedSubscribers()
	bc.seedClientIps()
}

func (bc *BoltClient) createBucket(bucketName string) {
	bc.boltDB.Update(func(tx *bolt.Tx) error {
		_, err := tx.CreateBucket([]byte(bucketName))
		if err != nil {
			return fmt.Errorf("create bucket failed: %s", err)
		}
		return nil
	})
}

func (bc *BoltClient) seedSubscribers() error {
	subsrciberCount := 1000
	subsrciberPrefix := strconv.Itoa(rand.Intn(100) + 100)

	for i := 0; i < subsrciberCount; i++ {
		key := strconv.Itoa(i + 1)
		subscriber := model.Subscriber{
			Id:              key,
			SubscriberID:    "2010" + subsrciberPrefix + strconv.Itoa(rand.Intn(1000000)+1000000),
			SubscriberEndTS: "",
		}
		err := bc.addSubscriber(subscriber)

		if err != nil {
			fmt.Errorf("could not add subscriber to DB: %v", err)
		}
	}
	return nil
}

func (bc *BoltClient) addSubscriber(subscriber model.Subscriber) error {
	jsonBytes, err := json.Marshal(subscriber)
	if err != nil {
		return fmt.Errorf("could not marshal subscriber json: %v", err)
	}
	key := subscriber.Id
	err = bc.boltDB.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("SUBSCRIBERS"))
		err := b.Put([]byte(key), jsonBytes)
		if err != nil {
			return fmt.Errorf("could not set config: %v", err)
		}
		return nil
	})
	//	fmt.Println("Set Config")
	return nil
}

func (bc *BoltClient) seedClientIps() error {
	ipCount := 1000
	for i := 0; i < ipCount; i++ {
		key := strconv.Itoa(i + 1)

		ip := model.ClientIp{
			Id:      key,
			IpID:    generateIP(),
			IpEndTS: ""}
		err := bc.addClientIp(ip)

		if err != nil {
			fmt.Errorf("could not add ip to DB: %v", err)
		}
	}
	return nil

}

func (bc *BoltClient) addClientIp(ip model.ClientIp) error {
	jsonBytes, err := json.Marshal(ip)
	if err != nil {
		return fmt.Errorf("could not marshal ip json: %v", err)
	}
	key := ip.Id
	err = bc.boltDB.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("CLIENTIPADD"))
		err := b.Put([]byte(key), jsonBytes)
		if err != nil {
			return fmt.Errorf("could not set ip: %v", err)
		}
		return nil
	})
	return nil
}

func (bc *BoltClient) GetSubscriberByKey(key string) (s model.Subscriber, e error) {
	e = bc.boltDB.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("SUBSCRIBERS"))
		v := b.Get([]byte(key))
		e = json.Unmarshal([]byte(v), &s)
		return e
	})
	return s, e
}

func (bc *BoltClient) GetClientIpByKey(key string) (s model.ClientIp, e error) {
	e = bc.boltDB.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("CLIENTIPADD"))
		v := b.Get([]byte(key))
		e = json.Unmarshal([]byte(v), &s)
		return e
	})
	return s, e
}
