# ax_book_imbalance.py 解读

## 作用定位

这份官方示例不是在这里重新实现一个新策略，而是把仓库里已经提供的 `OrderBookImbalance` 官方示例策略，挂到 AX Exchange 的 live 节点上运行。

它的职责只有两件事：

1. 配好 AX 的 live 数据端和交易端。
2. 用一组合适的参数实例化 `OrderBookImbalance`，再交给 `TradingNode` 执行。

因此，真正的策略逻辑主体不在本文件，而在：

- `nautilus_trader/examples/strategies/orderbook_imbalance.py`

这也意味着本文件更像“运行装配脚本”，不是“策略算法文件”。

---

## 文件做了什么

按执行顺序看，这个脚本分成 5 步。

### 1. 选定交易标的

```python
instrument_id = InstrumentId.from_str(f"XAU-PERP.{AX}")
```

这里固定使用 AX 上的黄金永续合约 `XAU-PERP.AX`。

后面所有的行情订阅、对账、下单，都会围绕这个 `instrument_id` 展开。

### 2. 构造 Live 节点配置

核心对象是 `TradingNodeConfig`。

这个配置里包含：

- `trader_id=TESTER-001`：当前交易员实例标识。
- `logging`：日志级别和是否使用 PyO3 日志桥。
- `exec_engine.reconciliation=True`：启动后做执行侧对账。
- `risk_engine.bypass=True`：风险引擎旁路，示例里为了演示方便直接放开。
- `data_clients`：AX 行情客户端配置。
- `exec_clients`：AX 交易客户端配置。
- 多个 `timeout_*`：连接、对账、停机等超时控制。

其中 `data_clients` 和 `exec_clients` 都使用了 `InstrumentProviderConfig(load_all=False, load_ids=...)`，表示：

- 不加载全市场合约。
- 只加载当前策略需要的 `XAU-PERP.AX`。

这个写法很实用，因为 live 场景下通常不希望把无关合约都拉进来。

### 3. 创建 `TradingNode`

```python
node = TradingNode(config=config_node)
```

此时只是生成一个 live 运行节点，还没有真正连接行情和交易端。

### 4. 创建并注册官方策略

```python
strategy = OrderBookImbalance(
    config=OrderBookImbalanceConfig(
        instrument_id=instrument_id,
        max_trade_size=Decimal(1),
        trigger_min_size=1.0,
        trigger_imbalance_ratio=0.10,
        min_seconds_between_triggers=5.0,
        book_type=BookType.L1_MBP,
        use_quote_ticks=True,
        manage_stop=True,
    ),
)
```

这里传入的是策略参数，不是节点参数。

含义如下：

- `max_trade_size=1`：单次最多交易 1 手。
- `trigger_min_size=1.0`：盘口较大一侧的数量至少要超过 1 才可能触发。
- `trigger_imbalance_ratio=0.10`：较小一侧 / 较大一侧 小于 0.10 才触发。
- `min_seconds_between_triggers=5.0`：两次触发之间至少间隔 5 秒。
- `book_type=L1_MBP`：只使用最优一档盘口。
- `use_quote_ticks=True`：不直接订阅深度增量，而是用 `QuoteTick` 驱动策略。
- `manage_stop=True`：当策略停止时，先撤单并平仓，再真正进入停止态。

注意：`manage_stop` 不是 `OrderBookImbalanceConfig` 自己定义的字段，而是继承自策略基类配置 `StrategyConfig`。这也是为什么它可以直接传进来。

### 5. 绑定工厂并启动

```python
node.add_data_client_factory(AX, AxLiveDataClientFactory)
node.add_exec_client_factory(AX, AxLiveExecClientFactory)
node.build()
```

这一步把 AX 适配器真正接到节点上，然后在 `__main__` 里执行：

```python
node.run()
```

退出时无论是否异常，都会执行：

```python
node.dispose()
```

这是一个标准的 live 脚本生命周期写法。

---

## 官方策略本体怎么工作

真正的交易逻辑在 `OrderBookImbalance` 中，可以概括成一句话：

> 当最优买卖盘两侧挂单量极度失衡时，朝“较强的一侧”方向，用对手价发一个 FOK 限价单立即打进去。

### 启动阶段

`on_start()` 做了几件事：

