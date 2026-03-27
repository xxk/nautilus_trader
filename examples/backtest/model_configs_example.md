# model_configs_example.py 解读

## 定位

这不是普通交易脚本，而是一个“配置系统演示”示例，重点展示 `ImportableStrategyConfig`、可导入的 fill/fee/latency model 如何装配到 `BacktestNode`。

## 主流程

1. 用字符串路径声明可导入的策略类与配置类。
2. 分别构造可导入的 fill、latency、fee model 配置。
3. 组合多个 `BacktestVenueConfig` 和 `BacktestRunConfig`。
4. 演示 `BacktestNode` 如何从配置驱动整个回测系统。

## 学习重点

- 重点不在信号，而在“声明式配置如何代替硬编码装配”。
- 适合用来理解 Nautilus 的配置驱动运行模式。
