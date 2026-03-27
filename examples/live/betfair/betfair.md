# betfair.py 解读

## 定位

这是 Betfair 的实时交易示例，针对指定 market 的多个 selection 装配一组 `OrderBookImbalance` 策略实例。

## 主流程

1. 先连接 Betfair 客户端并加载 market instruments。
2. 根据市场里的每个 instrument 构造一个 `OrderBookImbalance`。
3. 配置 Betfair data client、exec client 和 live 对账参数。
4. 运行 `TradingNode`，默认 `dry_run=True`，以免直接下真单。

## 关键点

- 这份脚本不是单标的，而是“按 market 中的多个 selection 批量建策略”。
- `order_id_tag=instrument.selection_id` 用于把 Betfair 的选择项标识带入订单层。
- `dry_run=True` 是非常关键的安全开关，改成 `False` 才会真的提单。

## 学习价值

适合理解 Betfair 这类预测/博彩市场，如何复用订单簿失衡策略模板。
