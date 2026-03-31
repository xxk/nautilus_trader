# CTP方案1最小接口草图

## 状态

- 状态：已选型，待预研实现
- 方案：`CTP 官方 C++ SDK -> 薄 C wrapper -> Rust FFI -> Nautilus adapter`
- 日期：2026-03-27

## 目标

这份文档只回答一件事：

如果采用方案 1，第一阶段应该把 `C wrapper` 做成什么样，Rust 侧怎么接，哪些东西必须放在 wrapper，哪些东西必须留在 Rust。

## 总体分层

```text
CTP 官方 SDK (C++)
  -> ctpwrapper (C ABI)
  -> nautilus-ctp-sys (Rust FFI)
  -> nautilus-ctp (Rust 安全封装 + adapter)
  -> PyO3 / Python provider
  -> LiveNode / InstrumentProvider / LiveMarketDataClient
```

核心原则：

1. `C++ 对象生命周期` 留在 wrapper。
2. `extern "C"` 只暴露稳定、窄口的 C ABI。
3. `线程、channel、状态机、模型映射` 放在 Rust。
4. 第一阶段先只做 `数据链路`，不提前把 execution 混进来。

## 为什么要做薄 wrapper

CTP SDK 是 C++ 类 + SPI 回调模型，不适合让 Rust 直接面对：

1. 虚函数回调多。
2. 生命周期要求强。
3. 头文件类型多为定长字符数组。
4. ABI 和编译器兼容性敏感。

因此 wrapper 的职责只有两个：

1. 把 C++ 类和回调包装成纯 C ABI。
2. 保证 Rust 永远只面对 POD 结构体、函数指针和句柄。

## 最小目录骨架

建议目录如下：

```text
crates/
  adapters/
    ctp/
      ctpwrapper/                 # C/C++ wrapper 工程
        include/
          ctpwrapper.h
        src/
          ctpwrapper.cpp
          ctp_md_spi.cpp
          ctp_trader_spi.cpp
      sys/                        # Rust FFI 低层绑定
        src/
          lib.rs
        build.rs
      src/                        # Rust 安全封装 + adapter
        common/
          consts.rs
          enums.rs
          error.rs
          models.rs
        md/
          client.rs
          callbacks.rs
          parse.rs
        config.rs
        data.rs
        factories.rs
        lib.rs
        python/
          mod.rs
          factories.rs
          config.rs

nautilus_trader/
  adapters/
    ctp/
      __init__.py
      config.py
      constants.py
      data.py
      providers.py
```

第一阶段不创建 `execution.py`，避免范围失控。

## wrapper 层最小边界

### wrapper 必须负责

1. 创建 / 销毁 `MdApi` 实例。
2. 注册 SPI 并把回调转成 C 函数指针。
3. 发起连接、登录、登出、订阅行情、查询合约。
4. 把原始 CTP 回调字段转换为稳定的 C 结构体。
5. 统一返回错误码和错误消息。

### wrapper 不应该负责

1. 订单状态机。
2. Nautilus 模型构造。
3. 策略级符号解析。
4. 复杂重连策略。
5. 业务规则判断。

换句话说：wrapper 只做 `桥`，不做 `业务脑子`。

## 第一阶段最小 C ABI

### 句柄与回调类型

```c
typedef struct ctp_md_api ctp_md_api_t;
typedef void* ctp_user_data_t;

typedef void (*ctp_on_front_connected_fn)(ctp_user_data_t user_data);
typedef void (*ctp_on_front_disconnected_fn)(int reason, ctp_user_data_t user_data);
typedef void (*ctp_on_rsp_error_fn)(int error_id, const char* error_msg, ctp_user_data_t user_data);
typedef void (*ctp_on_rsp_user_login_fn)(
    int error_id,
    const char* error_msg,
    const char* trading_day,
    const char* login_time,
    const char* broker_id,
    const char* user_id,
    ctp_user_data_t user_data
);
typedef void (*ctp_on_rsp_sub_market_data_fn)(
    const char* instrument_id,
    int error_id,
    const char* error_msg,
    ctp_user_data_t user_data
);
typedef void (*ctp_on_rtn_depth_market_data_fn)(
    const struct ctp_depth_market_data* data,
    ctp_user_data_t user_data
);
typedef void (*ctp_on_rsp_qry_instrument_fn)(
    const struct ctp_instrument_field* instrument,
    int is_last,
    int error_id,
    const char* error_msg,
    ctp_user_data_t user_data
);
```

