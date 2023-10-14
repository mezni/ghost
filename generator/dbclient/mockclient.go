package dbclient

import (
	"github.com/mezni/generator/model"
	"github.com/stretchr/testify/mock"
)

type MockBoltClient struct {
	mock.Mock
}

func (m *MockBoltClient) GetClientIpByKey(key string) (model.ClientIp, error) {
	args := m.Mock.Called(key)
	return args.Get(0).(model.ClientIp), args.Error(1)
}

func (m *MockBoltClient) GetSubscriberByKey(key string) (model.Subscriber, error) {
	args := m.Mock.Called(key)
	return args.Get(0).(model.Subscriber), args.Error(1)
}

func (m *MockBoltClient) InitDB() {
	// Does nothing
}

func (m *MockBoltClient) Seed() {
	// Does nothing
}
