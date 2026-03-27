use ahash::AHashMap;
use nautilus_backtest::{config::BacktestEngineConfig, engine::BacktestEngine};
use nautilus_execution::models::{fee::FeeModelAny, fill::FillModelAny};
use nautilus_model::{
    data::{Data, QuoteTick},
    enums::{AccountType, BookType, OmsType, OrderSide, OrderType},
    identifiers::{InstrumentId, StrategyId, Venue},
    instruments::{CurrencyPair, Instrument, InstrumentAny, stubs::{audusd_sim, gbpusd_sim}},
    orders::Order,
    types::{Money, Price, Quantity},
};
use nautilus_trading::examples::strategies::{
    UcDualLegBollingerHft, UcDualLegBollingerHftConfig,
};
use rstest::rstest;

fn create_engine() -> BacktestEngine {
    let config = BacktestEngineConfig::default();
    let mut engine = BacktestEngine::new(config).unwrap();
    engine
        .add_venue(
            Venue::from("SIM"),
            OmsType::Netting,
            AccountType::Margin,
            BookType::L1_MBP,
            vec![Money::from("1_000_000 USD")],
            None,
            None,
            AHashMap::new(),
            None,
            vec![],
            FillModelAny::default(),
            FeeModelAny::default(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    engine
}

fn quote(instrument_id: InstrumentId, bid: &str, ask: &str, ts: u64) -> Data {
    Data::Quote(QuoteTick::new(
        instrument_id,
        Price::from(bid),
        Price::from(ask),
        Quantity::from("100000"),
        Quantity::from("100000"),
        ts.into(),
        ts.into(),
    ))
}

fn build_dual_leg_quotes(leg1_id: InstrumentId, leg2_id: InstrumentId) -> Vec<Data> {
    let mut quotes = Vec::new();
    let base_ts: u64 = 1_735_689_600_000_000_000;
    let interval: u64 = 1_000_000_000;
    let warmup_spreads = [
        0.1000, 0.1001, 0.0999, 0.1002, 0.0998, 0.1001, 0.0999, 0.1000,
    ];

    let mut tick: u64 = 0;
    for spread in warmup_spreads {
        let leg2_mid = 1.0000;
        let leg1_mid = leg2_mid + spread;
        quotes.push(quote(
            leg1_id,
            &format!("{:.5}", leg1_mid - 0.00005),
            &format!("{:.5}", leg1_mid + 0.00005),
            base_ts + tick * interval,
        ));
        tick += 1;
        quotes.push(quote(
            leg2_id,
            &format!("{:.5}", leg2_mid - 0.00005),
            &format!("{:.5}", leg2_mid + 0.00005),
            base_ts + tick * interval,
        ));
        tick += 1;
    }

    // Short spread signal: leg1 richens sharply vs leg2.
    quotes.push(quote(leg1_id, "1.10245", "1.10255", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "0.99995", "1.00005", base_ts + tick * interval));
    tick += 1;

    // Next quote crosses the resting sell limit on leg1, which should trigger leg2 market hedge.
    quotes.push(quote(leg1_id, "1.10255", "1.10265", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "1.00000", "1.00010", base_ts + tick * interval));
    tick += 1;

    // Mean reversion to trigger exit.
    quotes.push(quote(leg1_id, "1.10005", "1.10015", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "0.99995", "1.00005", base_ts + tick * interval));

    quotes
}

#[rstest]
fn test_uc_dual_leg_bollinger_hft_executes_passive_first_dual_leg_flow(
    audusd_sim: CurrencyPair,
    gbpusd_sim: CurrencyPair,
) {
    let mut engine = create_engine();
    let leg1 = InstrumentAny::CurrencyPair(audusd_sim);
    let leg2 = InstrumentAny::CurrencyPair(gbpusd_sim);
    let leg1_id = leg1.id();
    let leg2_id = leg2.id();
    engine.add_instrument(&leg1).unwrap();
    engine.add_instrument(&leg2).unwrap();

    let config = UcDualLegBollingerHftConfig::new(leg1_id, leg2_id, Quantity::from("100000"))
        .with_lookback_quotes(8)
        .with_entry_zscore(1.5)
        .with_exit_zscore(0.25)
        .with_entry_limit_offset_ticks(0)
        .with_entry_limit_timeout_quotes(4)
        .with_strategy_id(StrategyId::from("UC_DLB_HFT-001"));

    engine.add_strategy(UcDualLegBollingerHft::new(config)).unwrap();
    engine.add_data(build_dual_leg_quotes(leg1_id, leg2_id), None, true, true);
    engine.run(None, None, None, false).unwrap();

    let result = engine.get_result();
    assert!(result.total_orders >= 4, "expected dual-leg entry and exit orders");

    let cache = engine.kernel().cache();
    let cache = cache.borrow();
    let strategy_id = StrategyId::from("UC_DLB_HFT-001");
    let leg1_closed = cache.orders_closed(None, Some(&leg1_id), Some(&strategy_id), None, None);
    let leg2_closed = cache.orders_closed(None, Some(&leg2_id), Some(&strategy_id), None, None);

    assert!(
        leg1_closed.iter().any(|order| {
            order.order_type() == OrderType::Limit && order.order_side() == OrderSide::Sell
        }),
        "expected a passive limit entry on leg1",
    );
    assert!(
        leg2_closed.iter().any(|order| {
            order.order_type() == OrderType::Market && order.order_side() == OrderSide::Buy
        }),
        "expected a market hedge on leg2 after leg1 fill",
    );
    assert!(
        !cache.positions_closed(None, None, Some(&strategy_id), None, None).is_empty(),
        "expected closed positions after mean reversion exit",
    );
}