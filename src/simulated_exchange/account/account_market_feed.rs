use std::{
    collections::HashMap,
    fmt,
    fmt::Debug,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::Stream;

use crate::{
    common_skeleton::{
        datafeed::{historical::HistoricalFeed, live::LiveFeed},
        instrument::Instrument,
    },
    data_subscriber::{
        connector::Connector,
        socket_error::SocketError,
        subscriber::{Identifier, SubKind},
    },
    simulated_exchange::account::account_market_feed::DataStream::{Historical, Live},
};

// 定义一个数据流别名，用于标识每个数据流。
pub type StreamID = String;

// 定义一个结构体，用于管理多个数据流。
pub struct AccountDataStreams<Event>
    where Event: Clone + Send + Sync + Debug + 'static + Ord /* 约束Event类型必须满足Clone, Send, Sync, 'static特性 */
{
    pub streams: HashMap<StreamID, DataStream<Event>>, // 使用HashMap存储数据流，键为StreamID
}

// 为 AccountDataStreams 实现 Debug trait，方便调试。
impl<Event> Debug for AccountDataStreams<Event>
    where Event: Debug + Clone + Send + Sync + Debug + 'static + Ord /* 约束Event类型必须满足Debug, Clone, Send, Sync, 'static特性 */
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // 打印 AccountDataStreams 的调试信息，包括流的标识符。
        f.debug_struct("AccountMarketStreams").field("streams", &self.streams.keys().collect::<Vec<_>>()).finish()
    }
}
impl<Event> AccountDataStreams<Event> where Event: Clone + Send + Sync + Debug + 'static + Ord
{
    // 添加一个新的方法用于添加WebSocket实时数据流
    pub async fn add_websocket_stream<Exchange, Kind>(&mut self, id: StreamID, subscriptions: &[Subscription<Exchange, Kind>]) -> Result<(), SocketError>
        where Exchange: Connector + Send + Sync,
              Kind: SubKind + Send + Sync,
              Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>
    {
        let stream = DataStream::from_websocket::<Exchange, Kind>(subscriptions).await?;
        self.add_stream(id, stream);
        Ok(())
    }
}

// NOTE this is foreign to this module
#[derive(Debug)]
pub struct Subscription<Exchange, Kind>
{
    pub exchange: Exchange,
    pub instrument: Instrument,
    pub kind: Kind,
}

// 为 AccountDataStreams 实现创建和增减数据流的方法，用于管理数据流。
impl<Event> AccountDataStreams<Event> where Event: Clone + Send + Sync + Debug + 'static + Ord /* 约束Event类型必须满足Clone, Send, Sync, 'static特性 */
{
    // 创建一个新的 AccountDataStreams 实例。
    pub fn new() -> Self
    {
        Self { streams: HashMap::new() }
    }

    // 向AccountDataStreams中添加一个新的数据流。
    pub fn add_stream(&mut self, id: StreamID, stream: DataStream<Event>)
    {
        self.streams.insert(id, stream);
    }

    // 从AccountDataStreams中移除一个数据流。
    pub fn remove_stream(&mut self, id: StreamID)
    {
        self.streams.remove(&id);
    }
}

// 定义一个枚举，表示数据流的类型，可以是实时数据流或历史数据流。
pub enum DataStream<Event>
    where Event: Clone + Send + Sync + Debug + 'static + Ord /* 约束Event类型必须满足Clone, Send, Sync, 'static特性 */
{
    Live(LiveFeed<Event>),             // 实时数据流
    Historical(HistoricalFeed<Event>), // 历史数据流
}

impl<Event> DataStream<Event> where Event: Clone + Send + Sync + Debug + 'static + Ord
{
    pub async fn from_websocket<Exchange, Kind>(subscriptions: &[Subscription<Exchange, Kind>]) -> Result<Self, SocketError>
        where Exchange: Connector + Send + Sync,
              Kind: SubKind + Send + Sync,
              Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>
    {
        let live_feed = LiveFeed::new::<Exchange, Kind>(subscriptions).await?;
        Ok(DataStream::Live(live_feed))
    }
}

// 为DataStream实现Stream trait，使其可以作为异步流处理。
impl<Event> Stream for DataStream<Event> where Event: Clone + Send + Sync + Debug + 'static + Ord /* 约束Event类型必须满足Clone, Send, Sync, 'static特性 */
{
    type Item = Event;

    // 数据流中的元素类型为 Event

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
    {
        // 根据数据流的类型，调用相应的poll_next方法。
        match self.get_mut() {
            | Historical(feed) => Pin::new(&mut feed.receiver).poll_recv(cx),
            | Live(feed) => Pin::new(&mut feed.stream).poll_recv(cx),
        }
    }
}
