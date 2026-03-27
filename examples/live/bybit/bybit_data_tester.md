# bybit_data_tester.py 解读

## 定位

这是 Bybit 的数据测试脚本，目标是验证实时行情与相关数据请求。

## 主流程

1. 配置 Bybit 数据客户端。
2. 选择产品类型和合约。
3. 用 `DataTesterConfig` 开启报价、成交、Bar 或簿订阅。
4. 运行节点观察数据回流。

## 学习重点

- 这是写 Bybit live 策略前最适合先跑的行情基线脚本。