1. 从缓存里取出合约对象。
2. 根据配置决定订阅 `QuoteTick` 还是 `OrderBookDeltas`。
3. 如果 `use_quote_ticks=True`，则内部自己维护一个 `OrderBook(L1_MBP)`。
4. 清空上次触发时间。

本示例选择的是 `use_quote_ticks=True`，所以策略收到报价后会先更新内部 `_book`，再检查是否触发。

### 触发判断

核心判断都在 `check_trigger()`：

1. 先取最优买量 `bid_size` 和最优卖量 `ask_size`。
2. 计算：

```text
ratio = min(bid_size, ask_size) / max(bid_size, ask_size)
```

3. 必须同时满足以下条件才会触发：

- 盘口有效，且 spread 存在。
- 买一量和卖一量都大于 0。
- 较大一侧数量超过 `trigger_min_size`。
- `ratio < trigger_imbalance_ratio`。
- 冷却时间已结束。
- 当前没有 inflight 订单。

### 买卖方向

逻辑非常直接：

- 如果 `bid_size > ask_size`：说明买盘更强，策略选择 `BUY`，并以当前最优卖价成交。
- 如果 `ask_size > bid_size`：说明卖盘更强，策略选择 `SELL`，并以当前最优买价成交。

这是一个典型的“顺着盘口失衡方向吃单”的微观结构示例。

### 下单方式

它发的是 FOK 限价单：

- `time_in_force=FOK`
- `post_only=False`

这代表：

- 要么立刻全部成交。
- 要么直接取消，不接受部分成交慢慢挂着。

这样做的目的很明确：这是一个短线失衡信号示例，不想把订单挂在簿上等待。

### 单次下单量

```python
trade_qty = min(level_size, self._max_qty)
```

也就是：

- 不会超过配置的 `max_trade_size`。
- 也不会超过对手盘当前最优档可成交量。

### 停止行为

策略自己的 `on_stop()` 会：

1. 撤掉该合约全部订单。
2. 平掉该合约全部仓位。

同时本示例又把 `manage_stop=True` 打开了，所以停止时会优先走框架层的自动 market exit 机制。实战理解上，可以把它看成“停机时主动清仓，不留尾巴”。

---

## 这个 live 示例和回测示例的关系

仓库里还有一个对应回测脚本：

- `examples/backtest/architect_ax_book_imbalance.py`

两者复用了同一个官方策略实现 `OrderBookImbalance`，区别只在运行环境：

- 回测脚本：自己构造 `BacktestEngine`，再喂历史行情。
- 当前 live 脚本：构造 `TradingNode`，接 AX 实时行情和交易通道。

因此这份 live 示例最值得学习的地方，不是信号本身，而是：

- 如何把“示例策略”挂到 live 节点。
- 如何同时配置数据端、执行端和合约提供器。
- 如何把 live 实盘接线和策略参数分层管理。

---

## 适合把它当成什么

这份官方示例适合当作以下几类模板：

1. AX live 接线模板。
2. 单合约最小 live 策略模板。
3. `QuoteTick + L1_MBP` 驱动策略模板。
4. `TradingNode + data client + exec client + strategy` 的最小装配模板。

不适合把它当成可直接上线赚钱的策略模板，因为源码已经明确写明：

> 这是一个没有任何 alpha 优势的测试策略，不应直接用于真实资金交易。

---

## 读这份示例时最容易忽略的点

### 1. 本文件不是策略算法本体

如果只看当前文件，很容易误以为策略逻辑就在这里。实际上这里只负责装配，真正逻辑在 `nautilus_trader/examples/strategies/orderbook_imbalance.py`。

### 2. `use_quote_ticks=True` 很关键

这决定了策略不是靠完整盘口深度增量驱动，而是靠最优买卖价和最优买卖量驱动。对接入方来说，数据要求更低。

### 3. `manage_stop=True` 来自基类

它不是这个示例私有字段，而是整个策略配置体系提供的通用停机清仓能力。

### 4. 风险引擎被旁路了

示例里 `LiveRiskEngineConfig(bypass=True)` 是为了方便演示，不代表生产环境建议这么做。

---

## 一句话总结

`ax_book_imbalance.py` 是一个官方 live 装配样例：它把 AX 沙盒连接、单合约加载、官方盘口失衡策略、以及停机清仓控制，组合成了一条最小可运行的实时交易链路。