package hypervisor

import (
	"context"

	"cosmossdk.io/collections"
	"cosmossdk.io/depinject"

	"cosmossdk.io/core/account"
	"cosmossdk.io/core/address"
	"cosmossdk.io/core/message"
	"cosmossdk.io/core/store"
	"cosmossdk.io/vm"
)

type Hypervisor struct {
	virtualMachines     map[string]vm.VirtualMachine
	schema              collections.Schema
	accountHandlerIds   collections.Map[address.Address, string]
	modules             map[string]*moduleInstance
	moduleMessageRouter map[string]*moduleInstance
	stateHandler        StateHandler
}

type InitParams struct {
	depinject.In
	KVStoreService store.KVStoreService
	StateHandler   StateHandler
}

func NewHyperVisor(params InitParams) *Hypervisor {
	return &Hypervisor{}
}

type moduleInstance struct {
	handler account.Handler
	name    string
	address address.Address
}

func (h *Hypervisor) Invoke(ctx context.Context, message message.Packet) error {
	_, err := h.accountHandlerIds.Get(ctx, message.TargetAddress())
	if err != nil {
		return err
	}

	panic("TODO")
}

type StateHandler interface {
	CreateState(ctx context.Context, address address.Address, config []byte) error
}
