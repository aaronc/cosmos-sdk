package address

import "bytes"

// Address defines a type for a byte array that represents an address.
type Address []byte

func (a Address) IsEmpty() bool {
	return len(a) == 0
}

func (a Address) IsValid() bool {
	return len(a) < 63
}

func (a Address) Equals(address Address) bool {
	return bytes.Equal(a, address)
}
