# ax_mean_reversion.py 解读

## 定位

这是 AX live 节点上的均值回归示例，把官方 `BBMeanReversion` 策略接到 `EURUSD-PERP.AX`。

## 主流程

1. 指定 AX 上的欧元兑美元永续合约。
2. 配置 sandbox 数据端、执行端和对账参数。
3. 用 `BBMeanReversionConfig` 设置布林带与 RSI 参数。
4. 将策略加入 `TradingNode` 后运行。

## 关键点

- 和 backtest 版本相比，差别主要在数据来源和运行容器，策略本体不变。
- `LiveRiskEngineConfig(bypass=True)` 只是示例简化，不是生产建议。
- 该脚本很适合作为 AX live 单合约模板。

## 学习价值

适合学习同一官方策略如何从回测脚本迁移到 live 节点。
