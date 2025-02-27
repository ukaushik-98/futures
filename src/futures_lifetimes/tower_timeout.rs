use std::{error::Error, pin::Pin, time::Duration};

use tower::{Service, timeout::Timeout};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

struct MyService<'a> {
    url: &'a String,
}

impl<'a, Request> Service<Request> for MyService<'a> {
    type Response = &'a String;

    type Error = Box<dyn Error + Send + Sync>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request) -> Self::Future {
        // Box::pin(async { Ok(self.url) })
        todo!()
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}

fn static_check<T: 'static>(t: T) {}

async fn runner2<'a>(m: &'a mut MyService<'a>) {
    let mut t = Timeout::new(m, Duration::from_millis(100));
    let y = tokio::spawn(t.call(())).await;
}

async fn wrapper() {
    let url = String::from("url");

    let mut m = MyService { url: &url };
    let x = runner2(&mut m);
}

// async fn runner() {
//     let mut m = MyService;
//     let mut t = Timeout::new(m, Duration::from_millis(100));
//     let x = t.call(());
//     // static_check(t);
//     let y = tokio::spawn(async move {
//         let a = x.await;
//     })
//     .await;
// }
