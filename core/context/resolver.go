package context

import (
	"context"

	"cosmossdk.io/core/message"
)

type Resolver interface {
	ResolveContext(message.Packet) context.Context
}
