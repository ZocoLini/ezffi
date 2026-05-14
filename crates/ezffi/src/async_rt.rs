pub type AsyncDispatch =
    fn(core::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send + 'static>>);

static DISPATCHER: std::sync::OnceLock<AsyncDispatch> = std::sync::OnceLock::new();

pub fn set_async_dispatcher(dispatcher: AsyncDispatch) {
    DISPATCHER
        .set(dispatcher)
        .map_err(|_| ())
        .expect("ezffi::set_async_dispatcher has already been called");
}

fn default_dispatch(
    fut: core::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send + 'static>>,
) {
    pollster::block_on(fut);
}

pub fn block_on<F, T>(fut: F) -> T
where
    F: core::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let dispatcher = DISPATCHER.get().copied().unwrap_or(default_dispatch);
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let boxed: core::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send + 'static>> =
        Box::pin(async move {
            let _ = tx.send(fut.await);
        });
    dispatcher(boxed);
    rx.recv()
        .expect("ezffi async dispatcher returned without driving the future to completion")
}