### 回调表

```c
typedef struct ctp_md_callbacks {
    ctp_on_front_connected_fn on_front_connected;
    ctp_on_front_disconnected_fn on_front_disconnected;
    ctp_on_rsp_error_fn on_rsp_error;
    ctp_on_rsp_user_login_fn on_rsp_user_login;
    ctp_on_rsp_sub_market_data_fn on_rsp_sub_market_data;
    ctp_on_rtn_depth_market_data_fn on_rtn_depth_market_data;
    ctp_on_rsp_qry_instrument_fn on_rsp_qry_instrument;
} ctp_md_callbacks_t;
```

### 配置结构体

```c
typedef struct ctp_md_config {
    const char* flow_path;
    const char* front_addr;
    const char* broker_id;
    const char* user_id;
    const char* password;
    const char* auth_code;
    const char* app_id;
    const char* product_info;
    int udp;
    int multicast;
} ctp_md_config_t;
```

### 结构体草图

```c
typedef struct ctp_instrument_field {
    char instrument_id[31];
    char exchange_id[9];
    char product_id[31];
    char product_class;
    int volume_multiple;
    double price_tick;
    char create_date[9];
    char open_date[9];
    char expire_date[9];
    char start_deliv_date[9];
    char end_deliv_date[9];
    double long_margin_ratio;
    double short_margin_ratio;
} ctp_instrument_field_t;

typedef struct ctp_depth_market_data {
    char trading_day[9];
    char instrument_id[31];
    char exchange_id[9];
    char exchange_inst_id[31];
    char update_time[9];
    int update_millisec;
    double last_price;
    double pre_settlement_price;
    double settlement_price;
    double open_interest;
    int volume;
    double turnover;
    double open_price;
    double highest_price;
    double lowest_price;
    double bid_price_1;
    int bid_volume_1;
    double ask_price_1;
    int ask_volume_1;
} ctp_depth_market_data_t;
```

### 函数集草图

```c
int ctp_md_create(
    const ctp_md_config_t* config,
    const ctp_md_callbacks_t* callbacks,
    ctp_user_data_t user_data,
    ctp_md_api_t** out_api
);

int ctp_md_init(ctp_md_api_t* api);
int ctp_md_release(ctp_md_api_t* api);
int ctp_md_register_front(ctp_md_api_t* api, const char* front_addr);
int ctp_md_req_user_login(ctp_md_api_t* api, int request_id);
int ctp_md_req_user_logout(ctp_md_api_t* api, int request_id);
int ctp_md_subscribe_market_data(ctp_md_api_t* api, const char** instrument_ids, int count);
int ctp_md_unsubscribe_market_data(ctp_md_api_t* api, const char** instrument_ids, int count);
int ctp_md_req_qry_instrument(ctp_md_api_t* api, const char* instrument_id_filter, int request_id);
const char* ctp_md_last_error_message(ctp_md_api_t* api);
```

## 为什么第一阶段只做这些函数

这套接口刚好覆盖：

1. 建连
2. 登录
3. 查询合约
4. 订阅 Tick
5. 收行情回调

已经足够完成：

1. `InstrumentProvider.load_all_async()`
2. `LiveMarketDataClient._connect()`
3. `_subscribe_quote_ticks()` 或 `_subscribe_trade_ticks()` 的最小实现

它不会把 execution 的复杂度提前引进来。

## Rust FFI 层建议

`sys` 层只做低层声明，不写业务逻辑：

```rust
#[repr(C)]
pub struct ctp_md_config_t { ... }

#[repr(C)]
pub struct ctp_instrument_field_t { ... }

#[repr(C)]
pub struct ctp_depth_market_data_t { ... }

unsafe extern "C" {
    pub fn ctp_md_create(... ) -> i32;
    pub fn ctp_md_init(... ) -> i32;
    pub fn ctp_md_release(... ) -> i32;
    pub fn ctp_md_req_user_login(... ) -> i32;
    pub fn ctp_md_subscribe_market_data(... ) -> i32;
    pub fn ctp_md_req_qry_instrument(... ) -> i32;
}
```

