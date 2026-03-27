# ema_cross.py 解读

## 定位

这是最标准的官方 EMA 双均线交叉策略，也是很多示例的基础策略本体。

## 触发数据

- `bars`
- 可选 `quote ticks`
- 可选 `trade ticks`

## 核心逻辑

1. 用 Bar 更新快 EMA 和慢 EMA。
2. 快 EMA 高于慢 EMA 时做多，快 EMA 低于慢 EMA 时做空或平反仓。
3. 支持在启动时请求历史 bars 预热指标，并可选订阅 quotes/trades 作为调试或辅助观察。

## 停止行为

- 可配置是否退订实时数据。
- 默认会撤单并平仓。

## 类型

- 趋势
