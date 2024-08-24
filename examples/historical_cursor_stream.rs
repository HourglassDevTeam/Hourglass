use unilink_execution::sandbox::account::account_market_feed::AccountDataStreams;
use std::{sync::Arc, time::Duration};
use unilink_execution::sandbox::clickhouse_api::queries_operations::ClickHouseClient;
use tokio::{sync::mpsc, time::timeout};
use chrono::{NaiveDate, Duration as ChronoDuration};
#[tokio::main]
async fn main() {
    // 创建 ClickHouse 客户端实例
    let client = Arc::new(ClickHouseClient::new());

    // 定义参数
    let exchange = "binance";
    let instrument = "futures";
    let base = "1000BONK";
    let quote = "USDT";

    // 定义日期范围
    let start_date = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 5, 7).unwrap();

    // 创建 AccountDataStreams 实例
    let mut data_streams = AccountDataStreams::new();

    // 逐日遍历日期范围
    let mut current_date = start_date;
    while current_date <= end_date {
        let date_str = current_date.format("%Y-%m-%d").to_string();

        // 获取游标
        let cursor_result = client.cursor_public_trades(exchange, instrument, &date_str, base, quote).await;

        match cursor_result {
            Ok(mut cursor) => {
                // 创建通道
                let (tx, rx) = mpsc::unbounded_channel();

                // 克隆 date_str 供异步任务使用
                let date_str_clone = date_str.clone();

                // 启动一个任务来从游标读取数据并发送到通道
                let cursor_task = tokio::spawn(async move {
                    loop {
                        match timeout(Duration::from_secs(5), cursor.next()).await {
                            Ok(Ok(Some(trade))) => {
                                if tx.send(trade).is_err() {
                                    // 如果发送失败（例如接收者已关闭），退出循环
                                    eprintln!("[UniLinkExecution] : Failed to send trade, receiver might be closed.");
                                    break;
                                }
                            }
                            Ok(Ok(None)) => {
                                println!("[UniLinkExecution] : Cursor data processing for date {} is complete.", date_str_clone);
                                break;
                            }
                            Ok(Err(_e)) => {
                                eprintln!("[UniLinkExecution] : No data available for date {}. Skipping to next date.", date_str_clone);
                                break;
                            }
                            Err(_) => {
                                eprintln!("[UniLinkExecution] : Timeout while reading cursor for {}", date_str_clone);
                                break;
                            }
                        }
                    }
                });

                // 将接收器添加到 AccountDataStreams
                data_streams.add_stream(format!("{}_{}", exchange, date_str), rx);

                // 等待 `cursor_task` 完成
                if let Err(e) = cursor_task.await {
                    eprintln!("[UniLinkExecution] : Cursor task for {} was aborted: {:?}", date_str, e);
                }

            }
            Err(e) => {
                eprintln!("[UniLinkExecution] : Error fetching trades for {}: {:?}", date_str, e);
            }
        }

        // 更新到下一天
        current_date += ChronoDuration::days(1);
    }

    // 你可以继续使用 data_streams 进行进一步的操作，例如合并、排序等
}
