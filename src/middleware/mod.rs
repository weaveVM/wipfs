use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;

pub async fn my_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("{} {:?}", req.path(), req.method());
    // pre-processing

    // invoke the wrapped middleware or service
    let res = next.call(req).await?;

    // post-processing

    Ok(res)
}
