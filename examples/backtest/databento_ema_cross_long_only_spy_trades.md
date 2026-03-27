# databento_ema_cross_long_only_spy_trades.py 解读

## 定位

这是 `EMACrossLongOnly` 在 SPY 逐笔成交数据上的示例，用来展示同一策略在 trade tick 粒度下的运行方式。

## 主流程

1. 准备 SPY 的 trade tick 历史数据。
2. 创建股票回测环境。
3. 装配 `EMACrossLongOnly`。
4. 运行回测并对比成交粒度下的结果。

## 关键点

- 和 AAPL Bar 版本相比，这里强调的是数据粒度变化，而不是策略类别变化。
- Trade tick 驱动通常更细，更接近真实事件流。
- 同一个策略可以在不同数据粒度下复用，这是 Nautilus 设计的一大重点。

## 学习价值

适合用来理解“同一策略逻辑在 Bar 与 TradeTick 环境下的装配差异”。
