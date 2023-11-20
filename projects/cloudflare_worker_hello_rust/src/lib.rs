use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!(
        "Hello, Rustacean! {} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    if !matches!(req.method(), Method::Get) {
        return Response::error("Method Not Allowed", 405);
    }

    Response::ok(format!("Hello, Rustacean! You hit region {} at {} with {}", req.cf().region().unwrap_or("unknown region".into()), req.path(), req.method().to_string()))
}