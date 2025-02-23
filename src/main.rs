use futures::futures_lifetimes::{MyRequest, MyService, Service};

#[tokio::main]
async fn main() {
    let mut m = MyService;
    let mr = m.call(MyRequest);
    let _ = tokio::spawn(mr).await;
}
