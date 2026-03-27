# Nautilus 学习路径与实践指南

**文档版本**: v1.0  
**目标读者**: 想系统学习 Nautilus 的开发者  
**前置阅读**: [架构全景指南](./Nautilus架构全景指南.md)

---

## 📚 推荐学习路径

### 阶段规划

```
┌────────────────────────────────────────────────────────┐
│ 阶段1: 基础认知 (1-2 周)                               │
│ ◆ 理解事件驱动架构                                      │
│ ◆ 掌握基本数据结构                                      │
│ ◆ 运行第一个回测                                        │
│ 🎯 目标: 能运行官方示例策略                             │
├────────────────────────────────────────────────────────┤
│ 阶段2: 深入理解 (2-4 周)                               │
│ ◆ 深入模块架构                                          │
│ ◆ 理解数据流与执行流                                    │
│ ◆ 编写简单策略                                          │
│ 🎯 目标: 能独立编写 EMA 交叉策略                        │
├────────────────────────────────────────────────────────┤
│ 阶段3: 高级应用 (1-3 月)                               │
│ ◆ 自定义指标开发                                        │
│ ◆ 多策略组合                                            │
│ ◆ 实盘部署与监控                                        │
│ 🎯 目标: 能部署实盘策略                                 │
├────────────────────────────────────────────────────────┤
│ 阶段4: 专家级 (3-6 月)                                 │
│ ◆ 性能优化                                              │
│ ◆ 自定义 Adapter 开发                                  │
│ ◆ 参数优化框架                                          │
│ 🎯 目标: 能扩展框架功能                                 │
└────────────────────────────────────────────────────────┘
```

---

## 🟢 阶段1: 基础认知 (1-2 周)

### 第1天-2天: 环境与概念

#### 任务清单
- [ ] 安装 Nautilus
- [ ] 理解事件驱动核心概念
- [ ] 学习基本数据类型

#### 1.1 安装 Nautilus

```bash
# 方式1: 从 PyPI 安装 (推荐)
pip install nautilus-trader

# 方式2: 从源码构建 (用于开发)
git clone https://github.com/nautechsystems/nautilus_trader.git
cd nautilus_trader
python build.py --help
python build.py build  # 需要 Rust 工具链
```

#### 1.2 验证安装

```python
# 检查版本
import nautilus_trader
print(nautilus_trader.__version__)

# 检查关键模块
from nautilus_trader.trading.strategy import Strategy
from nautilus_trader.model.data import Bar, Tick
from nautilus_trader.model.enums import OrderSide

print("✅ Nautilus 安装成功")
```

#### 1.3 核心概念学习

| 概念 | 解释 | 示例 |
|------|------|------|
| **事件驱动** | 所有操作由事件触发 | on_bar() 当新 Bar 到达时调用 |
| **回调函数** | 事件发生时自动调用 | 订单成交时 on_trade() 被调用 |
| **缓存** | 实时维护的数据集 | cache.bars() 获取历史 Bar |
| **适配器** | 交易所连接器 | IB Adapter 连接 Interactive Brokers |
| **时间模型** | 回测/实盘通用 | 相同代码在两种模式运行 |

### 第3天-4天: 官方示例学习

#### 任务清单
- [ ] 运行官方 EMA 交叉策略
- [ ] 理解策略的关键方法
- [ ] 修改参数进行回测

#### 2.1 运行官方示例

```python
# 查找官方示例路径
import nautilus_trader
import os

# 获取示例目录
example_path = os.path.join(
    os.path.dirname(nautilus_trader.__file__),
    "..",  # 回到上级目录
    "examples"
)
print(f"示例目录: {example_path}")

# 或直接从 GitHub 克隆
# https://github.com/nautechsystems/nautilus_trader/tree/develop/examples
```

#### 2.2 简单 EMA 交叉示例

