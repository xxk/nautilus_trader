# betfair_backtest_orderbook_imbalance.py 解读

## 定位

这是 Betfair 的回测示例，为同一 market 中的多个 selection 批量装配 `OrderBookImbalance`。

## 主流程

1. 创建 Betfair 回测环境和现金账户。
2. 通过测试工具构造 betting instruments。
3. 解析市场更新数据并送入引擎。
4. 为每个 selection 建一个 `OrderBookImbalance` 并统一回测。

## 学习重点

- 这是体育博彩/预测市场版的订单簿失衡策略示例。
- 很适合理解一个 market 下多 instrument 并行策略的装配方式。
