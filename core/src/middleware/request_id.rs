use salvo::prelude::*;
use salvo::request::id::RequestId;

pub fn request_id_middleware() -> RequestId {
    RequestId::with_uuid()
}
