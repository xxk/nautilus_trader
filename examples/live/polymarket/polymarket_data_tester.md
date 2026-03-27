# polymarket_data_tester.py 解读

## 定位

这是 Polymarket 的数据测试脚本，专注验证预测市场的数据订阅能力。

## 主流程

1. 配置 Polymarket 数据客户端。
2. 选择目标 market/instrument。
3. 用 `DataTester` 订阅或请求数据。
4. 观察事件输出。

## 学习重点

- 适合理解预测市场的数据模型如何接入 Nautilus。
