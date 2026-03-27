# binance_futures_testnet_exec_tester.py 解读

## 定位

这是 Binance 合约测试网的执行测试脚本，用 `ExecTester` 验证 futures adapter。

## 主流程

1. 连接 Binance Futures Testnet。
2. 加载目标永续或合约标的。
3. 配置执行测试策略。
4. 观察合约环境下的订单与持仓回报。

## 关键点

- 合约环境比现货多了仓位与保证金语义。
- 即使仍是 `ExecTester`，也更适合用来验证 futures 专属细节。
- 建议和 spot 版本对照看，理解同一测试策略在不同 venue 语义下的表现。

## 学习价值

适合理解 Binance Futures 执行适配器的最小验证路径。
