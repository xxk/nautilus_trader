# bybit_ema_cross_bracket_algo.py 解读

## 定位

这是 Bybit 版本的 `EMACrossBracketAlgo` 示例，把 EMA 信号、bracket 风控和 `TWAPExecAlgorithm` 执行算法组合到一起。

## 主流程

1. 配置 Bybit live 节点。
2. 用 `EMACrossBracketAlgoConfig` 设置 EMA、ATR、bracket 距离和交易量。
3. 同时实例化 `TWAPExecAlgorithm`。
4. 运行后由策略产生日志和订单，由执行算法负责入场拆单。

## 关键点

- 该示例是“策略 + 算法单”的标准组合模板。
- `LiveRiskEngineConfig(debug=True)` 让示例更利于观察风控链路。
- 学习时要分清：策略负责发意图，执行算法负责怎么成交。

## 学习价值

适合理解 Bybit live 场景下算法执行与策略逻辑的结合方式。
