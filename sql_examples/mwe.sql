WITH second_aggregated AS (
    SELECT
        toStartOfSecond(toDateTime64(timestamp / 1000, 3)) AS second_ts,
        max(timestamp) AS latest_ts
    FROM
        binance_futures_book_snapshot_25.binance_futures_book_snapshot_25_2020_12_19_XRPUSDT
    GROUP BY
        second_ts
)
SELECT
    s.second_ts,
    b.timestamp,
       b.`asks[0].price`,
             b.`asks[0].amount`,
             b.`bids[0].price`,
             b.`bids[0].amount`,
             b.`asks[1].price`,
             b.`asks[1].amount`,
             b.`bids[1].price`,
             b.`bids[1].amount`,
             b.`asks[2].price`,
             b.`asks[2].amount`,
             b.`bids[2].price`,
             b.`bids[2].amount`,
             b.`asks[3].price`,
             b.`asks[3].amount`,
             b.`bids[3].price`,
             b.`bids[3].amount`,
             b.`asks[4].price`,
             b.`asks[4].amount`,
             b.`bids[4].price`,
             b.`bids[4].amount`,
             b.`asks[5].price`,
             b.`asks[5].amount`,
             b.`bids[5].price`,
             b.`bids[5].amount`,
             b.`asks[6].price`,
             b.`asks[6].amount`,
             b.`bids[6].price`,
             b.`bids[6].amount`,
             b.`asks[7].price`,
             b.`asks[7].amount`,
             b.`bids[7].price`,
             b.`bids[7].amount`,
             b.`asks[8].price`,
             b.`asks[8].amount`,
             b.`bids[8].price`,
             b.`bids[8].amount`,
             b.`asks[9].price`,
             b.`asks[9].amount`,
             b.`bids[9].price`,
             b.`bids[9].amount`,
             b.`asks[10].price`,
             b.`asks[10].amount`,
             b.`bids[10].price`,
             b.`bids[10].amount`,
             b.`asks[11].price`,
             b.`asks[11].amount`,
             b.`bids[11].price`,
             b.`bids[11].amount`,
             b.`asks[12].price`,
             b.`asks[12].amount`,
             b.`bids[12].price`,
             b.`bids[12].amount`,
             b.`asks[13].price`,
             b.`asks[13].amount`,
             b.`bids[13].price`,
             b.`bids[13].amount`,
             b.`asks[14].price`,
             b.`asks[14].amount`,
             b.`bids[14].price`,
             b.`bids[14].amount`,
             b.`asks[15].price`,
             b.`asks[15].amount`,
             b.`bids[15].price`,
             b.`bids[15].amount`,
             b.`asks[16].price`,
             b.`asks[16].amount`,
             b.`bids[16].price`,
             b.`bids[16].amount`,
             b.`asks[17].price`,
             b.`asks[17].amount`,
             b.`bids[17].price`,
             b.`bids[17].amount`,
             b.`asks[18].price`,
             b.`asks[18].amount`,
             b.`bids[18].price`,
             b.`bids[18].amount`,
             b.`asks[19].price`,
             b.`asks[19].amount`,
             b.`bids[19].price`,
             b.`bids[19].amount`,
             b.`asks[20].price`,
             b.`asks[20].amount`,
             b.`bids[20].price`,
             b.`bids[20].amount`,
             b.`asks[21].price`,
             b.`asks[21].amount`,
             b.`bids[21].price`,
             b.`bids[21].amount`,
             b.`asks[22].price`,
             b.`asks[22].amount`,
             b.`bids[22].price`,
             b.`bids[22].amount`,
             b.`asks[23].price`,
             b.`asks[23].amount`,
             b.`bids[23].price`,
             b.`bids[23].amount`,
             b.`asks[24].price`,
             b.`asks[24].amount`,
             b.`bids[24].price`,
             b.`bids[24].amount`,
FROM
    second_aggregated s
    INNER JOIN binance_futures_book_snapshot_25.binance_futures_book_snapshot_25_2020_12_19_XRPUSDT b
ON s.latest_ts = b.timestamp
ORDER BY
    s.second_ts ASC;