package main

import (
	"encoding/json"
	"fmt"
	"log"

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
	DBClient := &BoltClient{}
	DBClient.initDB("tttt")
	DBClient.createBucket("SUBSCRIBERS")
	s := &Subscriber{"1", "50015001", ""}
	err := DBClient.addSubscriber(*s)
	
	if err != nil {
		fmt.Errorf("could not add subscriber to DB: %v", err)
	}

	s = &Subscriber{"2", "50015002", ""}
	err = DBClient.addSubscriber(*s)
	
	if err != nil {
		fmt.Errorf("could not add subscriber to DB: %v", err)
	}


	DBClient.boltDB.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("SUBSCRIBERS"))
		v := b.Get([]byte("1"))
		fmt.Printf("The answer is: %s\n", v)
		return nil
	})

/*
	err = DBClient.boltDB.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte("SUBSCRIBERS"))
		b.ForEach(func(k, v []byte) error {
			fmt.Println(string(k), string(v))
			return nil
		})
		return nil
	})
*/

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
	fmt.Println("Set Config")
	return nil
}
