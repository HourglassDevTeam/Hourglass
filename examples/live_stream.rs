// 引入所需的模块
// NOTE currently lacking underlying support for Okx Orderbooks from cerebro_data
// NOTE currently lacking proxies for Binance from cerebro_data
use cerebro_data::{
    exchange::okx::Okx,
    streams::Streams,
    subscription::trade::PublicTrades,
};
use cerebro_data::exchange::ExchangeId;
use cerebro_integration::model::instrument::kind::InstrumentKind;
use unilink_execution::common_skeleton::datafeed::live::LiveFeed;

#[tokio::main]
async fn main() {
    let mut streams = Streams::<PublicTrades>::builder()
        // 添加 PublicTrades 流
        .subscribe([
        (Okx::default(), "btc", "usdt", InstrumentKind::Perpetual, PublicTrades),
    ])
        .init()
        .await
        .unwrap();

    let okx_feed = streams
        .select(ExchangeId::Okx)
        .unwrap();

    let mut live_feed = LiveFeed::new(okx_feed);

    while let Some(trade) = live_feed.recv().await {
        println!("MarketEvent<PublicTrade>: {trade:?}");
    }
}
