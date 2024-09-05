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

func (m Packet) ContextToken() []byte {
	panic("not implemented")
}
