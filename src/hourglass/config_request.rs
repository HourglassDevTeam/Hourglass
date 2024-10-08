use crate::{
    common::{
        account_positions::{PositionDirectionMode, PositionMarginMode},
        instrument::Instrument,
        order::identification::client_order_id::ClientOrderId,
        Side,
    },
    Deserialize, Exchange,
};
use serde::Serialize;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct ConfigurationRequest
{
    pub exchange: Exchange,
    pub instrument: Instrument,
    pub timestamp: i64,
    pub cid: Option<ClientOrderId>,
    pub leverage_rate: Option<f64>,
    pub side: Side,
    pub position_margin_mode: Option<PositionMarginMode>,
    pub position_direction_mode: Option<PositionDirectionMode>,
}
