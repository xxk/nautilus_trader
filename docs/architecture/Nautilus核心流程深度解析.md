# Nautilus 核心流程深度解析

**文档版本**: v1.0  
**目标读者**: 想理解核心执行流程的开发者  
**相关文档**: [架构全景指南](./Nautilus架构全景指南.md) | [模块详解](./Nautilus模块详解指南.md)

---

## 📌 概览

本文档详细解析 Nautilus 框架的**三个核心流程**：

1. **回测流程** - 从历史数据到回测报告
2. **实盘交易流程** - 从市场数据到订单执行  
3. **事件驱动机制** - 所有流程的基础

---

## 🎮 流程 1: 回测执行流程

### 完整流程图

```
【第0步】环境准备
  ├─ 加载历史数据 (CSV/Parquet)
  ├─ 初始化账户 (现金、杠杆等)
  ├─ 实例化策略对象
  └─ 配置订单执行模式 (完全成交/部分成交/滑点)

【第1步】时间推进与数据喂入
  
  模拟时间: 2024-01-01 09:30:00
                    │
        ┌───────────┴───────────┐
        │ 历史数据库              │
        │ AAPL:                 │
        │ [09:30, 100.0]        │
        │ [09:31, 100.5]        │
        │ [09:32, 100.2]        │
        │ ...                   │
        └───────────┬───────────┘
                    │ 按时间顺序逐条
                    ↓
        ┌───────────────────────┐
        │ CacheManager 更新      │
        │ • Bar 缓存            │
        │ • Tick 缓存           │
        │ • 最新价 = 100.0      │
        └───────────┬───────────┘
                    │ 触发
                    ↓
        ┌───────────────────────┐
        │ Strategy.on_bar()     │
        │ • 计算指标            │
        │ • 生成交易信号        │
        └───────────┬───────────┘
                    │


【第2步】策略信号生成

  def on_bar(self, bar: Bar):
      # 1. 获取历史数据 (从缓存)
      bars = self.cache.bars(bar.instrument)
      # bars[-20:] = 最近20根 bar
      
      # 2. 计算指标
      sma = sum(b.close for b in bars[-20:]) / 20
      rsi = self.rsi.value  # 预先计算好的指标
      
      # 3. 生成交易信号
      if sma > 100 and rsi < 30:
          self.buy()  # 生成 BUY 信号
      elif rsi > 70:
          self.sell() # 生成 SELL 信号


【第3步】订单生成与风控检查

  self.buy() 
    ↓
  ┌────────────────────────────────┐
  │ 1. 创建 Order 对象              │
  │    • instrument: AAPL           │
  │    • side: BUY                  │
  │    • quantity: 100              │
  │    • order_type: MARKET         │
  │    • time_in_force: DAY         │
  └────────────┬───────────────────┘
               │
  ┌────────────▼───────────────────┐
  │ 2. 风险检查 (RiskManager)       │
  │    ✓ 现金充足?                 │
  │    ✓ 头寸限制?                 │
  │    ✓ 日亏限制?                 │
  │    ✓ 成交价合理?               │
  └────────────┬───────────────────┘
               │ 通过
               ↓
  ┌────────────────────────────────┐
  │ 3. 分配订单 ID                  │
  │    order_id = "ORDER-00001"    │
  │    timestamp = 09:30:00.123456 │
  └────────────┬───────────────────┘
               │
  ┌────────────▼───────────────────┐
  │ 4. 记录到 OrderManager          │
  │    status: Submitted           │
  └────────────┬───────────────────┘
               │


【第4步】模拟订单执行

  订单: 买 100 AAPL @ 市价
  当前价: 100.0
  
  ┌─────────────────────────────┐
  │ ExecutionAdapter (回测模式)  │
  ├─────────────────────────────┤
  │ • 执行策略: ImmediateExecutor│
  │ • 填充模式: Complete/Partial│
  │ • 滑点:     0.01 (1%)       │
  │ • 佣金:     0.001 (0.1%)    │
  └─────────────────────────────┘
  
  填充计算:
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  原始价:        100.0
  + 滑点:        1.0 (1%)
  = 实际成交价:  101.0
  × 数量:        100
  = 成交金额:    10100.0
  + 佣金:        10.1 (0.1%)
  = 总成本:      10110.1
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  
  账户更新:
  现金: 50000 - 10110.1 = 39889.9
  持仓: +100 AAPL
  
  事件生成: Trade (成交事件)


【第5步】成交事件处理

  Trade 事件
    │
    ├─→ OrderManager
    │    ├─ order.status = Filled
    │    └─ order.fill_price = 101.0
    │
    ├─→ PositionManager
    │    ├─ position.quantity = 100
    │    ├─ position.avg_price = 101.0
    │    └─ position.pnl = 0 (刚入场)
    │
    └─→ Strategy.on_trade()
         ├─ 记录成交
         └─ 更新统计


【第6步】实时指标更新 (并行)

  随着时间推进，每个新 Bar 到达时:
  
  ┌──────────────────────────┐
  │ 1. 指标更新 (实时计算)    │
  │    • SMA 滑动更新        │
  │    • RSI 增量计算        │
  │    • ATR 衰减更新        │
  └──────────────────────────┘
  
  ┌──────────────────────────┐
  │ 2. 持仓 PnL 更新         │
  │    未实现 PnL = 数量 × (现价 - 成本价)
  │    已实现 PnL = ∑成交盈亏
  │    总 PnL = 已实现 + 未实现
  └──────────────────────────┘
  
  ┌──────────────────────────┐
  │ 3. 组合指标更新          │
  │    • 总权益 = 现金 + 持仓市值
  │    • 回撤 = (高水位 - 当前) / 高水位
  │    • 杠杆 = 持仓市值 / 自有资金
  └──────────────────────────┘


【第7步】回测结束与报告生成

  模拟时间: 2024-03-27 16:00:00 (回测结束)
    │
    ├─ 平仓所有持仓 (Mark-to-Market)
    ├─ 计算总盈亏
    └─ 生成统计报告
    
  ┌─────────────────────────────────┐
  │   回测结果摘要                    │
  ├─────────────────────────────────┤
  │ 初始资金:      $50,000           │
  │ 最终权益:      $51,250           │
  │ 总收益:        $1,250 (2.50%)    │
  │ 年化收益:      4.38%             │
  │ 夏普率:        1.45              │
  │ 最大回撤:      -3.2%             │
  │ 胜率:          55%               │
  │ 盈利因子:      1.8               │
  │ 成交次数:      42                │
  └─────────────────────────────────┘
```

