# bybit_options_data_collector.py 解读

## 定位

这是一个 Bybit 期权数据采集型策略脚本，文件内自带 `BybitOptionsDataCollector`，重点在持续订阅和落地期权行情，而不是交易 alpha。

## 主流程

1. 连接 Bybit 期权数据流。
2. 持续订阅期权相关行情和元数据。
3. 在策略内部做数据整理、缓存或输出。
4. 长时间运行，作为数据收集节点使用。

## 关键点

- 这是“数据采集策略”，不是“交易决策策略”。
- 它和同目录的 `README_options_data_collector.md` 互补：README 讲使用方式，这个脚本体现具体实现入口。
- 如果目标是研究 Bybit 期权数据结构，这个文件比普通交易脚本更重要。

## 学习价值

适合理解官方如何把长期运行的数据采集任务也包装成 Strategy。
