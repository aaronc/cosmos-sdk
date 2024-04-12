type Address = Vec<u8>;

trait Object {
    type Key;
}

trait OwnedObject: Object {
    type CreateAction;
    type UpdateAction;

    fn create(owner: &Address, key: &Self::Key, action: &Self::CreateAction) -> Op<Self>;
    fn update(self, owner: &Address, key: &Self::Key, action: &Self::UpdateAction) -> Op<Self>;
}

enum Op<T> {
    Transfer(Address, T),
    Save(T),
    Delete,
}

struct Class {
    metadata: String,
}

impl Object for Class {
    type Key = String;
}

struct CreateClass {
    metadata: String,
}

enum ClassAction {
    UpdateMetadata(String),
    UpdateAdmin(Address),
}

impl OwnedObject for Class {
    type CreateAction = CreateClass;
    type UpdateAction = ClassAction;

    fn create(owner: &Address, key: &Self::Key, action: &Self::CreateAction) -> Op<Self> {
        Op::Save(Class {
            metadata: action.metadata.clone()
        })
    }

    fn update(self, owner: &Address, key: &Self::Key, action: &Self::UpdateAction) -> Op<Self> {
        match action {
            ClassAction::UpdateMetadata(new_metadata) => {
                Op::Save(Class {
                    metadata: new_metadata.clone()
                })
            }
            ClassAction::UpdateAdmin(new_admin) => {
                Op::Transfer(new_admin.clone(), self)
            }
        }
    }
}

struct OwnedAction<A> {
    action: A,
    owner: Address,
}

struct OwnedEntity<E: OwnedObject> {
    owner: Address,
    key: E::Key,
    object: E,
}