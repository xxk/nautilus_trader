# kraken_exec_tester.py 解读

## 定位

这是 Kraken 的执行测试脚本，主要用来验证订单适配层。

## 主流程

1. 配置 Kraken data/exec client。
2. 选择测试标的。
3. 用 `ExecTester` 触发标准订单动作。
4. 观察节点日志、订单状态和停止流程。

## 关键点

- 用途仍然是 adapter 验证，不是策略收益验证。
- 当你怀疑 Kraken 环境变量、权限或账户状态时，这类脚本最适合先跑。
- 适合作为自定义策略之前的底座校验。

## 学习价值

适合作为 Kraken live 接入的第一步冒烟测试。