要求：

1. `sys` 层不解析业务字段。
2. `sys` 层不创建 Nautilus 模型。
3. 所有 unsafe 聚合到 `nautilus-ctp` 的低层封装里。

## Rust 安全封装层建议

建议在 `crates/adapters/ctp/src/md/client.rs` 中提供 `CtpMdClient`：

```rust
pub struct CtpMdClient {
    handle: NonNull<ctp_md_api_t>,
    callbacks: Arc<CtpMdCallbacks>,
    event_tx: tokio::sync::mpsc::UnboundedSender<CtpMdEvent>,
}
```

建议事件枚举：

```rust
pub enum CtpMdEvent {
    FrontConnected,
    FrontDisconnected { reason: i32 },
    LoginResponse {
        error_id: i32,
        error_msg: String,
        trading_day: String,
    },
    Instrument {
        field: CtpInstrumentField,
        is_last: bool,
    },
    MarketData(CtpDepthMarketData),
    Error {
        error_id: i32,
        error_msg: String,
    },
}
```

关键点：

1. C 回调只做最小复制，然后推送到 channel。
2. Rust 异步任务从 channel 消费事件。
3. 真正的模型映射在 `parse.rs`。

## Nautilus 侧第一阶段落点

### 1. config

新增 `CtpDataClientConfig`，字段建议最小化：

1. `front_addr`
2. `broker_id`
3. `user_id`
4. `password`
5. `auth_code`
6. `app_id`
7. `product_info`
8. `flow_path`
9. `instrument_filter`

### 2. provider

新增 `CtpInstrumentProvider`：

1. 用 `req_qry_instrument` 拉合约
2. 把 `ctp_instrument_field_t` 映射到 `FuturesContract`
3. 第一阶段只支持期货，不碰期权和组合腿

### 3. data client

新增 `CtpLiveMarketDataClient`：

1. `_connect` 建连并登录
2. `_subscribe_instruments` 拉全量合约或过滤合约
3. `_subscribe_quote_ticks` 订阅深度行情
4. 把深度行情转换成 `QuoteTick`
5. 如有逐笔成交再考虑 `TradeTick`，第一阶段不强求

### 4. factories

新增 `CtpDataClientFactory`，只注册 data，不注册 execution。

## 第一阶段明确不做项

必须写死，避免范围膨胀：

1. 不做下单
2. 不做撤单
3. 不做账户与持仓
4. 不做结算确认
5. 不做夜盘恢复
6. 不做 SHFE/INE 平今平昨
7. 不做 full depth order book

这些都属于阶段 2 之后。

## 第一阶段验收标准

做到以下 6 条就算通过：

1. 可以加载 wrapper 动态库。
2. 可以成功登录 MdApi。
3. 可以查询并构造一批有效 `FuturesContract`。
4. 可以订阅指定合约的实时行情。
5. 可以稳定产出 `QuoteTick` 进入 Nautilus 数据引擎。
6. 断开连接时能明确报错，不做静默失败。

## 实现顺序建议

### 第 1 步

先单独做 `ctpwrapper` 小工程，验证：

1. 能编译
2. 能连 SimNow
3. 能把一条行情回调打印出来

### 第 2 步

再做 `sys` 层，把 `create/init/login/subscribe/query_instrument` 五类接口绑进去。

### 第 3 步

再做 Rust 事件封装和最小 data client。

### 第 4 步

最后接 Python provider 和示例脚本。

## 风险提醒

1. `flow_path`、回调线程和登录顺序不能靠猜，必须按 CTP 真实行为验证。
2. 不要在 wrapper 层塞入过多业务逻辑，否则后续 execution 会很难拆。
3. 不要第一阶段就碰 TraderApi，否则工期会迅速失控。
4. 不要试图直接从回调里构造 Nautilus 模型；先转内部事件，保持层次清晰。

## 下一步

如果按这条路线开工，建议接下来立即产出两样东西：

1. `阶段0预研清单`
2. `ctpwrapper.h / build.rs / lib.rs` 的空骨架