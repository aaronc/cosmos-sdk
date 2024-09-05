package message

import (
	"cosmossdk.io/core/address"
	"cosmossdk.io/core/transaction"
)

type Packet struct {
	data     []byte
	protoReq transaction.Msg
	protoRes transaction.Msg
}

func (m Packet) TargetAddress() address.Address {
	panic("not implemented")
}

func (m Packet) CallerAddress() address.Address {
	panic("not implemented")
}

func (m Packet) ContextToken() [32]byte {
	panic("not implemented")
}

func (m Packet) MessageName() string {
	panic("not implemented")
}

func (m Packet) SetContextToken(token [32]byte) {
	panic("not implemented")
}

type StateToken [32]byte

func (m Packet) StateToken() StateToken {
	panic("not implemented")
}

func (s StateToken) IsVolatile() bool {
	panic("not implemented")
}
