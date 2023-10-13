package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"strconv"

	"github.com/boltdb/bolt"
)

type Subscriber struct {
	Id              string
	SubscriberID    string
	SubscriberEndTS string
}

type BoltClient struct {
	boltDB *bolt.DB
}

func main() {
	db := &BoltClient{}
	db.initDB("tttt")
	db.createBucket("SUBSCRIBERS")
	db.createBucket("CLIENTIPADD")
	db.seedSubscribers()
	
}

func (bc *BoltClient) initDB(dbName string) {
	var err error
	bc.boltDB, err = bolt.Open(dbName, 0600, nil)
	if err != nil {
		log.Fatal(err)
	}
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
	subsrciberCount := 10
	subsrciberPrefix := rand.Intn(1000) + 1000

	for i := 0; i < subsrciberCount; i++ {
		key := strconv.Itoa(i + 1)
		subscriber := Subscriber{
			Id:              key,
			SubscriberID:    "2010" + strconv.Itoa(subsrciberPrefix) + strconv.Itoa(i+100000),
			SubscriberEndTS: "",
		}
		err := bc.addSubscriber(subscriber)

		if err != nil {
			fmt.Errorf("could not add subscriber to DB: %v", err)
		}
	}
	return nil
}

func (bc *BoltClient) addSubscriber(subscriber Subscriber) error {
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
