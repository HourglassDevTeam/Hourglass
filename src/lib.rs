// 高性能和标准化的交易接口，能够在多个金融场所执行交易。同时提供一个功能丰富的模拟交易所，以协助进行回测和干式交易。通过初始化其关联的ClientExecution实例与交易所进行通信。
// ClientExecution trait 提供了一个统一且简单的语言，用于与交易所进行交互。

#![allow(clippy::type_complexity)]

use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use cerebro_integration::model::Exchange;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    error::ExecutionError,
    universal::{
        balance::TokenBalance,
        event_and_status::ClientAccountEvent,
        order::{Cancelled, Open, Order, OrderId, RequestCancel, RequestOpen},
    },
};

/// 在实时、干运行或模拟执行过程中产生的错误。
pub mod error;
pub mod simulated;
/// 支持在交易所执行操作的核心数据结构。
/// 例如：`Order`（订单）、`Balance`（余额）、`Trade`（交易）等。
pub mod universal;
/// 定义与交易所的通信。每个交易所集成都需要自己的实现。
#[async_trait]
pub trait ClientExecution {
    const CLIENT: ExchangeKind;
    type Config;

    /// 使用提供的[`Self::Config`]和[`ClientAccountEvent`]发送器初始化一个新的[`ClientExecution`]。
    /// 通常包括启动一个异步WebSocket事件循环以从交易所接收[`ClientAccountEvent`]，
    /// 同时返回HTTP客户端`Self`。
    async fn init(config: Self::Config, event_tx: mpsc::UnboundedSender<ClientAccountEvent>) -> Self;
    async fn fetch_orders_open(&self) -> Result<Vec<Order<Open>>, ExecutionError>;
    async fn fetch_balances(&self) -> Result<Vec<TokenBalance>, ExecutionError>;
    async fn open_orders(&self, open_requests: Vec<Order<RequestOpen>>) -> Vec<Result<Order<Open>, ExecutionError>>;
    async fn cancel_orders(&self, cancel_requests: Vec<Order<RequestCancel>>) -> Vec<Result<Order<Cancelled>, ExecutionError>>;

    /// 取消所有账户中的[`Order<Open>`]（未完成订单）。
    async fn cancel_orders_all(&self) -> Result<Vec<Order<Cancelled>>, ExecutionError>;
}

/// Unique identifier for an [`ClientExecution`] implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename = "execution", rename_all = "snake_case")]
pub enum ExchangeKind {
    Simulated,
    Ftx,
}

impl From<ExchangeKind> for Exchange {
    fn from(execution_kind: ExchangeKind) -> Self {
        Exchange::from(execution_kind.as_str())
    }
}

impl Display for ExchangeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ExchangeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            | ExchangeKind::Simulated => "simulated",
            | ExchangeKind::Ftx => "ftx",
        }
    }
}

/// 用于生成测试所需的订单、挂单、交易、的实用工具。
pub mod test_util {
    use cerebro_data::subscription::trade::PublicTrade;
    use cerebro_integration::model::{
        instrument::{kind::InstrumentKind, Instrument},
        Exchange, Side,
    };

    use crate::{
        simulated::exchange::account::order::Orders,
        universal::{
            trade::{SymbolFees, Trade, TradeId},
            event_and_status::ClientOrderId,
        },
        Open, Order, OrderId,
    };

    /// 生成客户端订单集合。
    /// 接收交易编号、买单和卖单向量，返回一个`Orders`实例。
    pub fn generate_client_orders(batch_id: u64, bids: Vec<Order<Open>>, asks: Vec<Order<Open>>) -> Orders {
        Orders {
            order_batch_id: batch_id,
            bids,
            asks,
        }
    }

    /// 创建一个开放状态的挂单。
    /// 接收客户端订单ID、买卖方向、价格、数量和已成交量，
    /// 返回一个`Order<Open>`类型的实例。
    pub fn generate_order_open(cid: ClientOrderId, side: Side, price: f64, quantity: f64, filled: f64) -> Order<Open> {
        Order {
            exchange: Exchange::from("exchange"),
            instrument: Instrument::from(("base", "quote", InstrumentKind::Perpetual)),
            cid,
            side,
            state: Open {
                id: OrderId::from("order_id"),
                price,
                quantity,
                filled_quantity: filled,
            },
        }
    }

    /// 生成一个generate_public_trade记录。
    /// 接收买卖方向、价格和数量，返回一个`PublicTrade`类型的实例。
    pub fn generate_public_trade(side: Side, price: f64, amount: f64) -> PublicTrade {
        PublicTrade {
            id: "trade_id".to_string(),
            price,
            amount,
            side,
        }
    }

    /// 创建一个trade实例。
    /// 接收交易ID、买卖方向、价格、数量和费用，返回一个`Trade`类型的实例。
    pub fn generate_trade(id: TradeId, side: Side, price: f64, quantity: f64, fees: SymbolFees) -> Trade {
        Trade {
            id,
            order_id: OrderId::from("order_id"),
            instrument: Instrument::from(("base", "quote", InstrumentKind::Perpetual)),
            side,
            price,
            quantity,
            fees,
        }
    }
}
