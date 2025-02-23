use std::pin::Pin;

pub trait Service<Request> {
    type Response;
    type Error;
    type Future<'a, 'b>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a + 'b;

    fn call<'a, 'b>(&'a mut self, req: Request) -> Self::Future<'a, 'b>;
}

pub struct MyRequest;
pub struct MyResponse;
pub struct MyError;

#[derive(Debug)]
pub struct MyService;

impl Service<MyRequest> for MyService {
    type Response = MyResponse;
    type Error = MyError;
    // since there are 2 lifetimes, 'a doesn't have to be 'b.
    // 'b = 'static
    type Future<'a, 'b> =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'b>>;

    // 'b = 'static here but 'a borrow is decoupled from the result
    fn call<'a, 'b>(&'a mut self, req: MyRequest) -> Self::Future<'a, 'b> {
        Box::pin(async move {
            // println! ("{:?}", self);
            Ok(MyResponse)
        })
    }
}

async fn runner() {
    let mut m = MyService;
    let mr = m.call(MyRequest);
    tokio::spawn(mr);
}
