# ax_exec_tester.py 解读

## 定位

这是 AX 适配器的执行链路测试脚本，挂载的是官方测试策略 `ExecTester`，不是 alpha 策略。

## 主流程

1. 选择 `XAU-PERP.AX` 作为测试合约。
2. 创建 AX sandbox 的 data client 与 exec client。
3. 用 `ExecTesterConfig` 配置开仓、限价/止损单、post-only 等行为。
4. 启动 `TradingNode`，观察下单、撤单、成交和 stop 管理链路。

## 关键点

- `manage_stop=True` 表示停止时先平仓撤单，再真正停掉策略。
- `external_order_claims` 用于把该合约上的外部订单归属到当前策略。
- 这份脚本最适合用来验证 adapter 的执行语义是否打通。

## 学习价值

适合在接 AX 实盘前，先验证 sandbox 执行能力和订单生命周期。
