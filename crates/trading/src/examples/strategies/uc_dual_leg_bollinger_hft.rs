// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Quote-driven dual-leg Bollinger mean reversion strategy.
//!
//! This is a Rust-native higher-frequency variant of the `uc_dual_leg_bollinger`
//! demo strategy. It reacts on every quote update after both legs have a top-of-book
//! snapshot, computes a rolling spread z-score, then uses a passive-first sell-side:
//!
//! - leg1 enters with a resting post-only limit order,
//! - stale leg1 entry orders are canceled after a configurable number of quote snapshots,
//! - every leg1 fill immediately triggers a market hedge on leg2,
//! - exit uses market closing orders on both legs.

use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use anyhow::anyhow;
use nautilus_common::actor::{DataActor, DataActorCore};
use nautilus_model::{
    data::QuoteTick,
    enums::OrderSide,
    events::{OrderCanceled, OrderExpired, OrderFilled, OrderRejected},
    identifiers::{ClientOrderId, InstrumentId, StrategyId},
    instruments::{Instrument, InstrumentAny},
    orders::Order,
    types::{Price, Quantity},
};

use crate::strategy::{Strategy, StrategyConfig, StrategyCore};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpreadSide {
    Flat,
    Long,
    Short,
}

/// Configuration for the quote-driven dual-leg Bollinger strategy.
#[derive(Debug, Clone)]
pub struct UcDualLegBollingerHftConfig {
    pub base: StrategyConfig,
    pub leg1_instrument_id: InstrumentId,
    pub leg2_instrument_id: InstrumentId,
    pub trade_size: Quantity,
    pub lookback_quotes: usize,
    pub entry_zscore: f64,
    pub exit_zscore: f64,
    pub min_std: f64,
    pub entry_limit_offset_ticks: i32,
    pub entry_limit_timeout_quotes: usize,
}

impl UcDualLegBollingerHftConfig {
    #[must_use]
    pub fn new(
        leg1_instrument_id: InstrumentId,
        leg2_instrument_id: InstrumentId,
        trade_size: Quantity,
    ) -> Self {
        Self {
            base: StrategyConfig {
                strategy_id: Some(StrategyId::from("UC_DLB_HFT-001")),
                order_id_tag: Some("001".to_string()),
                ..Default::default()
            },
            leg1_instrument_id,
            leg2_instrument_id,
            trade_size,
            lookback_quotes: 64,
            entry_zscore: 2.0,
            exit_zscore: 0.25,
            min_std: 1e-9,
            entry_limit_offset_ticks: 0,
            entry_limit_timeout_quotes: 12,
        }
    }

    #[must_use]
    pub fn with_lookback_quotes(mut self, lookback_quotes: usize) -> Self {
        self.lookback_quotes = lookback_quotes;
        self
    }

    #[must_use]
    pub fn with_entry_zscore(mut self, entry_zscore: f64) -> Self {
        self.entry_zscore = entry_zscore;
        self
    }

    #[must_use]
    pub fn with_exit_zscore(mut self, exit_zscore: f64) -> Self {
        self.exit_zscore = exit_zscore;
        self
    }

    #[must_use]
    pub fn with_entry_limit_offset_ticks(mut self, ticks: i32) -> Self {
        self.entry_limit_offset_ticks = ticks;
        self
    }

    #[must_use]
    pub fn with_entry_limit_timeout_quotes(mut self, timeout_quotes: usize) -> Self {
        self.entry_limit_timeout_quotes = timeout_quotes;
        self
    }

    #[must_use]
    pub fn with_strategy_id(mut self, strategy_id: StrategyId) -> Self {
        self.base.strategy_id = Some(strategy_id);
        self
    }
}

/// Rust-native higher-frequency version of the UC dual-leg Bollinger strategy.
pub struct UcDualLegBollingerHft {
    core: StrategyCore,
    config: UcDualLegBollingerHftConfig,
    leg1_instrument: Option<InstrumentAny>,
    leg2_instrument: Option<InstrumentAny>,
    leg1_quote: Option<QuoteTick>,
    leg2_quote: Option<QuoteTick>,
    spreads: VecDeque<f64>,
    spread_side: SpreadSide,
    snapshot_counter: usize,
    pending_entry_order_id: Option<ClientOrderId>,
    pending_entry_side: Option<SpreadSide>,
    pending_entry_snapshot: Option<usize>,
    pending_cancel_requested: bool,
}

