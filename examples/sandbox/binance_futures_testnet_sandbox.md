# binance_futures_testnet_sandbox.py 解读

## 定位

这是 Binance Futures Testnet 的沙箱示例，装配 `VolatilityMarketMaker` 进行低风险联调。

## 主流程

1. 连接 Binance Futures testnet 数据源。
2. 执行端改用 sandbox。
3. 配置做市策略并加载目标合约。
4. 运行节点观察策略在实时数据下的模拟执行表现。

## 关键点

- 数据是真实时流，执行是沙箱模拟。
- 很适合在不承担真实成交风险的前提下看做市策略节奏。
- 也是验证 futures 数据链路和策略订阅是否正常的好入口。

## 学习价值

适合作为 Binance Futures 策略从 backtest 向 live 迁移的中间站。
