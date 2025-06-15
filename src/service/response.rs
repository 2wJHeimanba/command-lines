use std::pin::Pin;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, service::Service};

#[derive(Clone, Debug)]
pub struct ResponseHtml {}

fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
  Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
}

impl Service<Request<Incoming>> for ResponseHtml {
  type Response = Response<Full<Bytes>>;
  type Error = ::hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, _req: Request<Incoming>) -> Self::Future {
    Box::pin(async { mk_response("hello world".into()) })
  }
}
