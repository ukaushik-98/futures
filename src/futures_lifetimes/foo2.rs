use std::pin::Pin;

use super::multi::MyRequest;

pub trait Service<Request> {
    type Response;
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a>;
}

struct MyFooService<'a, T> {
    x: &'a T,
}

impl<'t, T> Service<MyRequest> for MyFooService<'t, T> {
    type Response = &'t T;

    type Error = ();

    type Future<'b>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>
    where
        Self: 'b;

    fn call<'a>(&'a mut self, req: MyRequest) -> Self::Future<'a> {
        todo!()
    }
}
