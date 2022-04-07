use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use log::error;
use log::info;
use serde::{Deserialize, Serialize};

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Headers;
use web_sys::{Request, RequestInit, Response};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchError {
    err: Option<String>,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self {
            err: value.as_string(),
        }
    }
}

pub async fn request<A: Serialize, B: for<'a> Deserialize<'a>>(
    verb: &str,
    url: &str,
    body: Option<A>,
) -> Result<B, FetchError> {
    let mut opts = RequestInit::new();

    info!("Request {verb} {url}");

    opts.method(verb);

    if let Some(body) = body {
        let js_string = serde_json::to_string(&body).unwrap();
        let js_value = JsValue::from_serde(&js_string).unwrap();
        opts.body(Some(&js_value));

        let headers = Headers::new().expect("Get header");
        headers.append("Content-Type", "application/json")?;
        opts.headers(&headers);
    }
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().expect("no window available");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let js_value = JsFuture::from(resp.json()?).await?;
    let data = js_value.into_serde().map_err(|e| {
        let msg = format!("Can't parse response: {:?}", e);
        error!("{msg}");
        FetchError { err: Some(msg) }
    })?;
    Ok(data)
}
