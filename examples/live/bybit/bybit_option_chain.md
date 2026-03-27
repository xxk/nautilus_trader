# bybit_option_chain.py 解读

## 定位

这是 Bybit 期权链订阅示例，核心组件是 `OptionChainTester` actor，不是交易策略。

## 主流程

1. 从缓存里发现目标 underlying 的期权合约。
2. 找到最近到期日并构造 `OptionSeriesId`。
3. 以 ATM 为中心订阅上下若干档 strike 的期权链切片。
4. 周期性打印 `OptionChainSlice` 快照和各 strike 的报价/greeks。

## 学习重点

- 重点是“期权链订阅接口怎么用”，不是下单。
- 适合研究 Bybit 期权数据结构与 ATM 相对 strike 订阅方式。
