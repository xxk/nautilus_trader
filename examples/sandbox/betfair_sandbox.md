# betfair_sandbox.py 解读

## 定位

这是 Betfair 沙箱示例，用 Betfair 实时数据 + SandboxExecutionClient 的组合来运行 `OrderBookImbalance`。

## 主流程

1. 先连接 Betfair 客户端并拉取指定 market 的 instruments。
2. 为 market 中每个 instrument 创建一个 `OrderBookImbalance` 策略。
3. 数据端走 Betfair，执行端走 Sandbox。
4. 运行节点，模拟真实数据下的策略行为。

## 关键点

- 它和 live 版 `betfair.py` 的主要区别在执行端：这里是 sandbox，不是真执行端。
- 为了让 sandbox 能识别合约，脚本会手动把 instruments 写入 cache。
- 很适合作为从回测过渡到实时但仍保留安全边界的中间层。

## 学习价值

适合理解“真实数据 + 模拟执行”的混合演练模式。
