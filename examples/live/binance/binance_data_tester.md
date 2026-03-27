# binance_data_tester.py 解读

## 定位

这是 Binance 公共行情数据测试脚本，核心组件是 `DataTester` actor。

## 主流程

1. 选择 spot 或 futures 数据口径。
2. 配置 Binance 数据客户端，不需要 API Key 也能看公共数据。
3. 用 `DataTesterConfig` 指定要订阅的 instrument、book 或 bar。
4. 运行节点观察实时数据是否正常流入。

## 学习重点

- 适合理解“不下单、只验行情”时的最小官方入口。
