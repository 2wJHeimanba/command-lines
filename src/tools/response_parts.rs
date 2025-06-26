use hyper::{HeaderMap, header};

use crate::Result;

pub fn make_response_parts(ext: &str) -> Result<HeaderMap> {
  let mut header = HeaderMap::new();
  match ext {
    "html" => {
      header.append(
        header::CONTENT_TYPE,
        mime::TEXT_HTML.as_ref().parse().unwrap(),
      );
      header.append(
        header::CACHE_CONTROL,
        "max-age=3600, public".parse().unwrap(),
      );
    }
    "ico" => {
      header.insert(header::CONTENT_TYPE, "image/x-icon".parse().unwrap());
      header.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
    }
    _ => {}
  }

  Ok(header)
}
