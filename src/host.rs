use super::{make_url, cmd, cmd::{primary, secondary}};
use reqwest::blocking as http;

use eventsource::reqwest::Client;
use reqwest::Url;

pub fn host() {
  // Get code
  cmd::clear();
  println!("{}", primary(&"Connecting...".to_string()));
  let code = http::get(make_url(&"new".to_string())).unwrap().text().unwrap();
  let url = make_url(&format!("events/{}", code).to_string());
  let mut client = Client::new(Url::parse(&url).unwrap());
  
  // Wait for players
  cmd::clear();
  println!("{}: {}", primary(&"Code".to_string()), code);
  println!("{}", secondary(&"Waiting for players...".to_string()));

  // Wait for Join event
  let ev = client.next().unwrap().unwrap();
  if ev.data != "\"Join\"\n" {
    panic!("invalid event");
  }

  // Play game
  cmd::clear();
}