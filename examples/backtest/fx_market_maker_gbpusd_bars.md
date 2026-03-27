# fx_market_maker_gbpusd_bars.py 解读

## 定位

这是一个 FX 做市回测示例，装配的是官方 `VolatilityMarketMaker`。

## 主流程

1. 构建回测引擎和 GBP/USD 外汇场景。
2. 准备 Bar 数据与相关利息/成交模型模块。
3. 通过 `VolatilityMarketMakerConfig` 设置 ATR、报价宽度和交易量。
4. 运行并分析做市结果。

## 关键点

- 该示例不预测方向，而是围绕波动率和报价宽度持续挂单。
- ATR 参数通常直接决定报价距离和节奏。
- 回测里还串联了 FX rollover interest 等模块，更接近真实 FX 场景。

## 学习价值

适合用来理解官方做市策略如何与 FX 特有模块一起工作。
