# binance_futures_demo_exec_tester.py 解读

## 定位

这是 Binance 合约 demo 环境的 `ExecTester` 示例。

## 主流程

1. 连接 Binance Futures demo 环境。
2. 指定测试标的并初始化 `ExecTesterConfig`。
3. 运行节点进行执行链路验证。
4. 检查订单、成交、仓位和停止流程。

## 关键点

- 该脚本仍然服务于 adapter 验证，而不是收益回测。
- demo 环境适合先摸清 futures 账户、下单限制和订单回报格式。
- 与 testnet 版本一起看，可以更清楚地区分不同测试环境口径。

## 学习价值

适合作为进入 Binance Futures 实盘前的过渡模板。
