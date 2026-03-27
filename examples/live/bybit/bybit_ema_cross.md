# bybit_ema_cross.py 解读

## 定位

这是 Bybit 上最标准的 live 示例之一，用 `EMACross` 在 `ETHUSDT-LINEAR.BYBIT` 上运行 EMA 交叉。

## 主流程

1. 选择 `LINEAR` 产品类型。
2. 配置 Bybit data client 和 exec client。
3. 通过 `EMACrossConfig` 设置 1 分钟外部 Bar、快慢 EMA 和交易量。
4. 把策略挂到节点并运行。

## 关键点

- 这是“单一信号、最小 live 装配”的典型样板。
- `external_order_claims` 把该合约上的订单归属绑定给当前策略。
- 如果你刚接触 Nautilus live，先读这份脚本通常比读复杂做市脚本更直接。

## 学习价值

适合用来理解 live 节点上的最小官方趋势策略模板。