impl UcDualLegBollingerHft {
    #[must_use]
    pub fn new(config: UcDualLegBollingerHftConfig) -> Self {
        Self {
            core: StrategyCore::new(config.base.clone()),
            spreads: VecDeque::with_capacity(config.lookback_quotes),
            config,
            leg1_instrument: None,
            leg2_instrument: None,
            leg1_quote: None,
            leg2_quote: None,
            spread_side: SpreadSide::Flat,
            snapshot_counter: 0,
            pending_entry_order_id: None,
            pending_entry_side: None,
            pending_entry_snapshot: None,
            pending_cancel_requested: false,
        }
    }

    fn strategy_id(&self) -> StrategyId {
        StrategyId::from(self.actor_id.inner().as_str())
    }

    fn mid(quote: QuoteTick) -> f64 {
        (quote.bid_price.as_f64() + quote.ask_price.as_f64()) / 2.0
    }

    fn current_spread(&self) -> Option<f64> {
        Some(Self::mid(self.leg1_quote?) - Self::mid(self.leg2_quote?))
    }

    fn update_leg_quote(&mut self, quote: &QuoteTick) {
        if quote.instrument_id == self.config.leg1_instrument_id {
            self.leg1_quote = Some(*quote);
        } else if quote.instrument_id == self.config.leg2_instrument_id {
            self.leg2_quote = Some(*quote);
        }
    }

    fn push_spread(&mut self, spread: f64) {
        if self.spreads.len() == self.config.lookback_quotes {
            self.spreads.pop_front();
        }
        self.spreads.push_back(spread);
    }

    fn rolling_zscore(&self, spread: f64) -> Option<f64> {
        if self.spreads.len() < self.config.lookback_quotes {
            return None;
        }

        let mean = self.spreads.iter().sum::<f64>() / self.spreads.len() as f64;
        let variance = self
            .spreads
            .iter()
            .map(|value| {
                let delta = *value - mean;
                delta * delta
            })
            .sum::<f64>()
            / self.spreads.len() as f64;
        let std = variance.sqrt();
        if std <= self.config.min_std {
            return None;
        }

        Some((spread - mean) / std)
    }

    fn has_open_positions(&self) -> bool {
        let cache = self.cache();
        let strategy_id = self.strategy_id();
        !cache
            .positions_open(None, None, Some(&strategy_id), None, None)
            .is_empty()
    }

    fn leg1_entry_price(&self, order_side: OrderSide) -> anyhow::Result<Price> {
        let quote = self
            .leg1_quote
            .ok_or_else(|| anyhow!("leg1 quote not initialized"))?;
        let instrument = self
            .leg1_instrument
            .as_ref()
            .ok_or_else(|| anyhow!("leg1 instrument not initialized"))?;

        match order_side {
            OrderSide::Buy => instrument
                .next_bid_price(quote.bid_price.as_f64(), self.config.entry_limit_offset_ticks)
                .ok_or_else(|| anyhow!("unable to derive passive bid price for leg1")),
            OrderSide::Sell => instrument
                .next_ask_price(quote.ask_price.as_f64(), self.config.entry_limit_offset_ticks)
                .ok_or_else(|| anyhow!("unable to derive passive ask price for leg1")),
            side => Err(anyhow!("unsupported entry order side: {side}")),
        }
    }