### 代码示例

```python
from nautilus_trader.backtest import BacktestEngine, BacktestConfig
from nautilus_trader.model.data import BarType, Bar
from nautilus_trader.model.enums import BarAggregation, PriceType

# 1. 配置
config = BacktestConfig(
    title="Simple EMA Cross",
    engine_seed=12345,
    logging_level="INFO",
)

# 2. 创建引擎
engine = BacktestEngine(config)

# 3. 加载数据
from nautilus_trader.persistence import ParquetDataCatalog

catalog = ParquetDataCatalog("/data/nautilus/")  # 历史数据目录

bars = catalog.load_bars(
    "AAPL.NASDAQ",
    BarType(
        instrument_id="AAPL.NASDAQ",
        aggregation=BarAggregation.MINUTE,
        price_type=PriceType.LAST,
        period=1,
    ),
    start="2024-01-01",
    end="2024-03-27",
)

engine.add_data_source(bars)

# 4. 添加策略
engine.add_strategy(MyEMAStrategy(config))

# 5. 运行
results = engine.run()

# 6. 查看结果
print(results.statistics())  # 打印统计信息
engine.save_reports("/output/report.html")  # 生成 HTML 报告
```

---

## 🌍 流程 2: 实盘交易流程

### 完整流程图

```
【第0步】系统启动与连接

  ┌──────────────────────────────┐
  │ 1. 初始化交易引擎            │
  └──────────────┬───────────────┘
                 │
  ┌──────────────▼───────────────┐
  │ 2. 连接到交易所 Adapter      │
  │    (例: Interactive Brokers) │
  │    • 解析配置文件             │
  │    • 建立 API 连接            │
  │    • TWS/IBGateway 握手       │
  └──────────────┬───────────────┘
                 │
  ┌──────────────▼───────────────┐
  │ 3. 订阅行市数据              │
  │    event_bus.subscribe(      │
  │      "MarketData",           │
  │      instruments=[...],      │
  │    )                         │
  └──────────────┬───────────────┘
                 │
  ┌──────────────▼───────────────┐
  │ 4. 查询账户信息              │
  │    • 现金余额                 │
  │    • 现有持仓                 │
  │    • 额度与杠杆               │
  │    • 待决订单                 │
  └──────────────┬───────────────┘
                 │
  ┌──────────────▼───────────────┐
  │ 5. 启动策略                  │
  │    strategy.on_start()       │
  │    ├─ 加载配置参数            │
  │    ├─ 初始化指标              │
  │    └─ 准备就绪                │
  └──────────────┬───────────────┘
                 │
                ✅ 系统就绪


【第1步】实时行情流入

  交易所 WebSocket
    │
    │ 原始数据: {"bid": 150.10, "ask": 150.15, "size": 1000}
    │
    ↓
  ┌─────────────────────────────────┐
  │ Adapter 解析器                  │
  │ • 格式验证                       │
  │ • 数据转换                       │
  │ • 时间戳统一                     │
  │ • 精度调整                       │
  └─────────────┬───────────────────┘
                │ 标准化数据
                ↓
  ┌─────────────────────────────────┐
  │ Nautilus Tick 对象              │
  │ {                               │
  │   instrument_id: "AAPL.NASDAQ"  │
  │   bid_price: 150.10             │
  │   ask_price: 150.15             │
  │   bid_size: 100                 │
  │   ask_size: 100                 │
  │   ts_init: 1711608600000000000  │ ← 纳秒时间戳
  │ }                               │
  └─────────────┬───────────────────┘
                │
                ↓
  ⚡ 事件总线发布 Tick 事件
     已订阅 Strategy 收到

【每分钟】Bar 生成

  Tick 序列           Bar 合成 (1分钟聚合)
  ─────────────────┐
  150.10 (09:30)  │
  150.12 (09:30)  ├─→ Bar {
  150.15 (09:30)  │     open: 150.10
  150.14 (09:31)  │     high: 150.15
  150.13 (09:31)  │     low: 150.10
  ─────────────────┘    close: 150.13
                        volume: 500
                        ts_init: ...
                      }
                      
                      ⚡ 发布 Bar 事件


【第2步】策略处理行情

def on_tick(self, tick: Tick):
    """实时 Tick 回调 (可选)"""
    # 计算点差
    spread = tick.ask_price - tick.bid_price
    
    # 检查机会
    if spread < 0.01:  # 点差很小
        # 执行交易逻辑
        pass

def on_bar(self, bar: Bar):
    """每分钟 Bar 回调 (更常见)"""
    
    # 1. 获取历史 Bar 数据 (从缓存)
    bars = self.cache.bars(
        bar.instrument,
        bar_type
    )  # 最近 100 根 bar
    
    # 2. 计算指标
    sma_20 = self.sma_20.value
    sma_50 = self.sma_50.value
    rsi_14 = self.rsi.value
    
    # 3. 当前持仓信息
    position = self.portfolio.position(bar.instrument)
    
    # 4. 生成交易信号
    if sma_20 > sma_50 and rsi_14 < 30:
        if not position or position.is_closed:
            self.submit_order(
                BUY_ORDER(quantity=100)
            )
    elif sma_20 < sma_50 and position and position.is_long:
        self.submit_order(
            SELL_ORDER(quantity=position.quantity)
        )


【第3步】订单生成与提交

  self.submit_order(order)
    │
    ├─ 生成 OrderId: "ORDER-xyz"
    ├─ 记录时间戳: 09:35:42.123456
    │
    ↓
  ┌──────────────────────────────────┐
  │ RiskManager 前置风控检查          │
  │ ✓ 余额检查                       │
  │ ✓ 头寸限制                       │
  │ ✓ 日亏限制                       │
  │ ✓ 波动性限制                     │
  └──────────────┬───────────────────┘
                 │ 通过
                 ↓
  ┌──────────────────────────────────┐
  │ 订单加入提交队列                  │
  │ • 异步处理避免阻塞策略            │
  │ • 支持批量提交                    │
  └──────────────┬───────────────────┘
                 │
                 ↓
  异步线程提交到交易所
    └─ REST API POST /submit_order
       + 签名认证
       + 请求压缩


【第4步】订单服务器处理与确认

  交易所服务器
    │
    ├─ 验证订单格式 ✓
    ├─ 验证账户余额 ✓
    ├─ 验证交易权限 ✓
    │
    ├─ 生成服务器 OrderId
    ├─ 加入交易队列
    │
    └─ 返回确认消息
       {"status": "accepted", "order_id": "12345"}
         │
         ↓
  Adapter 解析确认
         │
         ↓
  ⚡ 发布 OrderAccepted 事件
         │
         ↓
  ┌──────────────────────────────────┐
  │ OrderManager 状态更新            │
  │ • status: Submitted → Accepted   │
  │ • 记录服务器 OrderId             │
  │ • 记录确认时间                   │
  └──────────────┬───────────────────┘
                 │
                 ↓
  Strategy.on_order_accepted()
    └─ 确认收到，等待成交


【第5步】订单在交易所的匹配

  交易所撮合引擎
    │
    │ 买盘队列         卖盘队列
    │ ┌─────┐         ┌─────┐
    │ │150.10│150盘   │150.20│
    │ │150.05│200盘   │150.25│
    │ │150.00│500盘   │150.30│
    │ └─────┘         └─────┘
    │
    │ 我们的订单: 买 100 @ 150.15 (LIMIT)
    │
    ├─ 检查卖盘: 最低价 = 150.20
    ├─ 150.15 < 150.20 → 不成交
    └─ 加入买盘队列 (等待)
         │
         └─ 新的卖单来了: 100 @ 150.12
            ├─ 150.12 < 150.15 ✓ 可匹配
            ├─ 成交 100 @ 150.12
            └─ 返回成交消息


【第6步】成交确认与回调

  成交消息: Trade {
    order_id: "12345",
    filled_qty: 100,
    filled_price: 150.12,
    commission: 10.0,
    timestamp: 09:36:15.789123,
  }
    │
    ├─ Adapter 解析
    │
    ↓
  ⚡ 发布 Trade 事件
    │
    ├─→ OrderManager
    │    ├─ order.status = Filled
    │    ├─ order.filled_qty = 100
    │    ├─ order.avg_fill_price = 150.12
    │    └─ order.commission = 10.0
    │
    ├─→ PositionManager
    │    ├─ position.quantity = 100 (新增)
    │    ├─ position.avg_price = 150.12
    │    ├─ position.pnl = 0 (刚入场)
    │    └─ position.entry_timestamp = <ts>
    │
    ├─→ Portfolio
    │    ├─ cash = cash - (100 * 150.12 + 10)
    │    ├─ equity = cash + 持仓市值
    │    └─ free_margin = equity * 0.2 (假设5倍杠杆)
    │
    └─→ Strategy.on_trade()
         ├─ self.logger.info(
         │   f"成交: 买 100 @ 150.12"
         │ )
         └─ 更新自有统计


【第7步】持仓监控 (连续进行)

  每次新 Bar 到达:
    │
    ├─1. 获取最新价
    │    last_price = 150.25
    │
    ├─2. 更新持仓 PnL
    │    unrealized_pnl = (150.25 - 150.12) * 100 = 13
    │    realized_pnl = 0 (还没平)
    │
    ├─3. 回撤监控
    │    if equity < equity_high * 0.9:
    │        alert("回撤超过10%")
    │
    ├─4. 风险指标
    │    margin_used = 100 * 150.25 / equity
    │    if margin_used > 80%:
    │        alert("使用保证金超过80%")
    │
    └─5. 记录日志
         timestamp, price, pnl, margin_used, ...


【第8步】平仓与风险控制

  决策1: 获利出局
  ┌─────────────────┐
  │ if unrealized_pnl > 100:
  │     sell(quantity=100)
  └─────────────────┘
    │
    ├─→ 生成 SELL 订单
    ├─→ 风控检查 (再次)
    ├─→ 提交到交易所
    ├─→ 成交 100 @ 150.35
    │
    └─→ PositionManager
         ├─ position.is_closed = True
         ├─ realized_pnl = (150.35 - 150.12) * 100 - 佣金 = 23
         └─ 持仓历史记录


【第9步】账户更新与清算

  成交后账户:
  ┌──────────────────────────────┐
  │ 初始现金:     $50,000         │
  │ - 买入成本:   $15,012         │
  │ - 买入佣金:   $10             │
  │ = 持有现金:   $34,978         │
  │                              │
  │ 持仓市值:     $15,035         │
  │ 总权益:       $49,988         │
  │ 变化:         -$12 (-0.02%)   │
  │ 已实现盈亏:   +$23            │
  │ 未实现盈亏:   $0 (已平)       │
  └──────────────────────────────┘
```

