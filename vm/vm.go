package vm

import (
	"cosmossdk.io/depinject"

	"cosmossdk.io/core/account"
	"cosmossdk.io/core/message"
)

type Callbacks interface {
	Invoke(message message.Packet) error
}

type VirtualMachine interface {
	depinject.OnePerModuleType

	// Init must be called before any other method and only once.
	Init(callbacks Callbacks) error

	AccountHandler(handlerId string) account.Handler
}
