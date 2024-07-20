use std::{fmt::Debug, pin::Pin, sync::Arc};

use futures::Stream;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::simulated_exchange::load_from_clickhouse::queries_operations::ClickHouseClient;

pub struct HistoricalFeed<Event>
{
    // NOTE ClickHouseClient is opted and hardcoded for simplicity, as a makeshift solution.
    pub database_client: Arc<ClickHouseClient>,
    pub receiver: UnboundedReceiver<Event>,
}

impl<Event> HistoricalFeed<Event> where Event: Clone + Send + Sync + Debug + 'static
{
    pub fn poll_next(&mut self) -> Pin<&mut (dyn Stream<Item = Event> + Send)>
    {
        self.receiver.as_mut()
    }
}
