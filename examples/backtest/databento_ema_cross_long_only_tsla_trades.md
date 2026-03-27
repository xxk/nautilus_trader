# databento_ema_cross_long_only_tsla_trades.py 解读

## 定位

这是 `EMACrossLongOnly` 在 TSLA 逐笔成交数据上的官方回测示例。

## 主流程

1. 加载 TSLA 的历史 trade ticks。
2. 创建股票回测引擎与账户环境。
3. 实例化 `EMACrossLongOnlyConfig`。
4. 运行并输出报表。

## 关键点

- 与 SPY 版本相比，主要差异是标的本身，脚本结构几乎一致。
- 这类脚本非常适合作为“换标的不换框架”的样板。
- 核心学习点是策略参数、数据粒度和标的定义之间的解耦。

## 学习价值

适合用来理解官方示例如何把同一策略快速迁移到不同股票标的。
