# bybit_ema_cross_with_trailing_stop.py 解读

## 定位

这是 Bybit 版 `EMACrossTrailingStop` live 示例，用 EMA 交叉入场、用 trailing stop 退出。

## 主流程

1. 连接 Bybit 实时节点。
2. 选择目标合约并设置 EMA 参数。
3. 增加 trailing stop 相关配置。
4. 运行节点，观察盈利保护逻辑。

## 关键点

- 与普通 EMA 交叉相比，区别主要体现在退出机制。
- 这是很典型的“同一信号，不同风控包裹”的官方示例。
- 学习时可和 `bybit_ema_cross.py` 对照看，重点比对配置差异。

## 学习价值

适合理解官方如何用最小改动把趋势信号升级成带动态止损的 live 策略。