```python
"""
最简单的策略示例
"""
from nautilus_trader.trading.strategy import Strategy
from nautilus_trader.model.data import Bar
from nautilus_trader.model.enums import OrderSide, OrderType
from nautilus_trader.model.orders import MarketOrder

class SimpleEMAStrategy(Strategy):
    """简单 EMA 交叉策略"""
    
    def __init__(self, config):
        super().__init__(config)
        self.fast_period = 10  # 快线周期
        self.slow_period = 20  # 慢线周期
    
    def on_start(self):
        """策略启动时调用 - 订阅数据"""
        print("📍 策略启动")
        
        # 获取第一个交易品种
        instrument = list(self.cache.instruments())[0]
        
        # 订阅 K 线数据 (1分钟)
        self.subscribe_bars(instrument.id, "1-MINUTE-LAST")
        
        print(f"已订阅: {instrument.id}")
    
    def on_bar(self, bar: Bar):
        """每当新 K 线到达时调用"""
        
        # 1. 获取历史 K 线
        bars = self.cache.bars(bar.instrument.id)
        
        # 2. 检查数据量是否足够
        if len(bars) < self.slow_period:
            return  # 数据不足，继续等待
        
        # 3. 计算移动平均线
        close_prices = [b.close for b in bars]
        fast_ma = sum(close_prices[-self.fast_period:]) / self.fast_period
        slow_ma = sum(close_prices[-self.slow_period:]) / self.slow_period
        
        # 4. 获取当前持仓
        position = self.portfolio.position(bar.instrument.id)
        
        # 5. 交易逻辑
        if fast_ma > slow_ma:
            # 趋势上升 → 买入
            if not position or position.is_closed:
                order = MarketOrder(
                    instrument=bar.instrument,
                    side=OrderSide.BUY,
                    quantity=100,
                    init_id=self.order_factory.init_id(),
                    ts_init=self.clock.timestamp_ns(),
                )
                self.submit_order(order)
                self.logger.info(f"🟢 买入: {order.quantity}")
        
        elif fast_ma < slow_ma:
            # 趋势下降 → 卖出
            if position and position.is_long:
                order = MarketOrder(
                    instrument=bar.instrument,
                    side=OrderSide.SELL,
                    quantity=position.quantity,
                    init_id=self.order_factory.init_id(),
                    ts_init=self.clock.timestamp_ns(),
                )
                self.submit_order(order)
                self.logger.info(f"🔴 卖出: {order.quantity}")
    
    def on_order_filled(self, event):
        """订单成交时调用"""
        self.logger.info(f"✅ 成交: {event.order.side} {event.last_qty} @ {event.last_px}")
    
    def on_stop(self):
        """策略停止时调用 - 清理资源"""
        print("🛑 策略停止")
```

#### 2.3 运行回测

```python
# 完整回测脚本
from nautilus_trader.backtest import (
    BacktestEngine,
    BacktestConfig,
)
from nautilus_trader.persistence import ParquetDataCatalog
from nautilus_trader.model.data import BarType, Bar
from nautilus_trader.model.enums import BarAggregation, PriceType
import pandas as pd

# 配置回测
config = BacktestConfig(
    title="Simple EMA Cross",
    engine_seed=12345,
    logging_level="INFO",
)

# 创建回测引擎
engine = BacktestEngine(config)

# 加载数据 (假设有本地 Parquet 数据)
try:
    catalog = ParquetDataCatalog("/data/nautilus/")
    
    bars = catalog.load_bars(
        "AAPL.NASDAQ",
        BarType("AAPL.NASDAQ", BarAggregation.MINUTE, PriceType.LAST, 1),
        start="2024-01-01",
        end="2024-03-27",
    )
    
    engine.add_data_source(bars)
except:
    print("❌ 无本地数据，创建模拟数据")
    # 生成模拟 Bar 数据用于演示
    import numpy as np
    from datetime import datetime, timedelta
    
    start_date = datetime(2024, 1, 1)
    dates = [start_date + timedelta(minutes=i) for i in range(10000)]
    prices = 100 + np.cumsum(np.random.randn(10000) * 0.5)
    
    bars_data = [
        Bar(
            instrument_id="DEMO.TEST",
            open=prices[i],
            high=prices[i] + 0.5,
            low=prices[i] - 0.5,
            close=prices[i],
            volume=1000000,
            ts_init=int(dates[i].timestamp() * 1e9),
        )
        for i in range(len(prices))
    ]
    # 使用模拟数据...

# 添加策略
engine.add_strategy(SimpleEMAStrategy(config))

# 运行回测
results = engine.run()

# 查看结果
print("\n" + "="*50)
print("📊 回测结果")
print("="*50)
print(results.statistics())
```

