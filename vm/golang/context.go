package golang

import (
	"sync"

	"golang.org/x/net/context"

	corecontext "cosmossdk.io/core/context"
	"cosmossdk.io/core/message"
)

type ContextResolver struct {
	contexts sync.Map
}

func (c *ContextResolver) ResolveContext(packet message.Packet) context.Context {
	ctx, ok := c.contexts.Load(packet.ContextToken())
	if !ok {
		return context.Background()
	}
	return ctx.(context.Context)
}

var _ corecontext.Resolver = &ContextResolver{}
