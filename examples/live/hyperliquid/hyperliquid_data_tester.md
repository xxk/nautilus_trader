# hyperliquid_data_tester.py 解读

## 定位

这是 Hyperliquid 的数据测试脚本，作用是验证行情链路而不是执行链路。

## 主流程

1. 配置 Hyperliquid 数据客户端。
2. 选择测试合约。
3. 运行 `DataTester` actor。
4. 检查行情事件是否持续回流。

## 学习重点

- 适合在排查实时订阅问题时作为最小样本使用。