### 第5天-8天: 基础概念练习

#### 任务清单
- [ ] 理解不同订单类型 (MARKET, LIMIT, STOP)
- [ ] 学习持仓管理
- [ ] 理解风控配置

#### 3.1 订单类型详解

```python
from nautilus_trader.model.orders import (
    MarketOrder,      # 市价订单 - 立即成交
    LimitOrder,       # 限价订单 - 特定价格成交
    StopOrder,        # 止损单 - 触发价格达到时下单
    StopLimitOrder,   # 止损限价单 - 组合
)
from nautilus_trader.model.enums import TimeInForce

# 市价订单 - 用于快速进出
market_order = MarketOrder(
    instrument=instrument,
    side=OrderSide.BUY,
    quantity=100,
    init_id=self.order_factory.init_id(),
    ts_init=self.clock.timestamp_ns(),
)

# 限价订单 - 用于高精度入场
limit_order = LimitOrder(
    instrument=instrument,
    side=OrderSide.BUY,
    quantity=100,
    price=150.00,  # 限定价格
    time_in_force=TimeInForce.GTC,  # 直到取消为止
    init_id=self.order_factory.init_id(),
    ts_init=self.clock.timestamp_ns(),
)

# 止损单 - 用于风险控制
stop_order = StopOrder(
    instrument=instrument,
    side=OrderSide.SELL,
    quantity=100,
    trigger_price=145.00,  # 触发价格
    init_id=self.order_factory.init_id(),
    ts_init=self.clock.timestamp_ns(),
)

# 止损限价单 - 用于精确止损
stop_limit_order = StopLimitOrder(
    instrument=instrument,
    side=OrderSide.SELL,
    quantity=100,
    price=144.50,         # 限价
    trigger_price=145.00,  # 止损触发价
    init_id=self.order_factory.init_id(),
    ts_init=self.clock.timestamp_ns(),
)
```

#### 3.2 持仓管理

```python
def on_bar(self, bar: Bar):
    # 获取计算持仓信息
    position = self.portfolio.position(bar.instrument.id)
    
    if position is None:
        print("无持仓")
    else:
        # 检查持仓状态
        print(f"持仓方向: {position.side}")        # Side.LONG / NEUTRAL / SHORT
        print(f"持仓数量: {position.quantity}")    # 100
        print(f"平均成本: {position.avg_price}")   # 150.00
        print(f"当前浮动PnL: {position.pnl}")     # 500.00
        print(f"已实现PnL: {position.realized_pnl}")
        
        # 检查是否盈利
        if position.pnl > 0:
            print("✅ 账面盈利")
        elif position.pnl < 0:
            print("❌ 账面亏损")
        
        # 获取账户信息
        account = self.portfolio
        print(f"总权益: {account.equity()}")
        print(f"现金: {account.cash()}")
        print(f"已用保证金: {account.margin_used()}")
        print(f"可用保证金: {account.margin_available()}")
```

### 第9天-14天: 第一个完整策略

#### 任务清单
- [ ] 设计策略逻辑
- [ ] 实现策略代码
- [ ] 进行回测与优化
- [ ] 记录学习笔记

#### 4.1 策略设计示例

```
策略名称: RSI 超买超卖策略
===============================

1. 入场条件:
   - RSI < 30 (超卖) → 买入
   - RSI > 70 (超买) → 卖出

2. 出场条件:
   - 获利 1% 或
   - 亏损 0.5% 或
   - 持仓超过60分钟

3. 风控:
   - 最大持仓: 100 股
   - 日亏限制: $500
   - 单笔止损: 0.5%

4. 回测周期: 2024-01-01 ~ 2024-03-27
5. 交易品种: AAPL.NASDAQ
```

