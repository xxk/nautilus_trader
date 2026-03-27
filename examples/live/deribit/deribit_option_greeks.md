# deribit_option_greeks.py 解读

## 定位

这是 Deribit 期权希腊值订阅示例，核心组件是 `OptionGreeksTester` actor。

## 主流程

1. 从缓存中筛选目标 underlying 的 CALL 期权。
2. 按符号排序后订阅部分合约的 greeks 流。
3. 在回调里持续打印 delta、gamma、vega、theta、IV 和 underlying price。
4. 停止时取消全部订阅。

## 学习重点

- 这份脚本适合理解 Deribit 期权行情与 greeks 数据的接法。
- 对期权监控、波动率面板和风控研究都很有帮助。
