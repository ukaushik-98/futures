use std::{pin::Pin, time::Duration};

pub trait Service<Request> {
    type Response;
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a>;
}

pub struct MyRequest;
pub struct MyResponse;
pub struct MyError;

#[derive(Debug, Clone, Copy)]
pub struct MyService<T> {
    inner: T,
}

impl<T> Service<MyRequest> for MyService<T>
where
    T: Send,
{
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
pub struct MyServiceWrapper<T> {
    inner: MyService<T>,
}

impl<T> Service<MyRequest> for MyServiceWrapper<T>
where
    T: Send,
{
    type Response = T;

    type Error = MyError;

    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: MyRequest) -> Self::Future<'a> {
        Box::pin(async { self.inner.call(req).await })
    }
}

async fn runner<'a, T>(x: T)
where
    T: Send + 'a,
{
    let m = MyService { inner: x };
    let mut mw = MyServiceWrapper { inner: m };
    // let a = mw.call(MyRequest);
    let x = tokio::spawn(async move {
        let q = mw.call(MyRequest);
    })
    .await;
}
