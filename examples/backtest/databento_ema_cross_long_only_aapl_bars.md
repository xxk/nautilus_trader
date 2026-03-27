# databento_ema_cross_long_only_aapl_bars.py 解读

## 定位

这是一个股票场景回测脚本，用官方 `EMACrossLongOnly` 在 AAPL 的 Bar 数据上做单边做多测试。

## 主流程

1. 加载 Databento 历史股票 Bar 数据。
2. 创建回测引擎、股票 venue 和资金账户。
3. 构造 `EMACrossLongOnlyConfig`。
4. 运行回测并输出账户、成交和持仓结果。

## 关键点

- `LongOnly` 版本的重点是只做多，不处理做空分支。
- Bar 驱动意味着更适合中低频验证，不是微观结构级别的响应。
- AAPL 版本适合作为股票类最简官方回测模板。

## 学习价值

适合新手先从最简单的“单边股票趋势”示例入门 Nautilus 的回测装配。
