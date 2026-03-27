# ema_cross_stop_entry.py 解读

## 定位

这是一个把 EMA 信号与 stop entry 结合的趋势策略，强调“怎么入场”而不只是“什么时候入场”。

## 触发数据

- `bars`

## 核心逻辑

1. 用 EMA 决定趋势方向。
2. 用 ATR 和 trailing 参数构造 stop entry / MIT 风格的进场条件。
3. 成交后通过 trailing stop 做动态风控。

## 停止行为

- 停止时撤单并平仓。

## 类型

- 趋势
