# binance_spot_and_futures_market_maker.py 解读

## 定位

这是一个双策略 live 装配脚本，同时在 Binance 现货和 Binance Futures 上各跑一个 `VolatilityMarketMaker`。

## 主流程

1. 配置两个独立的 client：`BINANCE_SPOT` 与 `BINANCE_FUTURES`。
2. 分别为 `ETHUSDT` 和 `ETHUSDT-PERP` 构造做市策略配置。
3. 将两个策略一起加入同一个 `TradingNode`。
4. 统一 build 和 run。

## 关键点

- 这是多 client、多 venue 语义并存的典型示例。
- `external_order_claims` 与 `client_id` 让两套策略明确各自订单归属。
- futures 端默认指向 testnet，现货端默认是真实现货环境，需要格外注意环境变量与资金安全。

## 学习价值

适合理解一个节点里同时装多个 venue/client 和多个策略的方式。
