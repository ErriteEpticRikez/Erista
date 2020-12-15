#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientHello {
    #[prost(string, tag="1")]
    pub client_version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientHelloAck {
    #[prost(bool, tag="1")]
    pub client_pass: bool,
}
