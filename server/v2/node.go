package main

import (
	"context"
	"fmt"

	"cosmossdk.io/depinject"
)

type NodeModule struct {
	config NodeConfig
}

type NodeConfig struct {
	Name string
}

func (n *NodeModule) Start(ctx context.Context) error {
	fmt.Printf("Starting node %s\n", n.config.Name)
	<-ctx.Done()
	fmt.Printf("Stopping node %s\n", n.config.Name)
	return nil
}

func (n *NodeModule) ServerConfigSection() string {
	return "node"
}

func (n *NodeModule) ServerConfigObject() interface{} {
	return &n.config
}

func (n *NodeModule) IsManyPerContainerType() {}

func (n *NodeModule) IsServerModule() {}

var _ Module = (*NodeModule)(nil)
var _ HasConfig = (*NodeModule)(nil)
var _ HasStart = (*NodeModule)(nil)

func ProvideNodeModule() Module {
	return &NodeModule{}
}

func init() {
	RegisterServerOnlyModule(depinject.Provide(ProvideNodeModule))
}
