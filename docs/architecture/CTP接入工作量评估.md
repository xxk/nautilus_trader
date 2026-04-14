# CTP接入工作量评估

## 状态

- 状态：已评估，待决策
- 评估日期：2026-03-27
- 适用范围：Nautilus 主仓新增中国期货 CTP live provider

## 结论摘要

在当前 Nautilus 架构下，接入 CTP 不属于“小适配器”工作，属于一项中高复杂度集成。

如果目标只是“跑通登录 + 合约查询 + Tick 行情订阅”，工作量约为 `2-4 周 / 1 名熟悉 Rust + Python + CTP 的工程师`。

如果目标是“可用于真实交易的完整 provider”，包含行情、报单、撤单、成交回报、持仓/资金同步、启动重建、交易所差异处理和基础测试闭环，工作量约为 `8-12 周 / 1-2 名工程师`。

如果目标是“生产可用”，再加上稳定性打磨、仿真/实盘联调、夜盘与异常场景覆盖、CI 与发布链路整理，整体应按 `12-16 周` 评估更稳妥。

## 为什么工作量不小

Nautilus 现有 adapter 主要以 HTTP/WebSocket 交易所为主，标准落点在 [docs/developer_guide/adapters.md](docs/developer_guide/adapters.md)。CTP 与这类适配器差异很大：

1. CTP 不是 HTTP/WebSocket 模式，而是原生回调式 C API。
2. 行情和交易是两套 API，需要分别管理登录、线程、连接状态和回调生命周期。
3. 中国期货交易所有显著业务差异，尤其是上期所/能源中心的平今平昨规则。
4. 实盘前通常需要处理结算确认、交易日切换、夜盘、断线重连、重复回报去重等问题。
5. Nautilus 当前仓库没有现成 CTP adapter，需要从零新增 Rust crate、Python 接入层、配置层和测试层。

## 仓库内已有证据

### Nautilus 侧

1. 新 adapter 的标准结构和阶段顺序已在 [docs/developer_guide/adapters.md](docs/developer_guide/adapters.md) 明确，默认假设是 Rust core + Python layer 双层实现。
2. 工厂装配点在 [crates/system/src/factories.rs](crates/system/src/factories.rs)，新增 provider 至少要补 `ClientConfig`、`DataClientFactory`、`ExecutionClientFactory`。
3. LiveNode 客户端配置入口在 [crates/live/src/config.rs](crates/live/src/config.rs)，新增 provider 需要进入 `data_clients` / `exec_clients` 装配链。
4. 当前 `crates/adapters/` 下已有 Binance、Bybit、OKX、Databento 等适配器，但没有 CTP 目录，说明不是补小缺口，而是新增一条完整接入链。

### 旧仓可复用经验

1. 旧系统里已有 CTP 原生封装，见 [Branch_Common_Pub/Providers.master/CTPProviderCLI/CTPProviderCLI.h](../../../4.0.1_Compatible/Branch_Common_Pub/Providers.master/CTPProviderCLI/CTPProviderCLI.h) 与 [Branch_Common_Pub/Providers.master/CTPProviderCLI/CTPProviderCLI.cpp](../../../4.0.1_Compatible/Branch_Common_Pub/Providers.master/CTPProviderCLI/CTPProviderCLI.cpp)。
2. 旧实现里已经显式处理了 SHFE/INE 的平今平昨 offset flag，这一段复杂度不可忽略。
3. 旧仓还有持仓归并逻辑，见 [Branch_Common_Pub/Providers.master/APiBridge/PositionManager.cs](../../../4.0.1_Compatible/Branch_Common_Pub/Providers.master/APiBridge/PositionManager.cs)。

这些代码可以复用“业务规则”和“场景清单”，但不能直接低成本搬进 Nautilus。原因是技术栈不同：旧仓是 C# / C++/CLI，Nautilus 主体是 Rust + PyO3 + Python。

## 建议按三档目标评估

### 档位 A：最小可用行情接入

目标：

1. 连接 MdApi / TraderApi
2. 完成鉴权与登录
3. 查询合约
4. 订阅 Tick
5. 将 Tick 映射为 Nautilus 数据模型

预计工作量：`2-4 周`

主要任务：

1. 新建 `crates/adapters/ctp/` Rust crate。
2. 做 CTP SDK FFI 绑定或包装层。
3. 建立行情回调到 Nautilus `QuoteTick` / `TradeTick` 的映射。
4. 打通 Python 层 `InstrumentProvider` 与 data client。
5. 完成基础示例和手工验证脚本。

主要风险：

1. SDK 绑定方式选型错误会放大后续维护成本。
2. 字符编码、线程模型、回调转发容易踩坑。
3. 合约元数据字段未定义完整时，后续 execution 会返工。

### 档位 B：完整数据 + 基础交易

目标：

