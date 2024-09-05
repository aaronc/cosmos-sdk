package account

import (
	"context"

	"cosmossdk.io/core/message"
)

type Handler interface {
	Descriptor() HandlerDescriptor
	Handle(context.Context, message.Packet) error
}

type HandlerDescriptor struct {
	Methods     []MethodDescriptor
	StateConfig []byte
	ExtraData   any
}

type MethodDescriptor struct {
	Volatility Volatility
	MethodName string
	ExtraData  any
}

type Volatility int

const (
	Pure Volatility = iota
	Readonly
	Volatile
)
