use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize,Clone)]
pub enum MessageType{
    Success,Warning,Error
}

#[derive(Debug, serde::Serialize,Clone)]
pub struct MsgType{
    pub msg_type: MessageType,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Proxies{
    pub name:String,
    pub r#type:ProxyType,
    pub enabled:Option<bool>,
    pub local_ip:Option<String>,
    pub local_port:Option<u16>,
    pub remote_port:Option<u16>
}
#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Config{
    pub server_addr:String,
    pub server_port:u16,
    pub proxies:Option<Vec<Proxies>>,
    pub auth:Option<AuthClientConfig>
}

#[derive(Debug,Serialize, Deserialize,Clone,Copy)]
#[serde(rename_all="camelCase")]
pub enum AuthClientMethod{
    Token,Oidc
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AuthClientConfig{
    pub method:AuthClientMethod,
    pub token:Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ProxyType{
    Tcp,Udp,Http,Https
}