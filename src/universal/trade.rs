// 引入相关模块和结构体。
use cerebro_integration::model::{instrument::Instrument};
use serde::{Deserialize, Serialize};
use crate::universal::Side;

/// 标准化 [`Trade`]（交易）模型。
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Trade {
    // 交易ID，由交易所生成，不能假定其唯一性。
    pub id: TradeId,
    // 交易的金融工具。
    pub instrument: Instrument,
    // 交易方向（买入或卖出）。
    pub side: Side,
    // 交易价格。
    pub price: f64,
    // 交易标的的数量。
    pub size: f64,
    // 交易订单数量。
    pub count: i64,
    // 交易费用，以符号表示。
}

/// [`Trade`]  generated by an 交易所. Cannot be assume unique.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct TradeId(pub i64);

impl<S> From<S> for TradeId
where
    S: Into<i64>,
{
    fn from(id: S) -> Self {
        Self(id.into())
    }
}
