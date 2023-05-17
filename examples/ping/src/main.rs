extern crate connect;
extern crate proto;

use connect::{Context, Error, Request, Response};
use proto::example::ping::v1;

struct Server;

impl v1::pingconnect::PingServiceHandler for Server {
    fn ping(
        &self,
        _: Context,
        req: Request<v1::ping::PingRequest>,
    ) -> Result<Response<v1::ping::PingResponse>, Error> {
        let mut res = v1::ping::PingResponse::new();
        res.number = req.msg.number;
        res.text = req.msg.text;

        Ok(Response::new(res))
    }
}

fn main() {
    println!("ping example server implementation")
}
