use async_trait::async_trait;
use mpsc::UnboundedSender;
use oneshot::Sender;
use tokio::sync::{mpsc, mpsc::UnboundedReceiver, oneshot};

use crate::{
    common_infrastructure::{
        balance::TokenBalance,
        datafeed::event::MarketEvent,
        order::{Cancelled, Open, Order, Pending},
    },
    sandbox::clickhouse_api::datatype::clickhouse_trade_data::ClickhousePublicTrade,
    AccountEvent, ClientExecution, ExchangeVariant, ExecutionError, RequestCancel, RequestOpen,
};

#[derive(Debug)]
pub struct SandBoxClient
{
    pub local_timestamp: i64,
    pub request_tx: UnboundedSender<SandBoxClientEvent>, // NOTE 这是向模拟交易所端发送信号的发射器。注意指令格式是SandBoxClientEvent
    pub strategy_signal_rx: UnboundedReceiver<SandBoxClientEvent>, // NOTE 这是从策略收取信号的接收器。注意指令格式是SandBoxClientEvent
}

// NOTE 模拟交易所客户端可向模拟交易所发送的命令
// 定义类型别名以简化复杂的类型
type OpenOrderResults = Vec<Result<Order<Pending>, ExecutionError>>;
type CancelOrderResults = Vec<Result<Order<Cancelled>, ExecutionError>>;
type RequestOpenOrders = (Vec<Order<RequestOpen>>, Sender<OpenOrderResults>);
type RequestCancelOrders = (Vec<Order<RequestCancel>>, Sender<CancelOrderResults>);

// 模拟交易所客户端可向模拟交易所发送的命令
#[derive(Debug)]
pub enum SandBoxClientEvent
{
    FetchMarketEvent(MarketEvent<ClickhousePublicTrade>),
    FetchOrdersOpen(Sender<Result<Vec<Order<Open>>, ExecutionError>>),
    FetchBalances(Sender<Result<Vec<TokenBalance>, ExecutionError>>),
    OpenOrders(RequestOpenOrders),
    CancelOrders(RequestCancelOrders),
    CancelOrdersAll(Sender<Result<Vec<Order<Cancelled>>, ExecutionError>>),
}

#[async_trait]
impl ClientExecution for SandBoxClient
{
    // 注意：客户端的类型自然地由交易所决定并与其保持一致。
    const CLIENT_KIND: ExchangeVariant = ExchangeVariant::SandBox;

    // 注意：在我们的场景中，沙盒交易所的“可选”配置参数是一个 UnboundedSender。
    type Config = (UnboundedSender<SandBoxClientEvent>, UnboundedReceiver<SandBoxClientEvent>);

    async fn init(config: Self::Config, _: UnboundedSender<AccountEvent>, local_timestamp: i64) -> Self
    {
        // 从 config 元组中解构出 request_tx 和 request_rx
        let (request_tx, request_rx) = config;

        // 使用 request_tx 和 request_rx 初始化 SandBoxClient
        Self { request_tx,
               strategy_signal_rx: request_rx,
               local_timestamp }
    }

    async fn fetch_orders_open(&self) -> Result<Vec<Order<Open>>, ExecutionError>
    {
        let (response_tx, response_rx) = oneshot::channel();
        // 向模拟交易所发送获取开放订单的请求。
        self.request_tx
            .send(SandBoxClientEvent::FetchOrdersOpen(response_tx))
            .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to send FetchOrdersOpen request");
        // 从模拟交易所接收开放订单的响应。
        response_rx.await
                   .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to receive FetchOrdersOpen response")
    }

    async fn fetch_balances(&self) -> Result<Vec<TokenBalance>, ExecutionError>
    {
        let (response_tx, response_rx) = oneshot::channel();
        // 向模拟交易所发送获取账户余额的请求。
        self.request_tx
            .send(SandBoxClientEvent::FetchBalances(response_tx))
            .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to send FetchBalances request");
        // 从模拟交易所接收账户余额的响应。
        response_rx.await
                   .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to receive FetchBalances response")
    }

    async fn open_orders(&self, open_requests: Vec<Order<RequestOpen>>) -> Vec<Result<Order<Pending>, ExecutionError>>
    {
        let (response_tx, response_rx) = oneshot::channel();
        // 向模拟交易所发送开启订单的请求。
        self.request_tx
            .send(SandBoxClientEvent::OpenOrders((open_requests, response_tx)))
            .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to send OpenOrders request");
        // 从模拟交易所接收开启订单的响应。
        response_rx.await
                   .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to receive OpenOrders response")
    }

    async fn cancel_orders(&self, cancel_requests: Vec<Order<RequestCancel>>) -> Vec<Result<Order<Cancelled>, ExecutionError>>
    {
        let (response_tx, response_rx) = oneshot::channel();
        // 向模拟交易所发送取消订单的请求。
        self.request_tx
            .send(SandBoxClientEvent::CancelOrders((cancel_requests, response_tx)))
            .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to send CancelOrders request");
        // 从模拟交易所接收取消订单的响应。
        response_rx.await
                   .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to receive CancelOrders response")
    }

