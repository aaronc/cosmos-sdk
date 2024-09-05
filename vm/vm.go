package vm

import (
	"context"

	"cosmossdk.io/depinject"

	"cosmossdk.io/core/account"
	"cosmossdk.io/core/message"
)

type VirtualMachineFactory interface {
	Name() string
	Init(callbacks Callbacks) (VirtualMachine, error)
}

type Callbacks interface {
	Invoke(ctx context.Context, message message.Packet) error
}

type VirtualMachine interface {
	depinject.OnePerModuleType
	AccountHandler(handlerId string) account.Handler
	ModuleHandler(moduleId string) ModuleInitializer
	ModuleHandlers() []string
}

type ModuleInitializer interface {
	isModuleInitializer()
}

type DepinjectModuleInitializer interface {
	ModuleInitializer
	Init([]byte) (depinject.Config, error)
}

type DirectModuleInitializer interface {
	ModuleInitializer
	Init([]byte) (account.Handler, error)
}
