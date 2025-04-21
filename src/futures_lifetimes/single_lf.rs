use std::{pin::Pin, rc::Rc, time::Duration};

use tokio::time::sleep;

trait Service<Request> {
    type Response;
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>> + 'a
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a>;
}

struct MyRequest;
struct MyResponse;
struct MyError;

#[derive(Debug, Clone, Copy)]
struct MyService<T> {
    inner: T,
}

impl<T> Service<MyRequest> for MyService<T> {
    type Response = T;
    type Error = MyError;
    // 'a:'static because of spawn
    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>
    where
        T: 'a;

    // the borrow of mut self must live of 'static -> &'static mut self
    fn call<'a>(&'a mut self, req: MyRequest) -> Self::Future<'a> {
        Box::pin(async move {
            // println! ("{:?}", self);
            Err(MyError)
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct MyServiceWrapper<T> {
    inner: MyService<T>,
}

impl<T> Service<MyRequest> for MyServiceWrapper<T> {
    type Response = T;

    type Error = MyError;

    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: MyRequest) -> Self::Future<'a> {
        // Box::pin(async { self.inner.call(req).await })
        todo!()
    }
}

fn create_T<'a, T>() -> T {
    todo!()
}

fn create_serve<'a, T>(x: T) -> MyServiceWrapper<T> {
    let m = MyService { inner: x };
    MyServiceWrapper { inner: m }
}

fn static_check<T>(t: T)
where
    T: 'static,
{
}

async fn runner<'a, T>()
where
    T: Sync + Send + 'a,
{
    let x = tokio::spawn(async move {
        let t = create_T::<T>();
        let mut mw = create_serve(t);
        let x = mw.call(MyRequest).await;

        sleep(Duration::from_millis(100)).await;
    })
    .await;
}
