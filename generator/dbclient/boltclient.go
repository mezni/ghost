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
	OpenBoltDb()
	//        QueryAccount(accountId string) (model.Account, error)
	Seed()
}

type BoltClient struct {
	boltDB *bolt.DB
}

func (bc *BoltClient) OpenBoltDb() {
	var err error
	bc.boltDB, err = bolt.Open("accounts.db", 0600, nil)
	if err != nil {
		log.Fatal(err)
	}
}

func (bc *BoltClient) Seed() {
	bc.initBucket("SubscriberBucket")
	bc.initBucket("ClientIpAddressBucket")
	bc.seedSubscribers()
	bc.seedClientIpAddress()
}

func (bc *BoltClient) initBucket(bucketName string) {
	bc.boltDB.Update(func(tx *bolt.Tx) error {
		_, err := tx.CreateBucket([]byte(bucketName))
		if err != nil {
			return fmt.Errorf("create bucket failed: %s", err)
		}
		return nil
	})
}

func (bc *BoltClient) seedSubscribers() {
	subsrciberCount := 10000
	prefex := rand.Intn(1000) + 1000
	for i := 0; i < subsrciberCount; i++ {
		key := strconv.Itoa(i + 1)
		subscriber := model.Subsrciber{
			Id:               key,
			SubsrciberID:     "2010" + strconv.Itoa(prefex) + strconv.Itoa(i+100000),
			SubsrciberLastTS: "",
		}

		jsonBytes, _ := json.Marshal(subscriber)

		// Write the data to the AccountBucket
		bc.boltDB.Update(func(tx *bolt.Tx) error {
			b := tx.Bucket([]byte("SubscriberBucket"))
			err := b.Put([]byte(key), jsonBytes)
			return err
		})

	}

	fmt.Printf("Seeded %v fake accounts...\n", subsrciberCount)
}

func generateIP() string {
	ip := ""
	for i := 0; i <= 3; i++ {
		ip = ip + strconv.Itoa(rand.Intn(255)) + "."
	}
	ip = ip[0 : len(ip)-1]
	return ip
}

func (bc *BoltClient) seedClientIpAddress() {
	ipCount := 10000
	for i := 0; i < ipCount; i++ {
		key := strconv.Itoa(i + 1)
		ip := model.ClientIpAddress{
			Id:                key,
			ClientIpAddress:   generateIP(),
			ClientIpAddressTS: "",
		}

		jsonBytes, _ := json.Marshal(ip)

		// Write the data to the AccountBucket
		bc.boltDB.Update(func(tx *bolt.Tx) error {
			b := tx.Bucket([]byte("ClientIpAddressBucket"))
			err := b.Put([]byte(key), jsonBytes)
			return err
		})

	}

	fmt.Printf("Seeded %v fake accounts...\n", ipCount)
}
