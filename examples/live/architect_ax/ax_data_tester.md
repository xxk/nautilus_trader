# ax_data_tester.py 解读

## 定位

这是 AX 的数据链路测试脚本，挂载的是 `DataTester` actor，不是交易 alpha 策略。

## 主流程

1. 连接 AX sandbox 数据端。
2. 选择目标合约和对应 BarType。
3. 配置 `DataTesterConfig` 指定订阅/请求哪些数据。
4. 通过 `add_actor` 运行数据测试器，验证行情链路。

## 学习重点

- 重点是数据订阅、数据请求和事件回流是否正常。
