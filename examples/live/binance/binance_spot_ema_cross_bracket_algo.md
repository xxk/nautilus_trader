# binance_spot_ema_cross_bracket_algo.py 解读

## 定位

这是 Binance 现货 live 示例，把 `EMACrossBracketAlgo` 策略和 `TWAPExecAlgorithm` 一起挂到 `TradingNode` 上。

## 主流程

1. 配置 Binance spot 的数据端和执行端。
2. 用 `EMACrossBracketAlgoConfig` 设置 EMA、ATR 和 bracket 距离。
3. 单独实例化 `TWAPExecAlgorithm` 作为入场执行算法。
4. 将策略与执行算法同时注册到节点后运行。

## 关键点

- 这份脚本不是“只有信号”，而是“信号 + bracket + execution algo”三层组合。
- `entry_exec_algorithm_id=TWAP` 说明入场单不直接裸发，而是交给执行算法分拆。
- 很适合看清 Nautilus 中 strategy 与 exec algorithm 的职责边界。

## 学习价值

适合理解官方示例如何把信号策略和执行算法拼在一起。
