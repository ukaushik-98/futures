use std::{fmt::Debug, ops::DerefMut, pin::Pin};

pub trait Service<Request> {
    type Response;
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a>;
}

struct FooService<'a, T> {
    url: &'a T,
}

impl<'a, T, Request> Service<Request> for FooService<'a, T>
where
    T: Debug + Sync,
{
    type Response = &'a T;

    type Error = ();

    type Future<'b>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>
    where
        Self: 'b;

    fn call(&mut self, _req: Request) -> Self::Future<'_> {
        Box::pin(async move {
            // println!("{:?}", self.url);
            Err(())
        })
    }
}

// async fn runner() {
//     let mut v = vec![1, 2, 3];
//     let mut foo = FooService { url: &v };
//     // let rf = &mut foo;
//     // putting stuff in foo and moving it will cause issues unless the result future doesn't use
//     // the spawned structs
//     let s = tokio::spawn(async move {
//         let x = foo.call(()).await;
//     });
// }