### 代码示例

```python
from nautilus_trader.trading.strategy import Strategy
from nautilus_trader.model.data import Bar, Tick
from nautilus_trader.model.enums import OrderSide, OrderType, TimeInForce

class LiveTradingStrategy(Strategy):
    """实盘交易策略"""
    
    def on_start(self):
        """策略启动"""
        print(f"策略启动时间: {self.clock.utc_now()}")
        
        # 订阅行情
        self.subscribe_bars("AAPL.NASDAQ", "1-MINUTE-LAST")
        self.subscribe_ticks("AAPL.NASDAQ")
    
    def on_tick(self, tick: Tick):
        """处理 Tick 数据"""
        spread = tick.ask_price - tick.bid_price
        mid_price = (tick.bid_price + tick.ask_price) / 2
        
        # 记录点差
        self.logger.debug(f"点差: {spread:.4f}")
    
    def on_bar(self, bar: Bar):
        """每分钟处理一次"""
        
        # 1. 获取缓存的历史数据
        bars = self.cache.bars(bar.instrument)
        if len(bars) < 50:
            return  # 数据不足
        
        # 2. 计算指标
        sma_20 = sum(b.close for b in bars[-20:]) / 20
        sma_50 = sum(b.close for b in bars[-50:]) / 50
        
        # 3. 检查现有持仓
        position = self.portfolio.position(bar.instrument)
        
        # 4. 订单逻辑
        if sma_20 > sma_50:
            # 趋势向上
            if not position or position.is_closed:
                # 开仓
                order = self._create_order(
                    instrument=bar.instrument,
                    side=OrderSide.BUY,
                    quantity=100,
                    order_type=OrderType.MARKET,
                )
                self.submit_order(order)
                self.logger.info(f"开仓: {order}")
        else:
            # 趋势向下
            if position and position.is_long:
                # 平仓
                order = self._create_order(
                    instrument=bar.instrument,
                    side=OrderSide.SELL,
                    quantity=position.quantity,
                    order_type=OrderType.MARKET,
                )
                self.submit_order(order)
                self.logger.info(f"平仓: {order}")
    
    def on_order_filled(self, event):
        """订单成交时的回调"""
        order = event.order
        
        # 获取成交后的持仓
        position = self.portfolio.position(order.instrument)
        
        log_msg = f"""
        ===== Order Filled =====
        Instrument: {order.instrument}
        Side: {order.side}
        Filled Qty: {event.last_qty}
        Filled Price: {event.last_px}
        
        Position After:
        - Quantity: {position.quantity}
        - Avg Price: {position.avg_price}
        - PnL: {position.pnl}
        """
        self.logger.info(log_msg)
    
    def on_order_rejected(self, event):
        """订单被拒绝"""
        self.logger.warning(f"订单被拒: {event.reason}")
    
    def _create_order(self, instrument, side, quantity, order_type):
        """创建订单基类"""
        return MarketOrder(
            instrument=instrument,
            side=side,
            quantity=quantity,
            init_id=self.order_factory.init_id(),
            ts_init=self.clock.timestamp_ns(),
        ) if order_type == OrderType.MARKET else (
            LimitOrder(
                instrument=instrument,
                side=side,
                quantity=quantity,
                price=self.cache.latest(instrument.id).close,
                time_in_force=TimeInForce.GTC,
                init_id=self.order_factory.init_id(),
                ts_init=self.clock.timestamp_ns(),
            )
        )
```

