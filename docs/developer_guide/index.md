# Developer Guide

Guidance on developing and extending NautilusTrader, or contributing back to the project.

NautilusTrader uses a **Rust core with Python bindings** architecture:

- **Rust** handles networking, data parsing, order matching, and other performance-critical operations.
- **Python** provides the user-facing API for strategy development, configuration, and system integration.
- **PyO3** bridges the two, exposing Rust functionality to Python with minimal overhead.

This approach combines Python's simplicity and ecosystem with Rust's performance and memory safety.

## Contents

- [Environment Setup](environment_setup.md)
- [Coding Standards](coding_standards.md)
- [Rust](rust.md)
- [Python](python.md)
- [官方示例三层导航](官方示例三层导航.md)
- [GitHub Fork 高频策略排查清单](GitHub%20Fork%20高频策略排查清单.md)
- [GitHub 外部候选策略排查记录](GitHub%20外部候选策略排查记录.md)
- [Nautilus 数据库表解读](Nautilus%E6%95%B0%E6%8D%AE%E5%BA%93%E8%A1%A8%E8%A7%A3%E8%AF%BB.md)
- [国内期权接入缺口评估](%E5%9B%BD%E5%86%85%E6%9C%9F%E6%9D%83%E6%8E%A5%E5%85%A5%E7%BC%BA%E5%8F%A3%E8%AF%84%E4%BC%B0.md)
- [Live 与 Backtest 数据隔离方案](Live%E4%B8%8EBacktest%E6%95%B0%E6%8D%AE%E9%9A%94%E7%A6%BB%E6%96%B9%E6%A1%88.md)
- [pdk0007 新增入口逐项解读](pdk0007新增入口逐项解读.md)
- [pdk0007 与 Carlos 重叠入口对比](pdk0007与Carlos重叠入口对比.md)
- [Carlos 独有入口逐项解读](Carlos%E7%8B%AC%E6%9C%89%E5%85%A5%E5%8F%A3%E9%80%90%E9%A1%B9%E8%A7%A3%E8%AF%BB.md)
- [Testing](testing.md)
- [Test Datasets](test_datasets.md)
- [Docs Style](docs.md)
- [Release Notes](releases.md)
- [Adapters](adapters.md)
- [Data Testing Spec](spec_data_testing.md)
- [Execution Testing Spec](spec_exec_testing.md)
- [Benchmarking](benchmarking.md)
- [FFI Memory Contract](ffi.md)
