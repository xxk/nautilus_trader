# fx_ema_cross_bracket_gbpusd_bars_external.py 解读

## 定位

这是一个 `EMACrossBracket` 回测示例，重点演示外部括号单管理方式下的 GBP/USD 交易。

## 主流程

1. 构建回测引擎和 FX 场景。
2. 加载 GBP/USD Bar 数据。
3. 以 `EMACrossBracketConfig` 配置 EMA 信号和 bracket 风控参数。
4. 运行回测，观察入场后止盈止损如何协同工作。

## 关键点

- `external` 版本强调括号单更多依赖引擎/执行层机制。
- 这类示例适合理解 TP/SL 与主订单的联动关系。
- 策略重点已经从单纯信号转向“信号 + 风控订单结构”。

## 学习价值

适合用来理解 bracket order 在官方策略中的外部管理思路。
