# fx_ema_cross_bracket_gbpusd_bars_internal.py 解读

## 定位

这是 `EMACrossBracket` 的内部管理版本，演示策略内部如何处理 bracket 生命周期。

## 主流程

1. 构建 FX 回测环境。
2. 加载 GBP/USD Bar 数据。
3. 配置 `EMACrossBracketConfig`。
4. 在策略内部完成入场、止盈和止损联动。

## 关键点

- 与 `external` 版本相比，区别主要在 bracket 的职责边界放在哪里。
- 适合理解策略内部状态机和订单状态协同。
- 对比这两个脚本，可以快速看出官方对“同一交易意图的两种实现边界”。

## 学习价值

适合用来学习策略层自行管理 bracket 逻辑的模式。
