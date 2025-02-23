use std::pin::Pin;

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

#[derive(Debug)]
pub struct MyService;

impl Service<MyRequest> for MyService {
    type Response = MyResponse;
    type Error = MyError;
    type Future<'a> =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>; // 'a:'static because of spawn

    // the borrow of mut self must live of 'static -> &'static mut self
    fn call<'a>(&'a mut self, req: MyRequest) -> Self::Future<'a> {
        Box::pin(async move {
            // println! ("{:?}", self);
            Ok(MyResponse)
        })
    }
}

fn static_check<T: 'static>(t: T) {}

async fn runner() {
    let mut m = MyService;
    let mr = m.call(MyRequest).await;
    // tokio::spawn(mr);
}
