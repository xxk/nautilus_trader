# crypto_ema_cross_ethusdt_trade_ticks.py 解读

## 定位

这是一个加密货币回测入口，使用 `EMACrossTWAP` 官方策略，在 `ETHUSDT` 的逐笔成交数据上做 EMA 交叉信号与 TWAP 执行。

## 主流程

1. 配置 `BacktestEngine`。
2. 加载 ETHUSDT 的 trade ticks 和对应合约定义。
3. 通过 `EMACrossTWAPConfig` 装配信号参数与执行参数。
4. 运行回测并查看报表。

## 关键点

- 与普通 EMA 交叉不同，这里强调的是“信号生成”和 “TWAP 分拆执行”同时出现。
- 数据粒度是逐笔成交，不是 Bar。
- 适合观察成交粒度下策略响应更细、但执行更复杂的特点。

## 学习价值

适合用来理解“Nautilus 如何把 alpha 信号和 execution algo 组装成一个完整策略”。
