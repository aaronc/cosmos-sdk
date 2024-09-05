package hypervisor

import (
	"context"
	"fmt"
	"strings"
	"sync"

	"cosmossdk.io/collections"
	"cosmossdk.io/depinject"

	"cosmossdk.io/core/account"
	"cosmossdk.io/core/address"
	appmodulev2 "cosmossdk.io/core/appmodule/v2"
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
	contexts            sync.Map
	authMiddleware      AuthMiddleware
}

type InitParams struct {
	depinject.In
	KVStoreService store.KVStoreService
	StateHandler   StateHandler
	Modules        map[string]appmodulev2.AppModule
	AuthMiddleware AuthMiddleware `optional:"true"`
}

func ProvideHypervisor(params InitParams) (*Hypervisor, error) {
	return &Hypervisor{}, nil
}

type moduleInstance struct {
	handler      account.Handler
	name         string
	address      address.Address
	nativeModule appmodulev2.AppModule
}

type addressKey struct{}

type contextData struct {
	ctx     context.Context
	address address.Address
}

func (h *Hypervisor) Invoke(ctx context.Context, packet message.Packet) error {
	return h.invoke(ctx, packet)
}

func (h *Hypervisor) invoke(ctx context.Context, packet message.Packet) error {
	// TODO check volatility
	ctxToken := h.newContextToken()
	h.contexts.Store(ctxToken, &contextData{ctx: ctx, address: packet.TargetAddress()})
	packet.SetContextToken(ctxToken)
	defer h.contexts.Delete(ctxToken)

	if packet.TargetAddress().IsEmpty() {
		msgName := packet.MessageName()
		if !strings.HasPrefix(msgName, "module:") {
			return fmt.Errorf("unknown module message: %s", msgName)
		}
		msgName = strings.TrimPrefix(msgName, "module:")
		mod, ok := h.moduleMessageRouter[msgName]
		if !ok {
			return fmt.Errorf("unknown module message: %s", packet.MessageName())
		}
		return mod.handler.Handle(ctx, packet)
	}

	handlerId, err := h.accountHandlerIds.Get(ctx, packet.TargetAddress())
	if err != nil {
		return err
	}

	handlerParts := strings.Split(handlerId, ":")
	if len(handlerParts) < 2 {
		return fmt.Errorf("invalid handler id: %s", handlerId)
	}

	vmName := handlerParts[0]
	machine, ok := h.virtualMachines[vmName]
	if !ok {
		return fmt.Errorf("unknown virtual machine: %s", vmName)
	}

	vmHandlerId := strings.Join(handlerParts[1:], ":")
	handler := machine.AccountHandler(vmHandlerId)
	if handler == nil {
		return fmt.Errorf("unknown account handler: %s", handlerId)
	}

	return handler.Handle(ctx, packet)
}

func (h *Hypervisor) newContextToken() [32]byte {
	// TODO: generate a secure random token
	panic("TODO")
}

type StateHandler interface {
	CreateState(ctx context.Context, address address.Address, config []byte) error
}

type callbacks struct {
	hypervisor *Hypervisor
}

func (c callbacks) Invoke(message message.Packet) error {
	ctx, ok := c.hypervisor.contexts.Load(message.ContextToken())
	if !ok {
		return fmt.Errorf("context not found")
	}

	ctxData := ctx.(*contextData)

	if c.hypervisor.authMiddleware != nil {
		if err := c.hypervisor.authMiddleware.Authenticate(ctxData.ctx, ctxData.address, message); err != nil {
			return err
		}
	} else if message.CallerAddress().Equals(ctxData.address) {
		return fmt.Errorf("caller address mismatch")
	}

	return c.hypervisor.invoke(ctxData.ctx, message)
}

var _ vm.Callbacks = &callbacks{}

type AuthMiddleware interface {
	Authenticate(ctx context.Context, actualCaller address.Address, packet message.Packet) error
}
