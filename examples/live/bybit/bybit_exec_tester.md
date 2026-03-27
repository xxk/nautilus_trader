# bybit_exec_tester.py 解读

## 定位

这是 Bybit 的执行测试脚本，主要用于验证下单、撤单和状态回报链路。

## 主流程

1. 配置 Bybit data/exec client。
2. 选择目标产品类型和测试合约。
3. 用 `ExecTesterConfig` 生成标准订单测试动作。
4. 运行节点观察执行行为。

## 关键点

- 这不是收益策略，而是 adapter 验收脚本。
- 如果你要调 Bybit API 权限、环境变量或 client 配置，这类脚本最先跑。
- 适合排查 post-only、stop、撤单等执行层问题。

## 学习价值

适合把它当成 Bybit 执行能力的冒烟测试入口。
