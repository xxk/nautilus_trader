# deribit_option_chain.py 解读

## 定位

这是 Deribit 期权链订阅示例，结构和 Bybit 版本相似，但 venue 与结算货币语义不同。

## 主流程

1. 从缓存中筛选 Deribit 的有效期权合约。
2. 找到最近到期日并优先选择 BTC 结算系列。
3. 构造 `OptionSeriesId` 并订阅 ATM 附近的期权链切片。
4. 在回调中输出各 strike 的报价与 greeks 信息。

## 学习重点

- 适合理解 Deribit 期权链的系列化订阅方式。
- 这是研究期权面板、近月链和 ATM 附近结构的好入口。
