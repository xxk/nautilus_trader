# binance_spot_futures_sandbox.py 解读

## 定位

这是一个自带 `TestStrategy` 的沙箱示例，同时连现货和合约数据流，用来演示多市场实时数据消费。

## 主流程

1. 同时配置 Binance spot 和 futures 数据源。
2. 执行端使用 sandbox。
3. 运行文件内自定义 `TestStrategy`，消费 Bar、QuoteTick、TradeTick 或自定义数据。
4. 在实时数据环境下做无风险联调。

## 关键点

- 这份脚本更偏教学示例，重点不是 alpha，而是多类型数据事件如何进入策略。
- 因为同时连 spot 和 futures，所以很适合理解多 market 事件流。
- 如果你要写复杂实时订阅逻辑，这个示例非常有参考价值。

## 学习价值

适合理解实时多源数据在一个 Strategy 里的消费方式。
