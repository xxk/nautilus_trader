# crypto_orderbook_imbalance.py 解读

## 定位

这是一个加密货币订单簿失衡回测脚本，装配的官方策略是 `OrderBookImbalance`。

## 主流程

1. 载入 Binance 的 order book delta 历史数据。
2. 创建回测引擎和交易场景。
3. 配置 `OrderBookImbalanceConfig` 的触发阈值、最小数量与簿类型。
4. 运行回测，观察盘口失衡下的打单行为。

## 关键点

- 这是典型的微观结构策略示例，依赖盘口而不是 K 线。
- 订单通常使用更激进的成交方式，目标是快速响应失衡信号。
- 与 `architect_ax_book_imbalance.py` 的主要差别在于数据源和 venue 语义不同。

## 学习价值

适合用来理解 order book delta 驱动策略与 quote tick 驱动策略的区别。
