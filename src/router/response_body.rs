use hyper::Body;

pub trait ResponseBody {
    fn into_body(self) -> Body;
}

impl ResponseBody for () {
    fn into_body(self) -> Body {
        Body::empty()
    }
}

impl ResponseBody for &'static str {
    fn into_body(self) -> Body {
        Body::from(self)
    }
}

impl ResponseBody for String {
    fn into_body(self) -> Body {
        Body::from(self)
    }
}

impl ResponseBody for Vec<u8> {
    fn into_body(self) -> Body {
        match serde_json::to_string(&self) {
            Ok(json) => Body::from(json),
            Err(e) => {
                eprintln!("serializing failed: {}", e);
                Body::from(e.to_string())
            }
        }
    }
}
