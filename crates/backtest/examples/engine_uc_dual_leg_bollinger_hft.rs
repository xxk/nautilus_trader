//! Example: quote-driven dual-leg Bollinger HFT strategy using [`BacktestEngine`] directly.
//!
//! Run with:
//! `cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft`

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use ahash::AHashMap;
use nautilus_backtest::{config::BacktestEngineConfig, engine::BacktestEngine};
use nautilus_execution::models::{fee::FeeModelAny, fill::FillModelAny};
use nautilus_model::{
    data::{Data, QuoteTick},
    enums::{AccountType, BookType, OmsType},
    identifiers::{InstrumentId, Venue},
    instruments::{Instrument, InstrumentAny, stubs::{audusd_sim, gbpusd_sim}},
    types::{Money, Price, Quantity},
};
use nautilus_trading::examples::strategies::{
    UcDualLegBollingerHft, UcDualLegBollingerHftConfig,
};

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_stats(title: &str, stats: &AHashMap<String, f64>) -> String {
    let mut items: Vec<_> = stats.iter().collect();
    items.sort_by(|left, right| left.0.cmp(right.0));

    let rows = items
        .into_iter()
        .map(|(key, value)| {
            format!(
                "<tr><td>{}</td><td>{:.6}</td></tr>",
                html_escape(key),
                value
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "<section><h2>{}</h2><table><thead><tr><th>Metric</th><th>Value</th></tr></thead><tbody>{}</tbody></table></section>",
        html_escape(title),
        rows
    )
}

fn render_lines(title: &str, lines: &[String]) -> String {
    let body = if lines.is_empty() {
        "<p>None</p>".to_string()
    } else {
        format!(
            "<pre>{}</pre>",
            html_escape(&lines.join("\n\n"))
        )
    };

    format!("<section><h2>{}</h2>{}</section>", html_escape(title), body)
}

fn report_path() -> anyhow::Result<PathBuf> {
    let output_dir = std::env::current_dir()
        .context("unable to resolve current working directory")?
        .join("output")
        .join("engine_uc_dual_leg_bollinger_hft");
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("unable to create output directory: {}", output_dir.display()))?;
    Ok(output_dir.join("backtest_report.html"))
}

fn write_report(engine: &BacktestEngine, output_path: &Path) -> anyhow::Result<()> {
    let result = engine.get_result();
    let (closed_orders, closed_positions) = {
        let cache = engine.kernel().cache();
        let cache = cache.borrow();
        let closed_orders = cache
            .orders_closed(None, None, None, None, None)
            .into_iter()
            .map(|order| format!("{order}"))
            .collect::<Vec<_>>();
        let closed_positions = cache
            .positions_closed(None, None, None, None, None)
            .into_iter()
            .map(|position| format!("{position}"))
            .collect::<Vec<_>>();
        (closed_orders, closed_positions)
    };

    let mut pnl_sections = result
        .stats_pnls
        .iter()
        .map(|(currency, stats)| render_stats(&format!("PnL Stats ({currency})"), stats))
        .collect::<Vec<_>>();
    pnl_sections.sort();

    let summary = format!(
        concat!(
            "<section><h1>UC Dual Leg Bollinger HFT Backtest Report</h1>",
            "<table><tbody>",
            "<tr><td>Trader ID</td><td>{}</td></tr>",
            "<tr><td>Run ID</td><td>{}</td></tr>",
            "<tr><td>Iterations</td><td>{}</td></tr>",
            "<tr><td>Total Events</td><td>{}</td></tr>",
            "<tr><td>Total Orders</td><td>{}</td></tr>",
            "<tr><td>Total Positions</td><td>{}</td></tr>",
            "<tr><td>Elapsed Seconds</td><td>{:.6}</td></tr>",
            "<tr><td>Backtest Start</td><td>{}</td></tr>",
            "<tr><td>Backtest End</td><td>{}</td></tr>",
            "</tbody></table></section>"
        ),
        html_escape(&result.trader_id),
        html_escape(&result.run_id.map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())),
        result.iterations,
        result.total_events,
        result.total_orders,
        result.total_positions,
        result.elapsed_time_secs,
        html_escape(&result.backtest_start.map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())),
        html_escape(&result.backtest_end.map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())),
    );

    let html = format!(
        concat!(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\">",
            "<title>UC Dual Leg Bollinger HFT Backtest Report</title>",
            "<style>",
            "body{{font-family:Segoe UI,Arial,sans-serif;margin:24px;background:#f6f7f9;color:#111;}}",
            "section{{background:#fff;border:1px solid #d9dde3;border-radius:10px;padding:16px;margin-bottom:16px;}}",
            "h1,h2{{margin-top:0;}}table{{border-collapse:collapse;width:100%;}}",
            "td,th{{border:1px solid #d9dde3;padding:8px;text-align:left;vertical-align:top;}}",
            "pre{{white-space:pre-wrap;word-break:break-word;background:#f3f5f7;border-radius:8px;padding:12px;}}",
            "</style></head><body>{}{}{}{}{}</body></html>"
        ),
        summary,
        render_stats("Return Stats", &result.stats_returns),
        render_stats("General Stats", &result.stats_general),
        pnl_sections.join(""),
        format!(
            "{}{}",
            render_lines("Closed Orders", &closed_orders),
            render_lines("Closed Positions", &closed_positions)
        )
    );

    fs::write(output_path, html)
        .with_context(|| format!("unable to write report file: {}", output_path.display()))
}

