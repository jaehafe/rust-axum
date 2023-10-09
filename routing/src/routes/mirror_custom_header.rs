use axum::http::{Request, header:: HeaderMap};

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
  // let message_value = headers.get("x-message").unwrap();
  let message_value = headers.get("Cookie").unwrap();
  let message = message_value.to_str().unwrap().to_owned();

  // println!("message value: {:?}", message_value);
  // println!("message: {}", message);

  message
  // message_value
}