async fn foo() -> u8 { 10 }

fn bar() -> impl Future<Output=u8> {
    async {
        let x: u8 = foo().await;
        x + 20
    }
}
