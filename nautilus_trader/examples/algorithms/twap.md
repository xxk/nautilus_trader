# twap.py 解读

## 定位

这是官方提供的 `TWAPExecAlgorithm`，作用是把一个主订单按时间均匀拆分，在给定时间窗内分批提交子订单。

## 核心输入

- `Order`
- `TimeEvent`

## 核心逻辑

1. 只接受 `MARKET` 主订单，并要求 `exec_algorithm_params` 里提供 `horizon_secs` 与 `interval_secs`。
2. 根据总时长和间隔计算切片数量，把总数量尽量均匀地下切成多个子数量；如果切出来太小或没有意义，就直接整单提交。
3. 第一笔子单会立刻提交，随后通过 `clock.set_timer` 周期性触发 `on_time_event`，持续提交剩余子单。
4. 最后一笔会直接提交主订单本体，并调用 `complete_sequence` 结束这条执行序列。

## 关键实现点

- 用 `_scheduled_sizes` 维护每个主订单尚未执行的切片列表。
- 使用向下取整来适配合约 `size_precision`，避免切片后数量非法。
- 如果主订单已经关闭，定时器回调会自动结束该序列，避免继续无效拆单。

## 停止行为

- `on_stop` 会取消全部计时器。
- `on_reset` 会清空内部切片调度状态。

## 类型

- 执行算法
