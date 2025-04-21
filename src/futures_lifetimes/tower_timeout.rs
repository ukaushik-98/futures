use std::{error::Error, fmt::Display, pin::Pin, rc::Rc, time::Duration};

use tokio::time::sleep;
use tower::{Service, timeout::Timeout};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Copy)]
struct MyService<'a> {
    url: &'a String,
}

#[derive(Debug, Default)]
struct MyError;

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, Request> Service<Request> for MyService<'a> {
    type Response = &'a String;

    type Error = MyError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request) -> Self::Future {
        Box::pin(async move {
            sleep(Duration::from_millis(2)).await;
            Err(MyError)
        })
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}

// async fn runner2<'a>(m: &'a mut MyService<'a>) {
//     let mut t = Timeout::new(m, Duration::from_millis(100));
//     // let y = tokio::spawn(t.call(())).await;
// }

// async fn wrapper() {
//     let mut m = MyService { url: "url" };
//     let x = runner2(&mut m);
// }

fn static_check<T>(t: T)
where
    T: 'static,
{
}

async fn runner() {
    let y = tokio::spawn(async move {
        let url = String::from("url");
        let mut m = MyService { url: &url };

        let mut t = Timeout::new(m, Duration::from_millis(100));
        let x = t.call(());
        let a = x.await;
    })
    .await;
}
