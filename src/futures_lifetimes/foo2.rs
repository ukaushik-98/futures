use std::pin::Pin;

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

impl<'t, T, Request> Service<Request> for MyFooService<'t, T>
where
    T: Send + Sync,
{
    type Response = &'t T;

    type Error = ();

    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>
    where
        Self: 'a,
        't: 'a;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a> {
        Box::pin(async move { Ok(self.x) })
    }
}

fn runner() {
    let s = "hello";
    let mut a = MyFooService { x: &s };
    let b = a.call(());
}

fn spawn_runner<'a>(s: &'a String) {
    // let s = "hello";
    let mut a = MyFooService { x: s };
    let b = a.call(());
    // let s = tokio::spawn(a.call(()));
}
