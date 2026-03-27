# crypto_ema_cross_ethusdt_trailing_stop.py 解读

## 定位

这是一个 `EMACrossTrailingStop` 回测示例，展示 EMA 交叉入场后如何用 trailing stop 做风控退出。

## 主流程

1. 创建回测引擎并加载 ETHUSDT 数据。
2. 配置 EMA 交叉的快慢线参数。
3. 在配置里增加 trailing stop 相关参数。
4. 运行后观察策略如何跟踪利润并动态收紧退出条件。

## 关键点

- 策略核心不是更复杂的入场，而是把退出逻辑显式策略化。
- 适合理解 `EMACross` 家族示例如何逐步叠加执行与风险管理能力。
- 与 `trade_ticks` 版本相比，这个脚本更强调头寸保护。

## 学习价值

适合用来理解“简单信号 + 明确退出规则”在官方示例体系中的组合方式。
