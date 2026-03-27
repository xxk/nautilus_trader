# architect_ax_book_imbalance.py 解读

## 定位

这是一个 `BacktestEngine` 回测入口脚本，用代理出来的 AX 黄金永续合约 `XAU-PERP.AX` 去回测官方 `OrderBookImbalance` 策略。

## 主流程

1. 手工定义 AX 风格的 `PerpetualContract`。
2. 用 Databento 的 `mbp-1` 数据加载并重写为 `XAU-PERP.AX`。
3. 构造 `OrderBookImbalanceConfig` 并实例化策略。
4. 运行回测，输出账户、成交和持仓报告。

## 关键点

- 这份脚本的重点是“如何把代理数据接成 AX 合约回测”，不是重新实现策略本体。
- 真正的策略逻辑在 `nautilus_trader.examples.strategies.orderbook_imbalance`。
- 由于使用的是顶档盘口数据，所以配置通常配合 `L1_MBP` 和 `use_quote_ticks=True` 理解。

## 学习价值

适合用来理解“自定义合约定义 + 外部历史数据 + 官方微观结构策略”的最小闭环。