    fn submit_leg1_entry(&mut self, spread_side: SpreadSide) -> anyhow::Result<()> {
        let order_side = match spread_side {
            SpreadSide::Long => OrderSide::Buy,
            SpreadSide::Short => OrderSide::Sell,
            SpreadSide::Flat => return Err(anyhow!("cannot submit flat entry order")),
        };
        let price = self.leg1_entry_price(order_side)?;
        let order = self.core.order_factory().limit(
            self.config.leg1_instrument_id,
            order_side,
            self.config.trade_size,
            price,
            None,
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let client_order_id = order.client_order_id();
        self.submit_order(order, None, None)?;
        self.pending_entry_order_id = Some(client_order_id);
        self.pending_entry_side = Some(spread_side);
        self.pending_entry_snapshot = Some(self.snapshot_counter);
        self.pending_cancel_requested = false;
        Ok(())
    }

    fn submit_leg2_hedge(&mut self, spread_side: SpreadSide, quantity: Quantity) -> anyhow::Result<()> {
        let hedge_side = match spread_side {
            SpreadSide::Long => OrderSide::Sell,
            SpreadSide::Short => OrderSide::Buy,
            SpreadSide::Flat => return Err(anyhow!("cannot hedge flat spread state")),
        };
        let order = self.core.order_factory().market(
            self.config.leg2_instrument_id,
            hedge_side,
            quantity,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        self.submit_order(order, None, None)
    }

    fn cancel_pending_entry_if_stale(&mut self) -> anyhow::Result<()> {
        let Some(client_order_id) = self.pending_entry_order_id else {
            return Ok(());
        };
        if self.pending_cancel_requested {
            return Ok(());
        }
        let Some(submitted_snapshot) = self.pending_entry_snapshot else {
            return Err(anyhow!("pending entry snapshot missing"));
        };
        if self.snapshot_counter.saturating_sub(submitted_snapshot)
            < self.config.entry_limit_timeout_quotes
        {
            return Ok(());
        }

        let cached_order = {
            let cache = self.cache();
            cache.order(&client_order_id).cloned()
        };
        if let Some(order) = cached_order {
            if order.is_open() {
                self.cancel_order(order, None)?;
                self.pending_cancel_requested = true;
            } else {
                self.clear_pending_entry();
            }
        } else {
            self.clear_pending_entry();
        }
        Ok(())
    }

    fn clear_pending_entry(&mut self) {
        self.pending_entry_order_id = None;
        self.pending_entry_side = None;
        self.pending_entry_snapshot = None;
        self.pending_cancel_requested = false;
    }

    fn process_spread_snapshot(&mut self) -> anyhow::Result<()> {
        self.snapshot_counter += 1;

        if self.pending_entry_order_id.is_some() {
            self.cancel_pending_entry_if_stale()?;
            return Ok(());
        }

        let Some(spread) = self.current_spread() else {
            return Ok(());
        };
        self.push_spread(spread);
        let Some(zscore) = self.rolling_zscore(spread) else {
            return Ok(());
        };

        if self.spread_side == SpreadSide::Flat && !self.has_open_positions() {
            if zscore <= -self.config.entry_zscore {
                self.submit_leg1_entry(SpreadSide::Long)?;
                return Ok(());
            }
            if zscore >= self.config.entry_zscore {
                self.submit_leg1_entry(SpreadSide::Short)?;
                return Ok(());
            }
        }

        match self.spread_side {
            SpreadSide::Long if zscore >= -self.config.exit_zscore => {
                self.close_all_positions(self.config.leg1_instrument_id, None, None, None, None, None, None)?;
                self.close_all_positions(self.config.leg2_instrument_id, None, None, None, None, None, None)?;
                self.spread_side = SpreadSide::Flat;
            }
            SpreadSide::Short if zscore <= self.config.exit_zscore => {
                self.close_all_positions(self.config.leg1_instrument_id, None, None, None, None, None, None)?;
                self.close_all_positions(self.config.leg2_instrument_id, None, None, None, None, None, None)?;
                self.spread_side = SpreadSide::Flat;
            }
            _ => {}
        }

        Ok(())
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.config.leg1_instrument_id == self.config.leg2_instrument_id {
            anyhow::bail!("leg instruments must differ");
        }
        if self.config.lookback_quotes < 4 {
            anyhow::bail!("lookback_quotes must be >= 4");
        }
        if self.config.entry_zscore <= 0.0 {
            anyhow::bail!("entry_zscore must be > 0");
        }
        if self.config.exit_zscore < 0.0 {
            anyhow::bail!("exit_zscore must be >= 0");
        }
        if self.config.entry_limit_timeout_quotes < 1 {
            anyhow::bail!("entry_limit_timeout_quotes must be >= 1");
        }
        Ok(())
    }
}

impl Deref for UcDualLegBollingerHft {
    type Target = DataActorCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl DerefMut for UcDualLegBollingerHft {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

impl Debug for UcDualLegBollingerHft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(UcDualLegBollingerHft))
            .field("config", &self.config)
            .field("spread_side", &self.spread_side)
            .finish()
    }
}

impl DataActor for UcDualLegBollingerHft {
    fn on_start(&mut self) -> anyhow::Result<()> {
        self.validate()?;

        let (leg1, leg2) = {
            let cache = self.cache();
            let leg1 = cache
                .instrument(&self.config.leg1_instrument_id)
                .cloned()
                .ok_or_else(|| anyhow!("leg1 instrument not found in cache"))?;
            let leg2 = cache
                .instrument(&self.config.leg2_instrument_id)
                .cloned()
                .ok_or_else(|| anyhow!("leg2 instrument not found in cache"))?;
            (leg1, leg2)
        };

        self.leg1_instrument = Some(leg1);
        self.leg2_instrument = Some(leg2);
        self.subscribe_quotes(self.config.leg1_instrument_id, None, None);
        self.subscribe_quotes(self.config.leg2_instrument_id, None, None);
        Ok(())
    }

