package cel

import (
	"fmt"
	"time"

	"github.com/google/cel-go/cel"
	"github.com/google/cel-go/common/types"
	"github.com/google/cel-go/common/types/ref"
	"github.com/google/cel-go/interpreter"
	exprpb "google.golang.org/genproto/googleapis/api/expr/v1alpha1"
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/reflect/protoregistry"
)

type ExecutorOptions struct {
	ProtoRegistry *protoregistry.Files
}

func (e ExecutorOptions) Build() (*Executor, error) {
	reg, err := types.NewRegistry()
	if err != nil {
		return nil, err
	}

	if e.ProtoRegistry != nil {
		var err error
		e.ProtoRegistry.RangeFiles(func(f protoreflect.FileDescriptor) bool {
			err = reg.RegisterDescriptor(f)
			if err != nil {
				return false
			}
			return true
		})
		if err != nil {
			return nil, err
		}
	}

	env, err := cel.NewEnv(
		cel.CustomTypeProvider(reg),
		cel.Function("now",
			cel.Overload("now", []*cel.Type{}, cel.IntType,
				cel.FunctionBinding(func(values ...ref.Val) ref.Val {
					now := time.Now()
					fmt.Printf("now() called %v\n", now)
					return types.Int(now.Second())
				}),
			),
		),
	)
	if err != nil {
		return nil, err
	}

	return &Executor{
		env: env,
	}, nil
}

type Executor struct {
	env *cel.Env
}

func (e *Executor) Exec(expr string, expectedType *exprpb.Type) (ref.Val, error) {
	ast, iss := e.env.Compile(expr)
	if iss != nil {
		return nil, fmt.Errorf("compile issues: %+v", iss)
	}
	ast, iss = e.env.Check(ast)
	if iss != nil {
		return nil, fmt.Errorf("type check issues: %+v", iss)
	}

	expType, err := cel.ExprTypeToType(expectedType)
	if err != nil {
		return nil, err
	}
	if !ast.OutputType().IsAssignableType(expType) {
		return nil, fmt.Errorf("expression didn't match type %+v", expType)
	}

	prg, err := e.env.Program(
		ast,
		cel.CostLimit(10000),
		cel.EvalOptions(cel.OptTrackCost),
		cel.CostTracking(costEstimator{}),
	)
	if err != nil {
		return nil, err
	}

	res, details, err := prg.Eval(interpreter.EmptyActivation())
	if err != nil {
		return nil, err
	}

	cost := details.ActualCost()
	if cost != nil {
		fmt.Printf("actual cost: %d\n", *cost)
	}

	return res, nil
}

type costEstimator struct{}

func (c costEstimator) CallCost(function, overloadID string, args []ref.Val, result ref.Val) *uint64 {
	x := uint64(1)
	fmt.Printf("CallCost: %s %s %+v %+v\n", function, overloadID, args, result)
	return &x
}
