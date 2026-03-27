# databento_data_tester.py 解读

## 定位

这是 Databento 数据源的测试脚本，主要用于验证历史/实时数据接入链路。

## 主流程

1. 配置 Databento 数据客户端。
2. 指定合约和数据类型。
3. 用 `DataTester` actor 发起订阅或请求。
4. 观察节点收到的数据事件。

## 学习重点

- 适合理解 Databento 在 Nautilus 中作为纯数据源时的使用方式。
