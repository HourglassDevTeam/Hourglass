use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RequestId(u64);

impl RequestId {
    /// 生成一个新的 `RequestId`，采用雪花算法的变种。
    ///
    /// # 参数
    ///
    /// - `machine_id`: 用于标识生成 ID 的机器，最大值为 1023。
    /// - `counter`: 当前的请求计数器值。
    ///
    /// # 返回
    ///
    /// 返回一个唯一的 `RequestId`。
    pub fn new(machine_id: u16, counter: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        // 生成 RequestId: [timestamp:41 bits] [machine_id:10 bits] [counter:12 bits]
        let id = ((now & 0x1FFFFFFFFFF) << 22) | ((machine_id as u64 & 0x3FF) << 12) | (counter & 0xFFF);

        RequestId(id)
    }

    /// 返回 `RequestId` 的内部 `u64` 值。
    pub fn value(&self) -> u64 {
        self.0
    }
}