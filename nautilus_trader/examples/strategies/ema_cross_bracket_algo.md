# ema_cross_bracket_algo.py 解读

## 定位

这是 `EMA + bracket + 执行算法` 的组合策略，比普通 bracket 版本多了一层 execution algo 接入能力。

## 触发数据

- `bars`

## 核心逻辑

1. 先用 EMA 决定方向。
2. 再根据 ATR 计算 bracket 距离。
3. 入场单可接到外部执行算法，例如 TWAP，而不是直接由策略裸发。

## 停止行为

- 停止时统一撤单并平仓。

## 类型

- 趋势
