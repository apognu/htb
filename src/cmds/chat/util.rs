use std::borrow::Cow;

pub fn decode_message(message: &str) -> Cow<'_, str> {
    match htmlescape::decode_html(message) {
        Ok(message) => Cow::Owned(message),
        Err(_) => Cow::Borrowed(message),
    }
}
