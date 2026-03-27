# bybit_ema_cross_stop_entry.py 解读

## 定位

这是 `EMACrossStopEntry` 的 Bybit live 示例，核心是“信号触发后，用 stop 单入场”。

## 主流程

1. 选择产品类型和标的。
2. 配置 Bybit 节点。
3. 通过 `EMACrossStopEntryConfig` 设置 EMA、ATR、trailing 参数和触发价口径。
4. 运行并观察 stop entry 与 trailing stop 的联动。

## 关键点

- 这个示例强调的是“入场方式”，不是单纯的方向信号。
- `trigger_type`、`trailing_offset` 和 `trailing_offset_type` 是理解该策略的关键参数。
- 适合用来研究条件触发型订单在 live 环境里的行为。

## 学习价值

适合理解 stop entry、trailing stop 与 EMA 信号如何配合。
