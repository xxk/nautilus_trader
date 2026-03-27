# fx_ema_cross_audusd_bars_from_ticks.py 解读

## 定位

这是一个外汇 EMA 交叉回测入口，演示先从 tick 聚合出 Bar，再用 `EMACross` 策略交易 AUD/USD。

## 主流程

1. 准备 AUD/USD tick 数据。
2. 通过引擎内部聚合生成目标 Bar。
3. 以聚合后的 Bar 驱动 `EMACross`。
4. 运行回测并查看结果。

## 关键点

- 重点在于“从 ticks 生成策略真正消费的 bars”。
- 这个示例强调数据工程链路，不只是信号逻辑。
- 与 `fx_ema_cross_audusd_ticks.py` 正好构成对照组。

## 学习价值

适合用来理解 Nautilus 中外部 tick 数据与内部 Bar 聚合的配合方式。
