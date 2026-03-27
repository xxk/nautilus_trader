# connect_with_dockerized_gateway.py 解读

## 定位

这是 IB live 示例，演示如何通过 Dockerized IB Gateway 连接并运行 `SubscribeStrategy`。

## 主流程

1. 配置 Docker 化 IB Gateway 连接参数。
2. 设置 IB data client 与 instrument provider。
3. 装配 `SubscribeStrategy` 订阅目标合约数据。
4. 启动节点并观察订阅事件。

## 关键点

- 这份脚本的重点是 IB 连接方式，而不是交易逻辑。
- `SubscribeStrategy` 更像“订阅样板策略”，用于验证数据链路与合约加载。
- 对想把 IB 跑在容器里的场景，这个脚本很有代表性。

## 学习价值

适合理解 IB + Docker Gateway 的最小 live 装配方式。