1. 行情全链路可用
2. 支持下单、撤单、成交回报
3. 同步账户、持仓、委托状态
4. 支持启动后 reconciliation

预计工作量：`8-12 周`

主要任务：

1. 完成 TraderApi 请求/回报全链路映射。
2. 建立 OrderRef / FrontID / SessionID / ExchangeID 等标识映射。
3. 实现持仓、资金、委托、成交的标准化转换。
4. 处理 SHFE/INE 平今平昨规则。
5. 完成断线重连、重复回报去重、启动重建。
6. 补充集成测试、仿真环境联调与最小示例。

主要风险：

1. CTP 回报顺序与 Nautilus 内部状态机的语义对齐需要较多打磨。
2. 国内期货的 offset flag、投保标志、成交回报组合规则比较容易出现边界 bug。
3. 仿真环境与真实柜台表现不完全一致，联调周期不可压缩。

### 档位 C：生产可用

目标：

1. 支持夜盘/日盘切换
2. 支持结算确认流程
3. 支持断线恢复与启动重建
4. 支持稳定监控和错误定位
5. 可进入有限生产试运行

预计工作量：`12-16 周`

额外任务：

1. 增加交易日和夜盘切换处理。
2. 补齐错误码分类、重试边界和 fail-fast 日志。
3. 整理配置模板、发布方式和操作手册。
4. 设计 CI 里对 CTP SDK 的条件构建策略。
5. 增加回放用例和联调验收清单。

## 真正的复杂点

### 1. 不是普通 REST/WebSocket adapter

现有多数 adapter 可以按 HTTP + WebSocket 的通用模板推进，但 CTP 本质上是 SDK 回调模式，需要自己处理：

1. 原生动态库装载
2. 回调线程与运行时桥接
3. 请求号、回报流和状态同步
4. 会话恢复与断线重连

### 2. 交易规则比币圈/外盘现货复杂

重点复杂度：

1. 平今 / 平昨
2. 上期所 / 能源中心差异
3. 夜盘与交易日
4. 结算确认
5. 本地委托号与柜台委托号映射

### 3. 测试成本高于普通 adapter

因为 CTP 很难像 HTTP API 那样轻松做 mock server，很多问题只能在：

1. SimNow
2. 仿真柜台
3. 指定券商柜台
4. 指定交易时段

里验证。

### 4. 跨平台和发布链路要单独评估

Nautilus 当前是跨平台仓库，而 CTP SDK 交付通常带明显平台约束。需要尽早确定：

1. 只支持 Windows x64
2. 支持 Windows + Linux x64
3. 是否允许主仓仅做条件编译
4. Python 打包是否拆额外 extra / feature

如果这些问题不先定，后面返工会很重。

## 推荐实施顺序

### 阶段 0：预研与裁剪

预计：`3-5 天`

输出物：

1. SDK 绑定方案
2. 平台支持边界
3. 目标范围裁剪文档
4. 不做项清单

### 阶段 1：数据优先

预计：`2-3 周`

先只做：

1. 合约查询
2. Tick 订阅
3. 行情映射
4. InstrumentProvider

这样可以先验证 CTP SDK 桥接、符号建模和交易所枚举是否合理。

### 阶段 2：再接 execution

预计：`3-5 周`

再做：

1. 报单
2. 撤单
3. 委托/成交回报
4. 持仓/账户同步
5. reconciliation

### 阶段 3：最后做生产化

预计：`3-6 周`

包括：

1. 夜盘
2. 结算确认
3. 恢复与补偿
4. 联调与验收

## 人力建议

### 最低配置

1. `1 名主程`：熟悉 Rust / Python / CTP
2. `1 名业务测试或交易联调人员`：熟悉 SimNow / 柜台联调

### 更稳妥配置

1. `1 名 Rust/系统工程师`：负责 SDK、运行时、状态机、性能
2. `1 名 Python/策略侧工程师`：负责 provider、配置、示例、联调脚本
3. `1 名业务联调支持`：负责仿真账号、柜台差异、交易规则核验

## 决策建议

如果当前目标是 `nautilus_strategies` 先跑研究、回测和单机场景，不建议现在就做完整 CTP provider。

更合理的路线是：

1. 先做 `阶段 0 + 阶段 1`，把 CTP 数据接入跑通。
2. 数据链稳定后，再决定是否投入 execution。
3. 只有当你明确要走国内期货实盘，才投入完整 `8-12 周` 级别工作。

如果确定采用 `方案 1`，最小接口与目录设计见 [docs/architecture/CTP方案1最小接口草图.md](docs/architecture/CTP方案1最小接口草图.md)。

## 最终评估

- 数据 only：`中等工作量`
- 数据 + 基础交易：`中高工作量`
- 生产可用：`高工作量`

不建议按“新增一个普通 provider”估算。更接近“新增一个带本地原生 SDK、国内期货特殊规则和实盘联调要求的核心 adapter”。