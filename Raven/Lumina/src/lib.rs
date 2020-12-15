use std::io::Cursor;
use std::fmt::Write;

use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod client {
    include!("C:\\Users\\bizmi\\Documents\\Hackathon DO\\Hackthon DO\\Rust Crates\\Erista\\Raven\\Lumina\\genFiles\\Lumina.client.rs");
}

pub fn testClientHello(prov_mess: String) -> client::ClientHello
{
  let mut shirt = client::ClientHello::default();
  shirt.client_version = "1.0".to_string();
  shirt
}

pub fn sendClientAck(client_hmessage: client::ClientHello) -> client::ClientHelloAck
{
  let version = "1.0";
  let mut cAckMsg = client::ClientHelloAck::default();
  cAckMsg.client_pass = true;
  cAckMsg
}

pub fn serializeClientAck(client_ack: &client::ClientHelloAck) -> Vec<u8>
{
  let mut buf = Vec::new();
  buf.reserve(client_ack.encoded_len());
  // Unwrap is safe, since we have reserved sufficient capacity in the vector.
  client_ack.encode(&mut buf).unwrap();
  buf
}