---

## ⚙️ 流程 3: 事件驱动机制

### 事件总线架构

```
┌──────────────────────────────────────────────────────────────┐
│                       事件总线 (Message Bus)                   │
│                                                              │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ 事件发布方  │  │  事件队列   │  │ 事件消费方  │            │
│  │ Publisher  │→ │  Queue    │→ │ Subscriber │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│                                                              │
│  ☆ 异步非阻塞 (Async)                                         │
│  ☆ 支持一对多 (1:N)                                           │
│  ☆ 事件排序保证 (Order)                                       │
│  ☆ 优先级机制 (Priority)                                      │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### 事件类型与流向

```
事件分类:

┌─ 市场数据事件 (Data Events)
│  ├─ Tick        行情数据
│  ├─ Bar         K线数据
│  ├─ OrderBook   深度数据
│  └─ InstrumentChange  合约变更
│
├─ 订单事件 (Order Events)
│  ├─ OrderSubmitted      订单已提交
│  ├─ OrderAccepted       订单已接受
│  ├─ OrderRejected       订单被拒绝
│  ├─ OrderPartiallyFilled 部分成交
│  ├─ OrderFilled         完全成交
│  ├─ OrderCanceled       订单已取消
│  ├─ OrderExpired        订单已过期
│  └─ OrderTriggered      触发订单
│
├─ 成交事件 (Trade Events)
│  ├─ Trade       成交记录
│  └─ Commission  佣金
│
├─ 账户事件 (Account Events)
│  ├─ AccountState     账户状态
│  ├─ CashAccount      现金账户
│  ├─ MarginAccount    保证金账户
│  └─ ContingencyOrder 管理订单
│
├─ 系统事件 (System Events)
│  ├─ EngineStart      引擎启动
│  ├─ EngineStop       引擎停止
│  ├─ SessionOpen      交易时段开始
│  ├─ SessionClose     交易时段结束
│  └─ HeartBeat        心跳包
│
└─ 自定义事件 (Custom Events)
   └─ 用户定义的事件类型
