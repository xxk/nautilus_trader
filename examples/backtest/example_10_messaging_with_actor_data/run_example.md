# run_example.py 解读

## 定位

这是教学示例 10 的入口脚本，主题是“Actor 向 Strategy 发送数据”。

## 主流程

1. 创建回测环境。
2. 启动 actor 与示例策略。
3. 运行回测，让 actor 发送数据型消息。
4. 观察策略如何消费这些消息。

## 学习重点

- 看清 actor 和 strategy 的职责拆分。
