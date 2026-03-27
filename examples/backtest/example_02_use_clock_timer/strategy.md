# strategy.py 解读

## 定位

这个文件定义 `SimpleTimerStrategy`，重点演示 `clock.set_timer` 的使用。

## 核心行为

1. `on_start` 订阅 Bar 并注册周期性定时器。
2. `on_bar` 统计处理到的市场数据。
3. `on_timer` 在固定时间间隔触发自定义逻辑。

## 学习重点

- 理解 Nautilus 里的市场事件和时间事件可以并行驱动策略。
