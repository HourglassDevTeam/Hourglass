// 引入相关模块和结构体。
use cerebro_integration::model::{
    instrument::{symbol::Symbol, Instrument},
    Side,
};
use serde::{Deserialize, Serialize};

// 引入订单ID。
use super::order::OrderId;

/// 标准化 [`Trade`]（交易）模型。
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Trade {
    // 交易ID，由交易所生成，不能假定其唯一性。
    pub id: TradeId,
    // 关联的订单ID。
    pub order_id: OrderId,
    // 交易的工具/仪器。
    pub instrument: Instrument,
    // 交易方向（买入或卖出）。
    pub side: Side,
    // 交易价格。
    pub price: f64,
    // 交易数量。
    pub amount: f64,
    // 交易费用，以符号表示。
    pub fees: SymbolFees, // NOTE 从CilentAccountInfo公共信息中继承
}

/// [`Trade`]  generated by an 交易所. Cannot be assume unique.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct TradeId(pub String);

impl<S> From<S> for TradeId
where
    S: Into<String>,
{
    fn from(id: S) -> Self {
        Self(id.into())
    }
}

/// 以 [`Symbol`]（符号）表示的 [`Trade`]（交易）费用。
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SymbolFees {
    pub symbol: Symbol,
    pub fees: f64,
}

impl SymbolFees {
    /// 构造一个新的 [`SymbolFees`]。
    pub fn new<S>(symbol: S, fees: f64) -> Self
    where
        S: Into<Symbol>,
    {
        Self { symbol: symbol.into(), fees }
    }
}