fn open_report(output_path: &Path) -> anyhow::Result<()> {
    let path_str = output_path.display().to_string();
    let status = Command::new("cmd")
        .args(["/C", "start", "", &path_str])
        .status()
        .with_context(|| format!("unable to open report in browser: {}", output_path.display()))?;

    anyhow::ensure!(status.success(), "failed to launch report viewer: {}", output_path.display());
    Ok(())
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

fn build_quotes(leg1_id: InstrumentId, leg2_id: InstrumentId) -> Vec<Data> {
    let mut quotes = Vec::new();
    let base_ts: u64 = 1_735_689_600_000_000_000;
    let interval: u64 = 1_000_000_000;
    let spreads = [0.1000, 0.1001, 0.0999, 0.1002, 0.0998, 0.1001, 0.0999, 0.1000];
    let mut tick: u64 = 0;

    for spread in spreads {
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

    quotes.push(quote(leg1_id, "1.10245", "1.10255", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "0.99995", "1.00005", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg1_id, "1.10255", "1.10265", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "1.00000", "1.00010", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg1_id, "1.10005", "1.10015", base_ts + tick * interval));
    tick += 1;
    quotes.push(quote(leg2_id, "0.99995", "1.00005", base_ts + tick * interval));

    quotes
}

fn main() -> anyhow::Result<()> {
    let mut engine = BacktestEngine::new(BacktestEngineConfig::default())?;

    engine.add_venue(
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
    )?;

    let leg1 = InstrumentAny::CurrencyPair(audusd_sim());
    let leg2 = InstrumentAny::CurrencyPair(gbpusd_sim());
    let leg1_id = leg1.id();
    let leg2_id = leg2.id();
    engine.add_instrument(&leg1)?;
    engine.add_instrument(&leg2)?;

    let strategy = UcDualLegBollingerHft::new(
        UcDualLegBollingerHftConfig::new(leg1_id, leg2_id, Quantity::from("100000"))
            .with_lookback_quotes(8)
            .with_entry_zscore(1.5)
            .with_exit_zscore(0.25)
            .with_entry_limit_timeout_quotes(4),
    );
    engine.add_strategy(strategy)?;
    engine.add_data(build_quotes(leg1_id, leg2_id), None, true, true);
    engine.run(None, None, None, false)?;

    let output_path = report_path()?;
    write_report(&engine, &output_path)?;
    open_report(&output_path)?;

    println!("Backtest report saved to: {}", output_path.display());
    Ok(())
}