#### 4.2 完整实现

```python
class RSIStrategy(Strategy):
    """RSI 超买超卖交易策略"""
    
    def __init__(self, config):
        super().__init__(config)
        self.rsi_period = 14
        self.overbought = 70
        self.oversold = 30
        self.max_position_size = 100
        self.exit_profit_pct = 0.01  # 1% 利润
        self.exit_loss_pct = -0.005  # -0.5% 止损
        
        # 持仓跟踪
        self.entry_price = None
        self.entry_time = None
    
    def on_start(self):
        """初始化"""
        instrument = list(self.cache.instruments())[0]
        self.instrument = instrument
        
        # 订阅数据
        self.subscribe_bars(instrument.id, "5-MINUTE-LAST")
        self.subscribe_ticks(instrument.id)
        
        # 初始化指标
        from nautilus_trader.indicators import RelativeStrengthIndex
        self.rsi = RelativeStrengthIndex(self.rsi_period)
    
    def on_tick(self, tick):
        """处理 tick 数据"""
        # 可选: 记录点差等信息
        spread = tick.ask_price - tick.bid_price
        
    def on_bar(self, bar):
        """处理 K 线数据"""
        
        # 更新指标
        self.rsi.update(bar)
        
        if self.rsi.value is None:
            return  # 指标值还未准备好
        
        # 获取当前持仓
        position = self.portfolio.position(bar.instrument.id)
        
        # ===== 出场逻辑 =====
        if position and position.is_long:
            # 计算当前收益率
            pnl_pct = (bar.close - self.entry_price) / self.entry_price
            
            # 获利了结
            if pnl_pct >= self.exit_profit_pct:
                self._close_position(f"获利出局 ({pnl_pct:.2%})")
                return
            
            # 止损
            if pnl_pct <= self.exit_loss_pct:
                self._close_position(f"止损出局 ({pnl_pct:.2%})")
                return
            
            # 时间止损 (持仓超过60分钟)
            holding_bars = len(self.cache.bars(bar.instrument.id)) - (self.cache.bars(bar.instrument.id).index(
                [b for b in self.cache.bars(bar.instrument.id) if b.ts_init == self.entry_time][0]
            ))
            if holding_bars > 12:  # 60 分钟 = 12 根 5 分钟 K 线
                self._close_position("时间止损")
                return
        
        # ===== 入场逻辑 =====
        if not position or position.is_closed:
            if self.rsi.value < self.oversold:
                self._open_position(OrderSide.BUY, bar)
            elif self.rsi.value > self.overbought:
                self._open_position(OrderSide.SELL, bar)
    
    def _open_position(self, side, bar):
        """开仓"""
        quantity = self.max_position_size
        
        order = MarketOrder(
            instrument=bar.instrument,
            side=side,
            quantity=quantity,
            init_id=self.order_factory.init_id(),
            ts_init=self.clock.timestamp_ns(),
        )
        
        self.entry_price = bar.close
        self.entry_time = bar.ts_init
        
        self.submit_order(order)
        self.logger.info(f"🟢 开仓: {side} {quantity} 股 @ {bar.close}")
    
    def _close_position(self, reason):
        """平仓"""
        position = self.portfolio.position(self.instrument.id)
        
        if position and position.is_long:
            order = MarketOrder(
                instrument=self.instrument,
                side=OrderSide.SELL,
                quantity=position.quantity,
                init_id=self.order_factory.init_id(),
                ts_init=self.clock.timestamp_ns(),
            )
            
            self.submit_order(order)
            self.logger.info(f"🔴 平仓: {reason}")
```

---

## 🟡 阶段2: 深入理解 (2-4 周)

### 第1-2周: 架构与数据流

#### 任务清单
- [ ] 阅读模块详解指南
- [ ] 理解数据流向
- [ ] 研究关键类型定义

#### 1.1 核心模块研究

