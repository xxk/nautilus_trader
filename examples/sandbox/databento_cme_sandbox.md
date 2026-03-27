# databento_cme_sandbox.py 解读

## 定位

这是一个 CME 数据 + Sandbox 执行的示例，装配的是 `SimpleQuoterStrategy`。

## 主流程

1. 接入 Databento 实时或近实时数据源。
2. 配置 sandbox 执行环境。
3. 运行 `SimpleQuoterStrategy` 做模拟报价。
4. 观察报价和回报行为。

## 关键点

- 这是 quoter 的实时联调版，不是方向性策略。
- 数据是真实市场数据，执行是模拟的，适合验证报价逻辑。
- 对学习主动挂单和报价策略非常直观。

## 学习价值

适合理解报价型策略从回测走向实时联调的路径。
