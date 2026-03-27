# okx_exec_tester.py 解读

## 定位

这是 OKX 的高覆盖执行测试脚本，用 `ExecTester` 验证 spot、margin、swap、futures、option 多种产品类型。

## 主流程

1. 先通过 `instrument_type` 选择测试模式。
2. 根据产品类型切换 symbol、数量、保证金与 quote/base 数量语义。
3. 配置 OKX data/exec client 和 reconciliation 范围。
4. 运行 `ExecTester` 检查对应产品的执行链路。

## 关键点

- 这是 OKX 示例里最值得读的调试脚本之一，因为它把多种产品语义收在一个入口里。
- `use_hyphens_in_client_order_ids=False` 体现了 OKX 的一个实际接口约束。
- 如果你要做 OKX 多产品支持，这个文件是非常好的基线模板。

## 学习价值

适合理解一个统一脚本如何覆盖多产品执行语义。
