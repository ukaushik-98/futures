use std::pin::Pin;

trait Service<Request> {
    type Response;
    type Error;
    type Future<'a, 'b>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a + 'b;

    fn call<'a, 'b>(&'a mut self, req: Request) -> Self::Future<'a, 'b>;
}

struct MyRequest;
struct MyResponse;
struct MyError;
#[derive(Debug)]
struct MyService;

impl Service<MyRequest> for MyService {
    type Response = MyResponse;
    type Error = MyError;
    type Future<'a, 'b> =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'b>>;

    fn call<'a, 'b>(&'a mut self, req: MyRequest) -> Self::Future<'a, 'b> {
        Box::pin(async move {
            // println!("{:?}", self);
            Ok(MyResponse)
        })
    }
}

fn static_check<T: 'static>(t: T) {}

async fn runner() {
    let mut m = MyService;
    let mr = m.call(MyRequest);
    let x = tokio::spawn(mr).await;
}
