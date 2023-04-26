package main

import (
	"context"
	"sync"

	"cosmossdk.io/depinject"
	"github.com/spf13/cobra"
)

type Module interface {
	depinject.ManyPerContainerType

	IsServerModule()
}

type HasConfig interface {
	Module

	ServerConfigSection() string
	ServerConfigObject() interface{}
}

type HasCommands interface {
	Module

	ServerCommands() []*cobra.Command
}

type HasStart interface {
	Module

	Start(context.Context) error
}

var (
	serverOnlyModules    []depinject.Config
	serverOnlyModulesMtx sync.RWMutex
)

func RegisterServerOnlyModule(config depinject.Config) {
	serverOnlyModulesMtx.Lock()
	defer serverOnlyModulesMtx.Unlock()

	serverOnlyModules = append(serverOnlyModules, config)
}