```

### 事件处理流程

```
【事件生成】

数据源 (Adapter)
  ├─ REST API 返回
  ├─ WebSocket 推送
  └─ 本地定时器
         │
         ↓
    解析 → 转换 → 验证
         │
         ↓
  ┌──────────────────────┐
  │ 事件对象 Event{}      │
  │ • timestamp          │
  │ • event_id           │
  │ • details            │
  └──────────────────────┘


【事件入队】

  事件队列 (Priority Queue)
  
  优先级分类:
  1. 系统事件 (最高)
  2. 订单事件
  3. 成交事件
  4. 市场数据事件 (最低)
  
  ┌───────────┐
  │ Tick      │ ← 新到达的 Tick 事件
  ├───────────┤
  │ OrderFill │
  ├───────────┤
  │ Bar       │
  ├───────────┤
  │ Tick      │
  └───────────┘


【事件分发】

  EventDispatcher
    │
    ├─ 按事件类型分类
    ├─ 查找订阅者列表
    ├─ 调用对应的回调函数
    │
    ├─→ on_tick()
    ├─→ on_bar()
    ├─→ on_order_filled()
    ├─→ on_trade()
    └─→ on_account_state()


【事件处理】

策略回调函数
  │
  ├─ 执行用户代码
  ├─ 可能生成新的事件
  │  └─ submit_order() → OrderSubmitted 事件
  │
  └─ 更新内部状态


