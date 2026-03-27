# databento_cme_quoter.py 解读

## 定位

这是一个 CME 场景的主动报价回测示例，使用官方 `SimpleQuoterStrategy` 演示最小做市链路。

## 主流程

1. 从 Databento 准备历史期货数据。
2. 创建 `BacktestEngine` 和 CME 对应 Venue。
3. 用 `SimpleQuoterStrategyConfig` 设置报价相关参数。
4. 运行回测并观察报价与成交结果。

## 关键点

- 这是“主动挂单报价”示例，不是方向性 alpha 示例。
- 主要学习点在于报价逻辑与回测环境的连接方式。
- 适合和 `okx_spot_swap_quoter.py` 对照，理解回测版与 live 版 quoter 的差异。

## 学习价值

适合用来理解 Nautilus 中最小 quoter 策略的装配结构。
