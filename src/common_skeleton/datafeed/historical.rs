use crate::common_skeleton::datafeed::{Feed, MarketFeedDistributor};

/// 历史市场事件的 [`Feed`]。
/// MarketFeed 接受一个泛型迭代器[Iter]，并允许用户按需逐个获取历史市场事件。
/// 这种方式适合处理离线数据或在内存中加载整个历史数据集的情况。
#[derive(Debug)]
pub struct HistoricalFeed<Iter, Event>
    where Iter: Iterator<Item = Event>
{
    pub market_iterator: Iter,
}

impl<Iter, Event> MarketFeedDistributor<Event> for HistoricalFeed<Iter, Event> where Iter: Iterator<Item = Event>
{
    fn fetch_next(&mut self) -> Feed<Event>
    {
        self.market_iterator.next().map_or(Feed::Finished, Feed::Next)
    }
}

// HistoricalFeed生成的办法，新建一个接受Event泛型的历史市场事件迭代器
impl<Iter, Event> HistoricalFeed<Iter, Event> where Iter: Iterator<Item = Event>
{
    pub fn initiate<IntoIter>(market_iterator: IntoIter) -> Self
        where IntoIter: IntoIterator<Item = Event, IntoIter = Iter>
    {
        Self { market_iterator: market_iterator.into_iter() }
    }
}