【后处理】

  ├─ 记录事件日志
  ├─ 更新性能指标
  ├─ 持久化存储 (可选)
  └─ 清理资源
```

### 关键特性

#### 1. 决定性时间模型

```
┌─────────────────────────────────────────┐
│ 回测 vs 实盘的时间处理                   │
├─────────────────────────────────────────┤
│                                         │
│ 回测 (Deterministic)                    │
│ ◆ 时间由数据驱动                        │
│ ◆ 可重复 (相同数据)                     │
│ ◆ 加速运行 (可以 1000x)                 │
│ ◆ 精度: 纳秒                            │
│                                         │
│ 实盘 (Real-time)                        │
│ ◆ 时间由系统时钟                        │
│ ◆ 不可重复 (市场随机性)                 │
│ ◆ 实时运行                              │
│ ◆ 精度: 纳秒                            │
│                                         │
│ ✓ 同一套代码可在两种模式下运行          │
│ ✓ 执行语义完全相同                      │
│                                         │
└─────────────────────────────────────────┘
```

#### 2. 异步并发处理

```python
# 不同类型的事件可以并发处理

Timeline:
  09:30:01  Tick #1  → on_tick() (线程1)
  09:30:02  Tick #2  → on_tick() (线程2)
  09:31:00  Bar      → on_bar()  (线程3)
  09:31:01  Trade    → on_trade() (线程4)
  
