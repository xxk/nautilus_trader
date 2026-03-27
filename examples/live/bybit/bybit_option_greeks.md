# bybit_option_greeks.py 解读

## 定位

这是 Bybit 期权希腊值订阅示例，核心组件是 `OptionGreeksTester` actor。

## 主流程

1. 从缓存里发现目标 underlying 的 CALL 期权。
2. 按符号顺序选择一部分合约订阅 greeks。
3. 通过 `on_option_greeks` 持续接收 delta、gamma、vega、theta、IV 等数据。
4. 停止时统一取消订阅。

## 学习重点

- 适合理解 exchange-provided greeks 在 Nautilus 中的订阅和回调形式。
- 如果你要做期权监控或波动率研究，这个脚本非常有参考价值。
