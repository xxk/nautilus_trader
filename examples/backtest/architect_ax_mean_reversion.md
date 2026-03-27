# architect_ax_mean_reversion.py 解读

## 定位

这是一个 `BacktestEngine` 回测入口，演示如何把官方 `BBMeanReversion` 布林带均值回归策略接到 AX 风格的 `EURUSD-PERP.AX` 合约上。

## 主流程

1. 从本地 TrueFX CSV 读取欧元兑美元 tick 数据。
2. 手工定义 AX 风格的 `PerpetualContract`。
3. 把报价数据整理成 `QuoteTick`，再由内部 1 分钟中间价 Bar 驱动策略。
4. 加载 `BBMeanReversionConfig`，执行回测并打印报告。

## 关键点

- 这份脚本把传统 FX 数据映射成 AX 的永续合约语义，是典型的“代理标的回测”写法。
- 策略信号依赖 Bollinger Band 和 RSI 两组过滤条件。
- 重点不是数据下载，而是“如何将 tick 数据 wrangle 成可直接驱动策略的事件流”。

## 学习价值

适合对照 live 版本理解，同一个官方策略如何在 backtest 与 live 之间保持一致的装配方式。
