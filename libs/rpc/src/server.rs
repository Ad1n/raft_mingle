use bytes::Bytes;
use error::SimpleResult;
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

const NOTFOUND: &[u8] = b"Not Found";
const INDEX: &[u8] = b"Hello from raft";

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn serve_endpoints(
    req: Request<IncomingBody>,
) -> SimpleResult<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index") => Ok(Response::new(full(INDEX))),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(NOTFOUND))
                .unwrap())
        },
    }
}
