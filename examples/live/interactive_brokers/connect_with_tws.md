# connect_with_tws.py 解读

## 定位

这是 IB 连接本地 TWS 的 live 示例，仍然以 `SubscribeStrategy` 为核心。

## 主流程

1. 指向本地 TWS / IB Gateway 端口。
2. 配置 data client 和合约提供器。
3. 挂载 `SubscribeStrategy`。
4. 运行节点验证数据订阅是否正常。

## 关键点

- 与 docker 版本相比，这里更接近日常桌面调试场景。
- 重点不是下单，而是先把合约和行情订阅链路跑通。
- 如果 TWS 连不上或合约解析异常，这个脚本通常能最先暴露问题。

## 学习价值

适合作为本地 IB/TWS 接入的基准脚本。
