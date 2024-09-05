package golang

import (
	"cosmossdk.io/core/account"
	"cosmossdk.io/vm"
)

type VM struct {
}

func (V VM) AccountHandler(handlerId string) account.Handler {
	//TODO implement me
	panic("implement me")
}

func (V VM) ModuleHandler(moduleId string) vm.ModuleInitializer {
	//TODO implement me
	panic("implement me")
}

func (V VM) ModuleHandlers() []string {
	//TODO implement me
	panic("implement me")
}

var _ vm.VirtualMachine = &VM{}