    async fn cancel_orders_all(&self) -> Result<Vec<Order<Cancelled>>, ExecutionError>
    {
        // 创建一个 oneshot 通道以与模拟交易所通信。
        let (response_tx, response_rx) = oneshot::channel();
        // 向模拟交易所发送取消所有订单的请求。
        self.request_tx
            .send(SandBoxClientEvent::CancelOrdersAll(response_tx))
            .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to send CancelOrdersAll request");
        // 从模拟交易所接收取消所有订单的响应。
        response_rx.await
                   .expect("[UniLinkExecution] : Sandbox exchange is currently offline - Failed to receive CancelOrdersAll response")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;


    #[tokio::test]
    async fn test_fetch_orders_open() {
        // Create the sender and receiver for the request channel
        let (request_tx, mut request_rx) = mpsc::unbounded_channel();

        // Initialize the client with a dummy receiver for strategy signals
        let client = SandBoxClient {
            local_timestamp: 1622547800,
            request_tx: request_tx.clone(),
            strategy_signal_rx: mpsc::unbounded_channel().1, // dummy receiver
        };

        // Spawn a task to invoke the client's fetch_orders_open method
        let client_task = tokio::spawn(async move {
            let orders = client.fetch_orders_open().await.expect("fetch_orders_open failed");
            assert!(orders.is_empty(), "Expected an empty list of orders");
        });

        // Wait for the client to send a FetchOrdersOpen request
        let request_event = request_rx.recv().await.expect("Expected FetchOrdersOpen event");
        if let SandBoxClientEvent::FetchOrdersOpen(tx) = request_event {
            // Use the sender to send a response simulating an empty list of open orders
            let _ = tx.send(Ok(vec![]));

            // The rest of your test code...
        } else {
            panic!("Received unexpected event type");
        }

        // Wait for the client task to complete
        client_task.await.expect("Client task should complete successfully");

        // Test completed
        println!("Test completed");
    }
}
    //
    // #[tokio::test]
    // async fn test_open_orders() {
    //     // 创建一个模拟的 SandBoxClientEvent 发射器和接收器
    //     let (request_tx, mut request_rx) = mpsc::unbounded_channel();
    //     let (response_tx, response_rx) = oneshot::channel();
    //
    //     // 初始化 SandBoxClient
    //     let client = SandBoxClient {
    //         local_timestamp: 1622547800,
    //         request_tx: request_tx.clone(),
    //         strategy_signal_rx: request_rx,
    //     };
    //
    //     // 模拟订单请求
    //     let open_request = Order {
    //         kind: crate::common_infrastructure::order::OrderExecutionType::Limit,
    //         exchange: ExchangeVariant::Binance,
    //         instrument: crate::common_infrastructure::instrument::Instrument::new("BTC", "USDT", crate::common_infrastructure::instrument::kind::InstrumentKind::Perpetual),
    //         client_ts: chrono::Utc::now().timestamp_millis(),
    //         client_order_id: crate::common_infrastructure::event::ClientOrderId(uuid::Uuid::new_v4()),
    //         side: crate::common_infrastructure::Side::Buy,
    //         state: RequestOpen {
    //             reduce_only: false,
    //             price: 50000.0,
    //             size: 1.0,
    //         },
    //     };
    //
    //     // 模拟向客户端发送 OpenOrders 请求
    //     tokio::spawn(async move {
    //         let _ = client.open_orders(vec![open_request]).await;
    //     });
    //
    //     // 模拟从 SandBoxClientEvent 接收器获取 OpenOrders 事件
    //     if let Some(SandBoxClientEvent::OpenOrders((orders, tx))) = request_rx.recv().await {
    //         assert_eq!(orders.len(), 1);
    //         assert_eq!(tx.send(vec![Ok(Order {
    //             kind: orders[0].kind,
    //             exchange: orders[0].exchange,
    //             instrument: orders[0].instrument.clone(),
    //             client_ts: orders[0].client_ts,
    //             client_order_id: orders[0].client_order_id,
    //             side: orders[0].side,
    //             state: Pending {
    //                 reduce_only: orders[0].state.reduce_only,
    //                 price: orders[0].state.price,
    //                 size: orders[0].state.size,
    //                 predicted_ts: chrono::Utc::now().timestamp_millis(),
    //             },
    //         })]), Ok(()));
    //     }
    //
    //     // 验证 OpenOrders 的响应
    //     let result = response_rx.await;
    //     assert!(result.is_ok());
    //     let orders: Vec<Result<Order<Pending>, ExecutionError>> = result.unwrap();
    //     assert_eq!(orders.len(), 1);
    //     assert_eq!(orders[0].as_ref().unwrap().state.price, 50000.0);
    // }
    //
    //
    // #[tokio::test]
    // async fn test_cancel_orders_all() {
    //     // 创建一个模拟的 SandBoxClientEvent 发射器和接收器
    //     let (request_tx, mut request_rx) = mpsc::unbounded_channel();
    //     let (response_tx, response_rx) = oneshot::channel::<Result<Vec<Order<Cancelled>>, ExecutionError>>();
    //
    //     // 初始化 SandBoxClient
    //     let client = SandBoxClient {
    //         local_timestamp: 1622547800,
    //         request_tx: request_tx.clone(),
    //         strategy_signal_rx: request_rx,
    //     };
    //
    //     // 模拟向客户端发送 CancelOrdersAll 请求
    //     tokio::spawn(async move {
    //         let _ = client.cancel_orders_all().await;
    //     });
    //
    //     // 模拟从 SandBoxClientEvent 接收器获取 CancelOrdersAll 事件
    //     if let Some(SandBoxClientEvent::CancelOrdersAll(tx)) = request_rx.recv().await {
    //         assert_eq!(tx.send(Ok(vec![])), Ok(()));
    //     }
    //
    //     // 验证 CancelOrdersAll 的响应
    //     let result = response_rx.await.unwrap();
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), vec![]);
    // }

