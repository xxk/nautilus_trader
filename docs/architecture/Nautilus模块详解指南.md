# Nautilus 模块详解指南

**文档版本**: v1.0  
**目标读者**: 想深入了解各模块的开发者  
**相关文档**: [架构全景指南](./Nautilus架构全景指南.md)

---

## 📚 目录导航

- [I. 基础层模块](#i-基础层模块)
- [II. 数据处理层](#ii-数据处理层)
- [III. 连接与适配层](#iii-连接与适配层)
- [IV. 执行与交易层](#iv-执行与交易层)
- [V. 分析与风控层](#v-分析与风控层)
- [VI. 工具与测试](#vi-工具与测试)

---

## I. 基础层模块

### 1️⃣ `core` - 核心基础库

**代码量**: 905,188 行 | **文件数**: 2,325 | **地位**: 🔶 最关键

#### 用途
所有其他模块的基础依赖，提供零成本抽象和高效工具。

#### 核心子模块

```
crates/core/src/
├── time/                    时间管理
│   ├── atomic_clock.rs     原子时钟（确定性时间）
│   ├── datetime.rs         日期时间计算
│   └── timer.rs            定时器
│
├── uuid/                    身份标识
│   ├── uuid_v4.rs          随机UUID生成
│   └── ulid.rs             时间序列ID
│
├── collections/             数据结构
│   ├── hash_map.rs         高效HashMap
│   ├── vec.rs              向量优化
│   └── linked_list.rs      链表实现
│
├── math/                    数学计算
│   ├── precision.rs        浮点数精度
│   ├── interpolation.rs    插值算法
│   └── statistics.rs       统计函数
│
├── serialization/           序列化框架
│   ├── serde.rs            序列化特征
│   ├── deserialize.rs      反序列化
│   └── codecs/             编解码器
│
├── string/                  字符串处理
│   ├── stack_str.rs        栈上字符串（避免堆分配）
│   └── formatting.rs       格式化工具
│
└── env/                     环境工具
    ├── os.rs               操作系统检测
    └── config.rs           配置加载
```

#### 关键类型

| 类型 | 用途 | 示例 |
|------|------|------|
| `UnixNanos` | 纳秒时间戳 | 1711000000000000000 |
| `UUID4` | 订单/交易 ID | `550e8400-e29b-41d4-a716-446655440000` |
| `Decimal` | 精确价格/数量 | 150.25 (避免浮点误差) |
| `Price`/`Quantity` | 带单位的数值 | Price(15025 cents) |

#### 使用示例

```rust
use nautilus_core::{
    time::UnixNanos,
    uuid::UUID4,
    math::round_half_up,
    collections::FxHashMap,
};

// 当前时间
let now = UnixNanos::now();

// 生成唯一ID
let order_id = UUID4::new();

// 精确计算
let price = 150.25;
let quantity = 100.5;
let cost = round_half_up(price * quantity, 2);
```

---

### 2️⃣ `common` - 共享工具库

**代码量**: 40K | **用途**: 日志、错误、验证

#### 功能

```
common/
├── errors/              错误处理
│   ├── ValidationError
│   ├── ConfigError
│   └── RuntimeError
│
├── logging/             日志系统
│   ├── Logger            日志门面
│   └── LogLevel          日志级别
│
├── validation/          数据验证
│   ├─ validate_symbol()
│   ├─ validate_price()
│   └─ validate_quantity()
│
└── config/              配置管理
    ├─ ConfigParser
    └─ ConfigValidator
```

#### 错误处理模式

```rust
use nautilus_common::error::ValidationError;

// 创建错误
if quantity <= 0 {
    return Err(ValidationError::InvalidQuantity(quantity));
}

// 对标 Result，可用 ? 操作符
fn validate_order(order: &Order) -> Result<(), ValidationError> {
    validate_symbol(&order.instrument.symbol)?;
    validate_price(order.price)?;
    Ok(())
}
```

---

### 3️⃣ `model` - 数据模型

**代码量**: 80K | **用途**: 所有域模型的定义

#### 核心模型

```
model/
├── types/               基础类型
│   ├── Currency        货币 (USD, EUR, etc.)
│   ├── AssetClass      资产类别 (EQUITY, FUTURE, OPTION)
│   └── Side            方向 (BUY, SELL)
│
├── instrument/          合约定义
│   ├── Instrument       通用合约
│   ├── Equity           股票
│   ├── Future           期货
│   ├── Option           期权
│   └── CryptoPerpetual  加密永续
│
├── orders/              订单相关
│   ├── Order            订单基类
│   ├── OrderStatus      订单状态枚举
│   ├── OrderType        订单类型 (LIMIT/MARKET/etc)
│   ├── TimeInForce      时间有效性 (GTC/IOC/FOK/GTD)
│   └── ContingencyOrder  管理订单 (OCO/OTO/OUO)
│
├── fills/               成交相关
│   ├── OrderFilled      成交数据
│   ├── Trade            交易记录
│   └── PartialFill      部分成交
│
├── positions/           持仓相关
│   ├── Position         持仓对象
│   ├── PositionSide     多空方向
│   └── PositionStatus   持仓状态
│
├── accounting/          账户相关
│   ├── Account          账户对象
│   ├── Balance          余额
│   └── Cash             现金
│
└── data/                行情数据
    ├── Bar              K线数据
    ├── Tick             tick 数据
    ├── OrderBook        深度数据
    └── Instrument Change 合约变更
```

#### 模型关系图

```
Currency ◄─ Instrument ─► AssetClass
         ├─ Symbol
         ├─ Exchange (Venue)
         └─ PriceIncrement
              │
              ▼
          Order (Order)
          ├─ OrderId
          ├─ Instrument
          ├─ Side: {BUY, SELL}
          ├─ Quantity
          ├─ Price
          ├─ OrderType
          ├─ TimeInForce
          └─ Status: OrderStatus
              │
              ├─→ Submitted
              ├─→ Accepted
              ├─→ PartiallyFilled ─→ Trade
              ├─→ Filled ─────────→ Position ─→ PnL
              ├─→ Rejected
              └─→ Canceled

Position
  ├─ Instrument
  ├─ Quantity (current)
  ├─ AvgEntryPrice
  ├─ UnrealizedPnL
  └─ RealizedPnL ◄─ Account
                    ├─ Cash
                    ├─ Equity
                    └─ Margin
```

---

## II. 数据处理层

### 4️⃣ `data` - 数据管理

**代码量**: 75K | **核心职责**: 行情缓存、数据查询、订阅管理

#### 组件架构

```
DataEngine (核心)
  │
  ├─► CacheManager          缓存管理器
  │   ├─ instrument_cache   合约缓存
  │   ├─ bar_cache          K线缓存
  │   ├─ tick_cache         Tick缓存
  │   └─ book_cache         深度缓存
  │
  ├─► DataClient            数据查询客户端
  │   ├─ get_history_bars()
  │   ├─ get_latest_bar()
  │   ├─ get_quote_ticks()
  │   └─ get_trade_ticks()
  │
  └─► SubscriptionManager   订阅管理器
      ├─ subscribe_bars()
      ├─ subscribe_ticks()
      ├─ subscribe_books()
      └─ unsubscribe()
```

#### 数据流

```python
# 订阅行情
engine.subscribe_bars(
    "AAPL.NASDAQ",
    bar_type="1-MINUTE-LAST",
    depth=100  # 保留最近100条
)

# 当新 Bar 到达时
def on_bar(self, bar: Bar):
    # Bar 已经在缓存中
    cache = self.cache.bar("AAPL.NASDAQ", BarType)
    last_100_bars = cache  # 自动维护
    
    # 计算指标
    sma_20 = sum(b.close for b in last_100_bars[-20:]) / 20
```

#### 查询模式

```rust
// 查询历史数据
let bars = data_client.get_history_bars(
    "AAPL.NASDAQ",
    bar_type: "1-DAY-LAST",
    start: datetime(2024, 1, 1),
    end: datetime(2024, 3, 27),
)?;

println!("共 {} 条日K线", bars.len());  // 87条
for bar in bars {
    println!("{} | O:{} H:{} L:{} C:{}", 
        bar.ts, bar.open, bar.high, bar.low, bar.close);
}
```

---

### 5️⃣ `persistence` - 数据持久化

**代码量**: 65K | **用途**: 数据存储、加载、备份

#### 架构

```
PersistenceEngine
  │
  ├─► Database Layer
  │   ├─ RedisStore        实时数据
  │   └─ SQLiteStore       历史数据
  │
  ├─► Catalog              数据目录
  │   ├─ IndexInstruments
  │   ├─ ListAvailableBars
  │   └─ GetDataPath
  │
  └─► Writer               数据写入
      ├─ BatchInsert       批量插入
      ├─ AsyncWrite        异步写入
      └─ FlushBuffer       缓冲区刷新
```

#### 配置示例

```yaml
# persistence.yaml
persistence:
  type: "sqlite"
  path: "/data/nautilus.db"
  
  redis:
    enabled: true
    host: "localhost"
    port: 6379
    ttl: 86400  # 1天过期
    
  backup:
    enabled: true
    frequency: "daily"
    path: "/backup/"
```

#### 数据加载

```python
# 加载历史数据用于回测
catalog = ParquetDataCatalog("/data/nautilus/")

# 列出可用数据
instruments = catalog.list_instruments()
print(instruments)  # ['AAPL.NASDAQ', 'MSFT.NASDAQ', ...]

# 加载 Bar 数据
bars = catalog.load_bars(
    "AAPL.NASDAQ",
    bar_type="1-DAY-LAST",
    start="2024-01-01",
    end="2024-03-27"
)
```

---

## III. 连接与适配层

### 6️⃣ `adapters` - 交易所适配器

**代码量**: 80K | **用途**: 连接不同交易所和数据源

#### 适配器接口

```rust
pub trait DataAdapter {
    /// 连接到数据源
    async fn connect(&mut self) -> Result<()>;
    
    /// 断开连接
    async fn disconnect(&mut self) -> Result<()>;
    
    /// 订阅行情数据
    async fn subscribe(&mut self, instrument_id: &str) -> Result<()>;
    
    /// 处理数据推送
    fn handle_data(&mut self, data: RawData) -> Result<NautilusData>;
}

pub trait ExecutionAdapter {
    /// 提交订单
    async fn submit_order(&mut self, order: &Order) -> Result<()>;
    
    /// 取消订单
    async fn cancel_order(&mut self, order_id: &str) -> Result<()>;
    
    /// 查询订单状态
    async fn query_order(&mut self, order_id: &str) -> Result<Order>;
}
```

#### 内置适配器

| 适配器 | 类别 | 特性 |
|--------|------|------|
| **IB** | 传统市场 | 股票、期货、期权、外汇 |
| **Binance** | 加密交易所 | 现货、交割期货、永续 |
| **Bybit** | 加密交易所 | 现货、U本位/币本位期货 |
| **dYdX** | DEX | 去中心化交易 |
| **Kraken** | 加密交易所 | 现货、期货、杠杆 |

#### 实现示例 (IB)

```python
from nautilus_trader.adapters.interactive_brokers import (
    InteractiveBrokersLiveDataAdapter,
    InteractiveBrokersLiveExecAdapter,
)

# 数据源
ib_data = InteractiveBrokersLiveDataAdapter(config={
    'host': 'localhost',
    'port': 7497,
    'client_id': 1,
})

# 交易执行
ib_exec = InteractiveBrokersLiveExecAdapter(config={
    'host': 'localhost',
    'port': 7497,
    'client_id': 2,
})

engine.add_data_adapter(ib_data)
engine.add_exec_adapter(ib_exec)
```

---

### 7️⃣ `live` - 实盘执行

**代码量**: 70K | **用途**: 实盘交易管理

#### 实盘流程管理

```
EventLoop (主循环)
  │
  ├─► ConnectionManager
  │   ├─ WebSocket 连接
  │   ├─ 心跳检测
  │   ├─ 自动重连
  │   └─ 流量控制
  │
  ├─► OrderManager
  │   ├─ 订单生命周期
  │   ├─ 重试机制
  │   └─ 超时处理
  │
  └─► PositionMonitor
      ├─ 实时持仓
      ├─ 浮动盈亏
      └─ 缘利预警
```

#### 错误处理与恢复

```python
class LiveTradingEngine:
    def on_connection_lost(self):
        """连接断开时的处理"""
        # 1. 停止接收新订单
        self.stop_trading()
        
        # 2. 查询账户和持仓
        account = self.query_account()
        positions = self.query_positions()
        
        # 3. 与本地状态同步
        self.sync_with_broker(account, positions)
        
        # 4. 重新连接
        self.reconnect()
        
        # 5. 恢复交易
        self.resume_trading()
```

---

### 8️⃣ `network` - 网络通信

**代码量**: 55K | **用途**: 异步网络处理

#### 基础设施

```
NetworkEngine (tokio 异步运行时)
  │
  ├─► WebSocket Client
  │   ├─ 连接管理
  │   ├─ 自动重连
  │   ├─ Ping/Pong
  │   └─ 消息序列化
  │
  ├─► HTTP Client
  │   ├─ RESTful 请求
  │   ├─ 对象池
  │   ├─ 重试策略
  │   └─ 超时控制
  │
  └─► Message Queue
      ├─ 事件队列
      ├─ 异步消息处理
      └─ 反压处理 (backpressure)
```

#### 配置示例

```yaml
network:
  websocket:
    timeout_ms: 5000
    buffer_size: 65536
    ping_interval_ms: 30000
    
  http:
    timeout_ms: 10000
    max_retries: 3
    backoff_factor: 2.0
    
  ssl_verify: true
```

---

## IV. 执行与交易层

### 9️⃣ `execution` - 订单执行引擎

**代码量**: 85K | **核心职责**: 订单生命周期管理

#### 执行流程

```
Order Submission
  ↓
┌──────────────────────────┐
│ 1. Validation            │ ← 格式检查、参数校验
├──────────────────────────┤
│ 2. Risk Check            │ ← 风控检查
├──────────────────────────┤
│ 3. Order Assignment      │ ← 分配 OrderId
├──────────────────────────┤
│ 4. Queue to Adapter      │ ← 加入提交队列
├──────────────────────────┤
│ 5. Submit to Venue       │ ← 异步提交
└──────────────────────────┘
         ↓
┌──────────────────────────┐
│ Status: Submitted        │ ← 已提交给交易所
├──────────────────────────┤
│ Await Confirmation       │
└──────────────────────────┘
         ↓
    ┌────┴────┐
    │是否被接受?│
    └────┬────┘
    ┌────┴──────────┐
    │是             │否
    ↓              ↓
Accepted        Rejected
    │              │
    │          ❌ 通知策略
    │          (on_order_rejected)
    │
    ↓
等待成交
    │
    ├─ PartialFilled → Trade → Position Update
    │
    ├─ Filled → Order Close → PnL Calc
    │
    └─ Expired/Canceled → Order Close
```

#### 子模块

```
execution/
├── OrderManager
│   ├─ get_order(order_id)
│   ├─ get_orders_by_status(status)
│   ├─ get_orders_for_instrument(symbol)
│   └─ get_orders_open()
│
├── PositionManager
│   ├─ get_position(instrument)
│   ├─ get_positions()
│   ├─ get_position_pnl(instrument)
│   └─ flatten_position(instrument)
│
├── AccountManager
│   ├─ get_balance()
│   ├─ get_equity()
│   ├─ get_free_balance()
│   └─ get_margin_available()
│
└── ExecutionClient
    ├─ submit_order(order)
    ├─ cancel_order(order_id)
    ├─ modify_order(order_id, new_params)
    └─ logout()
```

#### Python API 示例

```python
class Strategy(Strategy):
    def on_bar(self, bar: Bar) -> None:
        # 获取持仓
        position = self.portfolio.position(bar.instrument)
        
        # 生成订单
        if bar.close > self.sma_20:
            order = MarketOrder(
                instrument=bar.instrument,
                side=OrderSide.BUY,
                quantity=100,
            )
            self.submit_order(order)
        
        # 查询账户
        print(f"账户权益: {self.portfolio.equity()}")
        print(f"可用资金: {self.portfolio.free_balance()}")
    
    def on_order_rejected(self, event: OrderRejected) -> None:
        self.logger.warning(f"订单被拒: {event.order_id}")
```

---

### 🔟 `trading` - 交易策略层

**代码量**: 85K | **用途**: 策略执行与信号处理

#### 交易引擎架构

```
TradingEngine
  │
  ├─► EventDispatcher       事件分发
  │   ├─ on_instrument_change
  │   ├─ on_bar()
  │   ├─ on_tick()
  │   ├─ on_order_filled()
  │   └─ on_order_rejected()
  │
  ├─► StrategyManager       策略管理
  │   ├─ load_strategy()
  │   ├─ start_strategy()
  │   ├─ stop_strategy()
  │   └─ get_strategy_status()
  │
  ├─► Portfolio             投资组合
  │   ├─ portfolio.positions()
  │   ├─ portfolio.equity()
  │   ├─ portfolio.margin_used()
  │   └─ portfolio.unrealized_pnl()
  │
  └─► Cache                 实时缓存
      ├─ cache.bar()
      ├─ cache.tick()
      ├─ cache.order()
      └─ cache.position()
```

#### 策略基类

```python
from nautilus_trader.trading.strategy import Strategy
from nautilus_trader.core.data import Bar, Tick

class MyStrategy(Strategy):
    """
    策略必须实现以下方法:
    - on_bar()      : 处理 K线数据
    - on_tick()     : 处理 Tick 数据
    - on_start()    : 策略启动初始化
    - on_stop()     : 策略停止清理
    """
    
    def __init__(self, config):
        super().__init__(config)
        self.fast_period = 20
        self.slow_period = 50
    
    def on_start(self):
        """策略启动时调用"""
        self.subscribe_bars("AAPL.NASDAQ", "1-MINUTE-LAST")
        self.subscribe_bars("AAPL.NASDAQ", "1-DAY-LAST")
    
    def on_bar(self, bar: Bar):
        """每当新 K线生成时调用"""
        # 获取缓存的历史数据
        bars = self.cache.bars(bar.instrument)
        
        # 简单判断 (示例)
        fast_ma = sum(b.close for b in bars[-self.fast_period:]) / self.fast_period
        slow_ma = sum(b.close for b in bars[-self.slow_period:]) / self.slow_period
        
        # 生成信号
        if fast_ma > slow_ma:
            self.buy()
        elif fast_ma < slow_ma:
            self.sell()
    
    def on_order_filled(self, event):
        """订单成交时调用"""
        filled_qty = event.last_qty
        filled_price = event.last_px
        self.logger.info(f"成交: {filled_qty} @ {filled_price}")
    
    def buy(self):
        order = MarketOrder(
            instrument=...,
            side=OrderSide.BUY,
            quantity=100,
        )
        self.submit_order(order)
    
    def sell(self):
        order = MarketOrder(
            instrument=...,
            side=OrderSide.SELL,
            quantity=100,
        )
        self.submit_order(order)
```

---

## V. 分析与风控层

### 1️⃣1️⃣ `analysis` - 实时分析

**代码量**: 70K | **用途**: 统计分析、回测报告

#### 分析模块

```
analysis/
├── performance/           性能分析
│   ├─ PnL 计算
│   ├─ Sharpe Ratio
│   ├─ Maximum Drawdown
│   ├─ Win Rate
│   └─ Profit Factor
│
├── statistics/           统计指标
│   ├─ Return Analysis
│   ├─ Volatility
│   ├─ Correlation
│   └─ Distribution
│
└── reporting/            报告生成
    ├─ HTML Report
    ├─ Performance Summary
    ├─ Trade Analysis
    └─ Risk Analysis
```

#### Python API

```python
from nautilus_trader.analysis import PerformanceAnalyzer

# 创建分析器
analyzer = PerformanceAnalyzer()

# 计算性能指标
stats = analyzer.calculate_statistics(trades)
print(f"总收益: {stats.total_return:.2%}")
print(f"年化收益: {stats.annual_return:.2%}")
print(f"夏普率: {stats.sharpe_ratio:.2f}")
print(f"最大回撤: {stats.max_drawdown:.2%}")
print(f"胜率: {stats.win_rate:.2%}")
print(f"盈利因子: {stats.profit_factor:.2f}")
```

---

### 1️⃣2️⃣ `indicators` - 技术指标库

**代码量**: 120K | **用途**: 120+ 预定义指标

#### 指标分类

```
缓动指标 (Trend)
  ├─ SMA (Simple Moving Average)
  ├─ EMA (Exponential Moving Average)
  ├─ DEMA (Double EMA)
  ├─ TEMA (Triple EMA)
  ├─ WMA (Weighted MA)
  └─ KMA (Kaufman Adaptive MA)

动量指标 (Momentum)
  ├─ RSI (Relative Strength Index)
  ├─ MACD (Moving Average Convergence Divergence)
  ├─ Stochastic
  ├─ CCI (Commodity Channel Index)
  ├─ ROC (Rate of Change)
  └─ Momentum

波动率指标 (Volatility)
  ├─ ATR (Average True Range)
  ├─ Bollinger Bands
  ├─ Standard Deviation
  ├─ Historical Volatility
  └─ Keltner Channel

成交量指标 (Volume)
  ├─ OBV (On Balance Volume)
  ├─ VWAP (Volume Weighted Average Price)
  ├─ CMF (Chaikin Money Flow)
  ├─ MFI (Money Flow Index)
  └─ Accumulation/Distribution

趋势指标 (Trend)
  ├─ ADX (Average Directional Index)
  ├─ Ichimoku
  ├─ Alligator
  └─ Zigzag
```

#### 使用示例

```python
from nautilus_trader.indicators import (
    SimpleMovingAverage, 
    RelativeStrengthIndex,
    BollingerBands,
)

class MyStrategy(Strategy):
    def __init__(self, config):
        super().__init__(config)
        
        # 创建指标
        self.sma_20 = SimpleMovingAverage(20)
        self.sma_50 = SimpleMovingAverage(50)
        self.rsi = RelativeStrengthIndex(14)
        self.bb = BollingerBands(20, 2.0)
    
    def on_bar(self, bar: Bar):
        # 更新指标
        self.sma_20.update(bar)
        self.sma_50.update(bar)
        self.rsi.update(bar)
        self.bb.update(bar)
        
        # 获取指标值
        sma20_value = self.sma_20.value  # None 如果未准备好
        if sma20_value is not None:
            rsi_value = self.rsi.value
            bb_upper = self.bb.upper
            bb_lower = self.bb.lower
            bb_middle = self.bb.middle
            
            # 交易逻辑
            if sma20_value > self.sma_50.value and rsi_value < 30:
                self.buy()
```

---

### 1️⃣3️⃣ `portfolio` - 投资组合管理

**代码量**: 65K | **用途**: 组合分析、风险计算

#### 组合管理

```python
from nautilus_trader.portfolio import Portfolio

class MyStrategy(Strategy):
    def on_bar(self, bar: Bar):
        # 获取投资组合信息
        portfolio = self.portfolio
        
        # 总体统计
        print(f"总权益: ${portfolio.equity()}")
        print(f"现金: ${portfolio.cash()}")
        print(f"已用保证金: ${portfolio.margin_used()}")
        print(f"可用保证金: ${portfolio.margin_available()}")
        
        # 持仓信息
        positions = portfolio.positions()
        for position in positions:
            print(f"{position.instrument}")
            print(f"  数量: {position.quantity}")
            print(f"  平均成本: ${position.avg_price}")
            print(f"  当前价: ${bar.close}")
            print(f"  浮动盈亏: ${position.pnl}")
            print(f"  盈利%: {position.pnl_pct:.2%}")
        
        # 特定合约的持仓
        aapl_position = portfolio.position(bar.instrument)
        if aapl_position:
            print(f"AAPL 持仓: {aapl_position.quantity}")
        
        # 杠杆比例
        leverage = portfolio.leverage()
        print(f"杠杆率: {leverage:.2f}x")
```

---

### 1️⃣4️⃣ `risk` - 风控管理

**代码量**: 60K | **用途**: 实寸订单审批、限额管理

#### 风控引擎

```
RiskEngine
  │
  ├─► Pre-Trade Checks        交易前检查
  │   ├─ Account Balance       账户余额
  │   ├─ Margin Check          保证金
  │   ├─ Position Limit        头寸限制
  │   ├─ Daily Loss Limit      日亏限制
  │   └─ Risk Limit            综合风险
  │
  └─► Post-Trade Monitoring   成交后监控
      ├─ Updated Exposure
      ├─ New Risk Metrics
      └─ Alert Triggers
```

#### Python 配置

```python
from nautilus_trader.risk import (
    RiskSettings,
    PositionLimit,
    DailyLossLimit,
)

# 风控配置
risk_settings = RiskSettings(
    # 头寸限制 (per instrument)
    max_position_size=1000,
    
    # 日内亏损限制
    max_daily_loss=10000,
    
    # 总回撤限制
    max_drawdown=0.20,  # 20%
    
    # 杠杆限制
    max_leverage=3.0,
)

engine.add_risk_settings(risk_settings)
```

---

## VI. 工具与测试

### 1️⃣5️⃣ `backtest` - 回测框架

**代码量**: 75K | **用途**: 历史数据回测

#### 回测流程

```python
from nautilus_trader.backtest import (
    BacktestEngine,
    BacktestConfig,
)

# 配置回测
config = BacktestConfig(
    title="EMA Cross Strategy",
    engine_seed=12345,
    logging_level="INFO",
)

# 创建引擎
engine = BacktestEngine(config)

# 添加数据
engine.add_data_source(
    catalog.load_bars(
        "AAPL.NASDAQ",
        "1-MINUTE-LAST",
        start="2024-01-01",
        end="2024-03-27",
    )
)

# 添加策略
engine.add_strategy(MyStrategy(config))

# 运行回测
engine.run()

# 生成报告
engine.print_statistics()
```

---

### 1️⃣6️⃣ `testkit` - 测试工具

**代码量**: 35K | **用途**: 单元测试、Mock 对象

#### 测试示例

```python
from nautilus_trader.testkit import (
    MockDataAdapter,
    MockExecutionAdapter,
)
from nautilus_trader.model.data import Bar

def test_strategy_on_bar():
    # 创建 Mock 适配器
    data_adapter = MockDataAdapter()
    exec_adapter = MockExecutionAdapter()
    
    # 创建策略实例
    strategy = MyStrategy(config={})
    
    # 模拟 Bar 数据
    bar = Bar(
        instrument=...,
        open=100.0,
        high=102.0,
        low=99.0,
        close=101.0,
        volume=1000000,
        ts_init=123456789,
    )
    
    # 调用 on_bar
    strategy.on_bar(bar)
    
    # 验证行为
    orders = exec_adapter.submitted_orders
    assert len(orders) == 1
    assert orders[0].side == OrderSide.BUY
```

---

## 📊 模块间依赖关系

```
最高层:
  Strategy (用户代码)
      │
降序:  ├─ Trading
      ├─ Execution
      ├─ Risk
      │
      ├─ Indicators
      ├─ Analysis
      └─ Portfolio
         
中层:  ├─ Adapters (IB, Binance等)
      ├─ Live
      ├─ Network
      │
      ├─ Data
      ├─ Persistence
      └─ Model
      
基层:  ├─ Core
      ├─ Common
      └─ Serialization
```

---

## 🎯 快速参考

### 问题排查

| 问题 | 检查项 |
|------|--------|
| 订单被拒 | RiskManager 日志 → 风控限制 |
| 没有行情数据 | Adapter 连接状态 → Data 缓存 |
| 持仓计算错误 | Position 平均成本 → Trade 记录 |
| 回测结果差 | 滑点设置 → 数据质量 |

---

**下一步**: 阅读 [核心流程深度解析](./Nautilus核心流程深度解析.md)

