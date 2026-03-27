# interactive_brokers_sandbox.py 解读

## 定位

这是一个 IB 沙箱示例，用 IB 数据流和 sandbox 执行端一起运行 `EMACross`。

## 主流程

1. 从 Parquet catalog 加载目标 IB 合约。
2. 配置 IB 数据客户端和 instrument provider。
3. 为每个合约实例化一个 `EMACross`。
4. 执行端走 sandbox，运行节点进行联调。

## 关键点

- 这不是单标的示例，而是可以按 catalog 中的多个 instrument 批量建策略。
- `subscribe_quote_ticks=True` 说明策略会主动消费报价数据。
- 该脚本非常适合理解 IB 数据源与模拟执行端的组合方式。

## 学习价值

适合作为 IB 策略从数据订阅联调到真实执行前的中间模板。
