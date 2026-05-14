use std::sync::OnceLock;
use std::time::Duration;

static TOKIO_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

#[ezffi::export]
pub fn init_tokio() {
    TOKIO_RT
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_time()
                .build()
                .unwrap(),
        )
        .ok();
    ezffi::set_async_dispatcher(|fut| {
        TOKIO_RT.get().unwrap().block_on(fut);
    });
}

#[ezffi::export]
pub async fn async_double(x: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(1)).await;
    x * 2
}

#[ezffi::export]
pub async fn async_sum_three(a: u32, b: u32, c: u32) -> u32 {
    let s1 = async_inner(a, b).await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    s1 + c
}

async fn async_inner(a: u32, b: u32) -> u32 {
    a + b
}
