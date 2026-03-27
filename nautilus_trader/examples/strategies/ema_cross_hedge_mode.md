# ema_cross_hedge_mode.py 解读

## 定位

这是 EMA 交叉在对冲模式语义下的一个变体，用来演示特定持仓模式下的趋势策略写法。

## 触发数据

- `bars`
- 可选 `trade ticks`

## 核心逻辑

1. 仍然基于快慢 EMA 判断方向。
2. 与普通 `ema_cross.py` 相比，更强调对冲/持仓模式下的行为差异。
3. 适合对接支持 hedge mode 的 venue 时作为参考。

## 停止行为

- 撤单并平仓。

## 类型

- 趋势
