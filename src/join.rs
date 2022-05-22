use super::{make_url, cmd, cmd::{primary, secondary}};
use reqwest::{blocking as http, StatusCode};

use eventsource::reqwest::Client;
use reqwest::Url;

pub fn join(code: String) {
  // Join
  cmd::clear();
  println!("{}", primary(&"Connecting...".to_string()));
  let url = make_url(&format!("join/{}", code).to_string());
  let resp = http::get(url.as_str()).unwrap();
  if resp.status() != StatusCode::OK {
    cmd::error(&resp.text().unwrap());
  }

  // Make client
  let url = make_url(&format!("events/{}", code).to_string());
  let mut client = Client::new(Url::parse(&url).unwrap());

  // Play game
  cmd::clear();
}