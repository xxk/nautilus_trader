# polymarket_exec_tester.py 解读

## 定位

这是 Polymarket 的执行测试脚本，用于验证预测市场场景下的下单链路。

## 主流程

1. 配置 Polymarket 客户端。
2. 选择测试 market/instrument。
3. 用 `ExecTester` 发起标准订单动作。
4. 观察订单、成交和停止过程。

## 关键点

- 重点仍然是 adapter 验证，而不是市场 alpha。
- Polymarket 的市场结构和常规现货/期货不同，所以 tester 更有必要先跑。
- 适合作为接预测市场执行前的基线。

## 学习价值

适合理解 Nautilus 如何把 `ExecTester` 复用到非传统市场。
