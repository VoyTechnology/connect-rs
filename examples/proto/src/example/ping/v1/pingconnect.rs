use super::ping::{PingRequest, PingResponse};
use connect::{Context, Error, Request, Response};

pub const PING_SERVICE_PING_PROCEDURE: &str = "/example.ping.v1.PingService/Ping";

pub trait PingServiceHandler {
    fn ping(
        &self,
        ctx: Context,
        req: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Error>;
}
