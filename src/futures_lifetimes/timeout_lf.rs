use std::{error::Error, pin::Pin, time::Duration};

use tokio::time::sleep;

use super::single::Service;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Timeout<T> {
    inner: T,
    timeout: Duration,
}

// ===== impl Timeout =====

impl<T> Timeout<T> {
    /// Creates a new [`Timeout`]
    pub const fn new(inner: T, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }

    /// Get a reference to the inner service
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner service
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consume `self`, returning the inner service
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request> + Clone,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>
    where
        Self: 'a;

    fn call<'a>(&'a mut self, request: Request) -> Self::Future<'a> {
        // let clone = self.inner.clone();
        // Box::pin(async move {
        //     let x = clone.call(request);
        //     Ok(x)
        // })
        todo!()
    }
}

#[derive(Clone)]
struct MyService;

impl<Request> Service<Request> for MyService {
    type Response = ();

    type Error = Box<dyn Error + Send + Sync>;

    type Future<'a> =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>;

    fn call<'a>(&'a mut self, req: Request) -> Self::Future<'a> {
        todo!()
    }
}

fn static_check<T>(t: T)
where
    T: 'static,
{
}

async fn runner() {
    let y = tokio::spawn(async move {
        let mut m = MyService;
        let mut t = Timeout::new(m, Duration::from_millis(100));
    })
    .await;
}
