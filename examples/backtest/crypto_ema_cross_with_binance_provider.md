# crypto_ema_cross_with_binance_provider.py 解读

## 定位

这是一个带 Binance Provider 的 EMA 回测示例，使用官方 `EMACrossTrailingStop` 策略演示“通过交易所适配器提供合约与数据”的装配方法。

## 主流程

1. 初始化回测引擎。
2. 通过 Binance provider 获取合约定义与历史数据上下文。
3. 构造 `EMACrossTrailingStopConfig`。
4. 执行回测并输出结果。

## 关键点

- 重点不是策略逻辑本身，而是 provider 接入方式。
- 这种写法更接近真实交易所适配器驱动的数据链路。
- 对照纯测试数据脚本看，会更容易理解 provider 在 Nautilus 里的职责边界。

## 学习价值

适合用来理解“同一策略如何从测试数据切换到真实交易所语义的数据提供器”。
