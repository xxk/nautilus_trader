# hyperliquid_exec_tester.py 解读

## 定位

这是 Hyperliquid 的执行测试脚本，用于验证订单与持仓链路。

## 主流程

1. 配置 Hyperliquid 客户端。
2. 选择测试合约。
3. 用 `ExecTester` 执行标准动作。
4. 观察执行回报与节点生命周期。

## 关键点

- 这类脚本主要服务于 adapter 验收，而不是策略研究。
- 当你想确认 Hyperliquid 接口是否工作正常时，它比复杂策略更适合先跑。
- 一旦 tester 不稳定，正式策略通常也无法可靠运行。

## 学习价值

适合作为 Hyperliquid 接入的最小执行验证模板。
