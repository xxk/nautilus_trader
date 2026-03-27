# dydx_market_maker.py 解读

## 定位

这是 dYdX v4 的官方做市示例，装配的是 `VolatilityMarketMaker`。

## 主流程

1. 配置 dYdX Rust-backed data/exec client。
2. 选择 `ETH-USD-PERP` 作为目标永续市场。
3. 用 `VolatilityMarketMakerConfig` 设置 ATR 参数和交易量。
4. 运行 `TradingNode` 进入实时做市状态。

## 关键点

- 这个示例的重点是 dYdX v4 新适配器的 live 装配方式。
- `VolatilityMarketMaker` 用 ATR 驱动报价宽度，是官方最典型的做市模板之一。
- 环境变量里钱包地址和私钥是关键前置条件。

## 学习价值

适合理解 dYdX 新版适配器与官方做市策略的结合方式。