✓ 多个事件同时处理，提高吞吐量
✓ 使用线程池避免阻塞
✓ 保证同一策略的事件顺序
```

#### 3. 优先级与排序

```python
# 事件优先级确保关键事件先执行

优先级队列:
  Level 1 (最高):    OrderFilled, OrderRejected
                     (影响账户状态，必须第一时间处理)
  
  Level 2:          Bar 
                     (用于策略决策)
  
  Level 3 (最低):   Tick
                     (高频数据，可延迟处理)

```

---

## 📊 完整流程事件链

```
┌─────────────────────────────────────────┐
│      完整交易事件链 (Timeline)            │
├─────────────────────────────────────────┤
│                                         │
│ 09:30:00                                │
│ ├─ SessionStart (交易时段开始)          │
│ └─ HeartBeat (系统心跳)                 │
│                                         │
│ 09:30:05                                │
│ ├─ Tick (ask=100.1, bid=100.0)         │
│ ├─ Tick (ask=100.2, bid=100.1)         │
│ ├─ Tick (ask=100.3, bid=100.2)         │
│ └─ ... (更多 Tick 数据)                 │
│                                         │
│ 09:31:00                                │
│ ├─ Bar (open=100.05, high=100.3, ...）  │
│ ├─ on_bar() 回调执行                    │
│ ├─ OrderSubmitted                       │
│ ├─ OrderAccepted                        │
│ └─ ... (可能的更多事件)                 │
│                                         │
│ 09:31:15                                │
│ ├─ Trade (filled=50, price=100.2)      │
│ ├─ PartiallyFilled                      │
│ ├─ on_trade() 回调                      │
│ └─ Position 更新                        │
│                                         │
│ 09:32:00                                │
│ ├─ Bar (新的一分钟)                     │
│ ├─ on_bar() 再次执行                    │
│ ├─ OrderSubmitted (第二个订单)          │
│ └─ ... (链式反应)                       │
│                                         │
│ ... 持续到收盘 ...                      │
│                                         │
│ 16:00:00                                │
│ ├─ SessionClose (交易时段结束)          │
│ ├─ EngineStop (引擎停止)                │
│ └─ 生成最终报告                         │
│                                         │
└─────────────────────────────────────────┘
```

---

## 🎯 常见问题

### Q1: 为什么要事件驱动?

**A**: 
- ✅ 并发高效 (异步非阻塞)
- ✅ 代码清晰 (回调模式)
- ✅ 易于维护 (事件解耦)
- ✅ 易于测试 (可以模拟事件)
- ✅ 扩展灵活 (新增事件类型)

### Q2: 事件顺序如何保证?

**A**: 
- 单一事件队列 (全局)
- 同一策略的事件按顺序执行
- 使用序列化装饰器 (@EventHandler)

### Q3: 延迟有多大?

**A**: 
- Tick 处理: < 1ms (C语言实现的 Rust)
- Bar 生成: < 10ms
- 订单提交: 网络延迟 (通常 10-100ms)

### Q4: 怎样处理事件风暴?

**A**: 
- 背压处理 (backpressure)
- 事件丢弃策略
- 批量处理

---

**下一步**: 阅读 [学习路径与实践指南](./Nautilus学习路径与实践指南.md)