    fn on_stop(&mut self) -> anyhow::Result<()> {
        self.cancel_all_orders(self.config.leg1_instrument_id, None, None)?;
        self.cancel_all_orders(self.config.leg2_instrument_id, None, None)?;
        self.close_all_positions(self.config.leg1_instrument_id, None, None, None, None, None, None)?;
        self.close_all_positions(self.config.leg2_instrument_id, None, None, None, None, None, None)?;
        self.unsubscribe_quotes(self.config.leg1_instrument_id, None, None);
        self.unsubscribe_quotes(self.config.leg2_instrument_id, None, None);
        Ok(())
    }

    fn on_quote(&mut self, quote: &QuoteTick) -> anyhow::Result<()> {
        if quote.instrument_id != self.config.leg1_instrument_id
            && quote.instrument_id != self.config.leg2_instrument_id
        {
            return Ok(());
        }

        self.update_leg_quote(quote);
        self.process_spread_snapshot()
    }

    fn on_order_filled(&mut self, event: &OrderFilled) -> anyhow::Result<()> {
        if Some(event.client_order_id) != self.pending_entry_order_id {
            return Ok(());
        }

        let Some(spread_side) = self.pending_entry_side else {
            return Err(anyhow!("pending entry side missing for leg1 fill"));
        };
        self.spread_side = spread_side;
        self.submit_leg2_hedge(spread_side, event.last_qty)?;

        let closed = {
            let cache = self.cache();
            cache
                .order(&event.client_order_id)
                .is_some_and(|order| order.is_closed())
        };
        if closed {
            self.clear_pending_entry();
        }
        Ok(())
    }

    fn on_order_canceled(&mut self, event: &OrderCanceled) -> anyhow::Result<()> {
        if Some(event.client_order_id) == self.pending_entry_order_id {
            self.clear_pending_entry();
        }
        Ok(())
    }

    fn on_reset(&mut self) -> anyhow::Result<()> {
        self.leg1_instrument = None;
        self.leg2_instrument = None;
        self.leg1_quote = None;
        self.leg2_quote = None;
        self.spreads.clear();
        self.spread_side = SpreadSide::Flat;
        self.snapshot_counter = 0;
        self.clear_pending_entry();
        Ok(())
    }
}

impl Strategy for UcDualLegBollingerHft {
    fn core(&self) -> &StrategyCore {
        &self.core
    }

    fn core_mut(&mut self) -> &mut StrategyCore {
        &mut self.core
    }

    fn on_order_rejected(&mut self, event: OrderRejected) {
        if Some(event.client_order_id) == self.pending_entry_order_id {
            self.clear_pending_entry();
        }
    }

    fn on_order_expired(&mut self, event: OrderExpired) {
        if Some(event.client_order_id) == self.pending_entry_order_id {
            self.clear_pending_entry();
        }
    }
}