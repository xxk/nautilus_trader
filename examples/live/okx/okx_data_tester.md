# okx_data_tester.py 解读

## 定位

这是 OKX 的数据测试脚本，能够覆盖 spot、swap、futures、option 等多产品数据口径。

## 主流程

1. 通过 `instrument_type` 选择要测试的产品类型。
2. 配置 OKX 数据客户端和 instrument provider。
3. 用 `DataTesterConfig` 指定报价、成交、mark price、funding rate、bars 等订阅项。
4. 运行后验证多种市场数据是否正常可用。

## 学习重点

- 这是 OKX 数据侧非常有代表性的官方诊断脚本。
