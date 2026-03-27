# ema_cross_trailing_stop.py 解读

## 定位

这是在 EMA 交叉基础上增加 trailing stop 的版本，重点是退出保护而不是更复杂的信号。

## 触发数据

- `bars`

## 核心逻辑

1. EMA 决定方向和入场。
2. 成交后自动挂出 trailing stop。
3. 价格朝有利方向运行时，止损线随之移动，帮助锁定利润。

## 停止行为

- 停止时撤单并平仓。

## 类型

- 趋势
