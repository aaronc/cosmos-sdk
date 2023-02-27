package cel

import (
	"testing"

	exprpb "google.golang.org/genproto/googleapis/api/expr/v1alpha1"
	"google.golang.org/protobuf/reflect/protoregistry"
	"gotest.tools/v3/assert"
)

func TestCEL(t *testing.T) {
	exec, err := ExecutorOptions{ProtoRegistry: protoregistry.GlobalFiles}.Build()
	assert.NilError(t, err)
	res, err := exec.Exec("google.protobuf.Timestamp{seconds: now()}", &exprpb.Type{
		TypeKind: &exprpb.Type_MessageType{
			MessageType: "google.protobuf.Timestamp",
		},
	})
	assert.NilError(t, err)
	t.Logf("result: %+v", res)
	t.Logf("result type: %+v", res.Type())
}
