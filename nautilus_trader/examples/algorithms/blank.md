# blank.py 解读

## 定位

这是一个空白执行算法模板，用来给开发者编写自定义 `ExecAlgorithm` 提供骨架。

## 核心输入

- `Order`
- `OrderList`

## 核心逻辑

1. 提供最小 `MyExecAlgorithmConfig` 和 `MyExecAlgorithm` 结构。
2. 把 `on_start`、`on_stop`、`on_reset`、`on_save`、`on_load`、`on_order`、`on_order_list` 这些关键钩子全部留出。
3. 默认只打印收到的订单或订单列表，不执行任何拆单、调度或路由逻辑。

## 停止行为

- 默认无额外停止行为，需要开发者按自己的执行算法补充。

## 类型

- 模板 / 骨架