| 模块 | 学习重点 | 预计时长 |
|------|----------|--------|
| `core` | 基础类型 (UUID, Price) | 2小时 |
| `model` | 数据模型 (Order, Position) | 3小时 |
| `data` | 数据缓存与查询 | 3小时 |
| `execution` | 订单执行流程 | 4小时 |
| `analysis` | 回测分析与指标 | 3小时 |

#### 1.2 阅读源代码

```
优先阅读顺序:

1. src/model/orders.rs      订单定义
2. src/model/positions.rs   持仓定义
3. src/execution/manager.rs 执行管理
4. src/data/cache.rs        数据缓存
5. src/trading/engine.rs    策略引擎
```

### 第3-4周: 指标开发与策略优化

#### 任务清单
- [ ] 学习现有指标的实现
- [ ] 开发自定义指标
- [ ] 进行参数优化

#### 2.1 自定义指标开发

```python
from nautilus_trader.indicators.base import Indicator
from nautilus_trader.model.data import Bar

class CustomMomentumIndicator(Indicator):
    """自定义动量指标"""
    
    def __init__(self, period: int, name: str = "CMI"):
        super().__init__(name)
        self.period = period
        self._values = []
    
    def update(self, bar: Bar):
        """更新指标值"""
        self._values.append(bar.close)
        
        if len(self._values) < self.period:
            return
        
        # 计算动量 (当前价 - N 周期前价格)
        momentum = self._values[-1] - self._values[-self.period]
        self._value = momentum  # 父类属性
    
    @property
    def value(self):
        """获取指标值"""
        return self._value if hasattr(self, '_value') else None

# 使用示例
class StrategyWithCustomIndicator(Strategy):
    def on_start(self):
        self.momentum = CustomMomentumIndicator(period=14)
    
    def on_bar(self, bar):
        self.momentum.update(bar)
        
        if self.momentum.value is None:
            return
        
        if self.momentum.value > 0:
            # 动量为正，趋势向上
            self.buy()
```

#### 2.2 参数优化框架

```python
from itertools import product

def optimize_strategy(
    param_ranges: dict,
    start_date: str,
    end_date: str,
) -> list:
    """
    参数网格搜索优化
    
    Args:
        param_ranges: {'fast_period': [5, 10, 15], 'slow_period': [20, 30, 50]}
        start_date: 回测起始日期
        end_date: 回测结束日期
    
    Returns:
        优化结果列表
    """
    results = []
    
    # 生成所有参数组合
    param_names = list(param_ranges.keys())
    param_values = list(param_ranges.values())
    
    for combination in product(*param_values):
        params = dict(zip(param_names, combination))
        
        # 运行回测
        config = BacktestConfig(title=f"Test_{params}")
        engine = BacktestEngine(config)
        
        # 创建策略实例 (使用当前参数)
        strategy = EMAStrategy(config, **params)
        engine.add_strategy(strategy)
        
        # 运行回测
        results_single = engine.run()
        
        # 记录结果
        results.append({
            'params': params,
            'sharpe_ratio': results_single.statistics().sharpe_ratio,
            'total_return': results_single.statistics().total_return,
            'max_drawdown': results_single.statistics().max_drawdown,
        })
    
    # 按夏普率排序
    results.sort(key=lambda x: x['sharpe_ratio'], reverse=True)
    
    return results

# 使用示例
optimal_params = optimize_strategy(
    param_ranges={
        'fast_period': [10, 15, 20],
        'slow_period': [30, 40, 50],
    },
    start_date="2024-01-01",
    end_date="2024-03-27",
)

print("最优参数:")
for i, result in enumerate(optimal_params[:3]):
    print(f"{i+1}. {result['params']}")
    print(f"   夏普率: {result['sharpe_ratio']:.2f}")
```

---

## 🔴 阶段3: 高级应用 (1-3 月)

### 多策略组合

