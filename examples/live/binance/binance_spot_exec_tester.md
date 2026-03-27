# binance_spot_exec_tester.py 解读

## 定位

这是 Binance 现货执行测试脚本，核心策略是 `ExecTester`。

## 主流程

1. 选择 `ETHUSDT.BINANCE` 作为测试标的。
2. 配置 Binance spot data client 和 exec client。
3. 用 `ExecTesterConfig` 控制是否启用限价买卖、stop 单和开仓行为。
4. 启动节点后用真实订单生命周期验证 adapter。

## 关键点

- 默认更像“执行链路验收”，不是收益策略。
- `external_order_claims` 用来接管该标的上的订单归属。
- 观察这个脚本时，应重点看订单创建、状态变化和回报链路。

## 学习价值

适合在真正写 Binance 策略前，先验证现货执行链路是否稳定。
