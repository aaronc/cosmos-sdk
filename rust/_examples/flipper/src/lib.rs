#![account_handler(Flipper)]

use state_objects::*;
use cosmos_core::*;

//#[derive(Schema)]
pub struct Flipper {
    value: Item<bool>,
}

#[publish]
impl Flipper {
    fn get(&self, ctx: &Context) -> Response<bool> {
        // self.value.get(ctx)
        todo!()
    }

    fn flip(&self, ctx: &mut Context) -> Response<()> {
        let value = *self.value.get(ctx)?;
        self.value.set(ctx, &!value)?;
        ctx.ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmos_testing::*;

    #[test]
    fn test1() {
        let mut test_app = TestApp::new();
        test_app.add_handler::<Flipper>();
        test_app.client_context();
    }
}