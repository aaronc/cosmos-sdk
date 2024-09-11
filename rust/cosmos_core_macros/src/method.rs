use syn::Signature;

// for each method generate a struct with the same fields as its parameters
// use that struct to decode the parameters and also for metadata about the method
// because field macro arguments will get properly dealt with
