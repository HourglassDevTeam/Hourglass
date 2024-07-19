use fmt::Display;
use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use tokio::net::TcpStream;
pub use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::MaybeTlsStream;

use crate::{
    common_skeleton::instrument::Instrument,
    data_subscriber::{mapper::SubscriptionMapper, socket_error::SocketError, subscriber::SubKind},
    simulated_exchange::account::account_market_feed::Subscription,
};

pub mod connector;
mod mapper;
pub mod socket_error;
pub mod subscriber;
pub mod validator;
mod websocket;

#[derive(Eq, Hash, PartialEq)]
pub struct SubscriptionId(pub String);
// 为 SubscriptionId 实现 Display trait
impl Display for SubscriptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用 self.0 来访问结构体中的 String
        write!(f, "{}", self.0)
    }
}
pub struct SubscriptionMeta
{
    pub instrument_map: SubscriptionMap<Instrument>,
    /// 存储 WebSocket 消息的向量。
    pub subscriptions: Vec<WsMessage>,
}

/// 用于存储 [`SubscriptionId`] 与泛型类型 `T` 之间的映射。
pub struct SubscriptionMap<T>(pub HashMap<SubscriptionId, T>);

/// 使用 tokio-tungstenite 库的 [WebSocketStream]，可能是 TLS 或非 TLS 的 TcpStream。
pub type WebSocket = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

#[async_trait]
/// `Subscriber` 特征
pub trait Subscriber
{
    /// 关联的订阅映射器类型。
    type SubscriptionMapper: SubscriptionMapper;
    async fn subscribe<Kind>(subscriptions: &[Subscription<Kind>]) -> Result<(WebSocket, SubscriptionMap<Instrument>), SocketError>
        where Kind: SubKind + Send + Sync;
}
