trait Value {
    type MaybeBorrowed<'a>;
}

impl Value for bool {
    type MaybeBorrowed<'a> = bool;
}

trait Key: Value {}

impl Value for bool {
    type MaybeBorrowed<'a> = bool;
}
