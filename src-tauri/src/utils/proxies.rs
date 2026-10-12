use std::default;

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
#[derive(Debug,Serialize, Deserialize,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct Config{
    pub server_addr:String,
    pub server_port:u16,
    pub auth:Option<AuthClientConfig>,
    pub proxies:Option<Vec<Proxies>>,
    pub transport:Option<ClientTransportConfig>
}

#[derive(Debug,Serialize, Deserialize,Clone,Copy)]
#[serde(rename_all="camelCase")]
pub enum AuthClientMethod{
    Token,Oidc
}

#[derive(Debug,Serialize, Deserialize,Clone)]
#[serde(rename_all="camelCase")]
pub struct AuthClientConfig{
    // #[serde(default="default_auth_method")]
    pub method:Option<AuthClientMethod>,
    pub additional_scopes:Option<Vec<AdditionalScopes>>,
    pub token:Option<String>,
}

// fn default_auth_method()->AuthClientMethod{
//     AuthClientMethod::Token
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ProxyType{
    Tcp,Udp,Http,Https
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdditionalScopes{
    HeartBeats,NewWorkConns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum Protocol{
    Tcp, Kcp, Quic, Websocket, Wss
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientTransportConfig{
    #[serde(default="default_protocol")]
    pub protocol:Option<Protocol>,
    #[serde(default="default_timeout")]
    pub dial_server_timeout:Option<i32>,
    #[serde(default="default_pool_count")]
    pub pool_count:Option<i32>,
    #[serde(default="default_tcp_mux")]
    pub tcp_mux:Option<bool>,
    #[serde(default="default_tcp_mux_keepalive_interval")]
    pub tcp_mux_keepalive_interval:Option<i32>,
    #[serde(default="default_heartbeat_interval")]
    pub heartbeat_interval:Option<i32>,
    #[serde(default="default_heartbeat_timeout")]
    pub heartbeat_timeout:Option<i32>
}

fn default_protocol()->Option<Protocol>{
    Some(Protocol::Tcp)
}

fn default_timeout()->Option<i32>{
    Some(10)
}

fn default_pool_count()->Option<i32>{
    Some(10)
}

fn default_tcp_mux()->Option<bool>{
    Some(true)
}

fn default_tcp_mux_keepalive_interval()->Option<i32>{
    Some(30)
}

fn default_heartbeat_interval()->Option<i32>{
    Some(30)
}

fn default_heartbeat_timeout()->Option<i32>{
    Some(90)
}
