# bitmex_exec_tester.py 解读

## 定位

这是 BitMEX 的执行测试脚本，策略核心是 `ExecTester`。

## 主流程

1. 配置 BitMEX 的 data/exec client。
2. 选择目标合约。
3. 用 `ExecTesterConfig` 生成标准测试动作。
4. 运行节点验证订单生命周期。

## 关键点

- 重点不在 alpha，而在 adapter 是否正确处理 BitMEX 订单语义。
- 非常适合作为排查下单失败、撤单异常、状态回报缺失的基准脚本。
- 如果你要写自己的 BitMEX live 策略，最好先跑通这个例子。

## 学习价值

适合用来理解 BitMEX 适配器的最小执行闭环。