```python
class MultiStrategyPortfolio:
    """多策略组合管理"""
    
    def __init__(self, engine):
        self.engine = engine
        self.strategies = {}
        self.weights = {}
    
    def add_strategy(self, name: str, strategy, weight: float):
        """添加策略到组合"""
        self.strategies[name] = strategy
        self.weights[name] = weight
        self.engine.add_strategy(strategy)
    
    def rebalance(self, period: int = 60):
        """定期重新平衡"""
        # 每 60 分钟检查一次各策略的配置
        pass

# 使用示例
portfolio = MultiStrategyPortfolio(engine)
portfolio.add_strategy("EMA", EMAStrategy(), weight=0.5)
portfolio.add_strategy("RSI", RSIStrategy(), weight=0.3)
portfolio.add_strategy("MACD", MACDStrategy(), weight=0.2)
```

### 实盘部署

```python
from nautilus_trader.adapters.interactive_brokers import (
    InteractiveBrokersLiveDataAdapter,
    InteractiveBrokersLiveExecAdapter,
)

# 实盘配置
ib_config = {
    'host': 'localhost',
    'port': 7497,
    'client_id': 1,
}

# 创建实时引擎
engine = TradingEngine()

# 添加 IB 适配器
data_adapter = InteractiveBrokersLiveDataAdapter(ib_config)
exec_adapter = InteractiveBrokersLiveExecAdapter(ib_config)

engine.add_data_adapter(data_adapter)
engine.add_exec_adapter(exec_adapter)

# 启动策略
engine.add_strategy(LiveStrategy())
engine.start()

# 监控运行
while True:
    engine.process_messages()
    time.sleep(0.1)
```

---

## 📖 推荐阅读资源

### 官方文档
1. **API 参考**: https://nautilustrader.io/docs/
2. **教程**: https://nautilustrader.io/docs/tutorials/
3. **最佳实践**: https://nautilustrader.io/docs/concepts/

### 源代码
1. **核心库**: `crates/core/src/`
2. **执行引擎**: `crates/execution/src/`
3. **示例策略**: `nautilus_trader/examples/`

### GitHub 讨论
- Issues: 问题与 Bug 报告
- Discussions: 设计讨论
- Pull Requests: 社区贡献

---

## 🎯 常见陷阱与解决方案

| 问题 | 原因 | 解决方案 |
|------|------|--------|
| 回测无数据 | 缺少历史数据文件 | 检查数据路径、数据格式 |
| 指标值为 None | 数据量不足 | 等待足够的 Bar 数据 |
| 订单被拒 | 风控限制 | 检查 RiskManager 日志 |
| 内存泄漏 | 事件监听器未清理 | 及时 unsubscribe |
| 策略不执行 | 未订阅合约 | 在 on_start() 中订阅 |

---

## 💡 学习建议

### 学习方法

1. **边学边练** - 先运行官方示例，再修改参数
2. **源代码阅读** - 深入关键模块的实现
3. **代码审查** - 参考社区贡献的策略
4. **错误调试** - 通过日志理解执行流程
5. **社区互动** - 在 GitHub/Discord 讨论疑问

### 时间分配

```
每天学习时间: 2-3 小时

├─ 1 小时: 理论学习 (文档/视频)
├─ 1 小时: 代码阅读 (源码/示例)
└─ 1 小时: 动手实践 (编码/调试)
```

### 进度检查清单

```
☐ 能独立运行回测
☐ 能编写简单策略 (3 个以上)
☐ 能理解数据流向
☐ 能进行参数优化
☐ 能自定义指标
☐ 能理解风控机制
☐ 能部署到实盘
☐ 能监控与调试
```

---

## 📞 获取帮助

### 官方渠道
- 📧 Email: support@nautilustrader.io
- 💬 Discord: https://discord.gg/NautilusTrader
- 🐙 GitHub Issues: https://github.com/nautechsystems/nautilus_trader/issues

### 自助资源
- 📚 官方文档: https://nautilustrader.io/docs/
- 🎥 视频教程: (社区提供)
- 💻 代码示例: nautilus_trader/examples/

---

**总结**: 通过系统的学习路径，你可以在 3-6 个月内掌握 Nautilus 框架，并能够开发和部署生产级的交易策略。关键是**持之以恒**和**动手实践**！

🚀 开始你的 Nautilus 学习之旅吧！

