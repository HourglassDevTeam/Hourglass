use std::fmt::Debug;

use futures::StreamExt;
use mpsc::UnboundedReceiver;
use tokio::sync::mpsc;

use crate::{
    simulated_exchange::account::account_market_feed::Subscription,
};
use crate::common_skeleton::datafeed::event::MarketEvent;

/// Live feed for events.
pub struct LiveFeed<Event>
{
    pub(crate) receiver: UnboundedReceiver<MarketEvent<Event>>,
}

impl<Event> LiveFeed<Event> where Event: Clone + Send + Sync + Debug + 'static
{
    pub fn recv_next(&mut self) -> Option<MarketEvent<Event>>
    {
        // 尝试从接收器中接收事件
        self.receiver.try_recv().ok()
    }
}

impl<Event> LiveFeed<Event> where Event: Clone + Send + Sync + Debug + 'static + Ord
{
    pub async fn new<Exchange, SubscriptionKind>(subscriptions: &[Subscription<Exchange, SubscriptionKind>]) -> Result<Self, SocketError>
        where Exchange: Connector + Send + Sync,
              SubscriptionKind: SubKind + Send + Sync,
              Subscription<Exchange, SubscriptionKind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>
    {
        let (websocket, _instrument_map) = WebSocketSubscriber::subscribe(subscriptions).await?;
        let stream = websocket.map(|msg| Event::parse_ws(msg)).boxed();

        Ok(Self { receiver: stream })
    }
}
