use futures::executor;

async fn foo() -> u8 {
    10
}

async fn bar() -> u8 {
    let a = foo().await;
    println!("{}", a);
    a
}

fn main() {
    executor::block_on(bar());
}
