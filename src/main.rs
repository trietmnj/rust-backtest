use parking_lot::Mutex;
use serde_json;
use std::{collections::HashMap, fs, sync::Arc};

use anyhow;
use barter::{
    data::handler::historical::{HistoricalCandleHandler, HistoricalDataLego},
    engine::{trader::Trader, Engine},
    event::{Event, EventTx},
    execution::{
        simulated::{Config as ExecutionConfig, SimulatedExecution},
        Fees,
    },
    portfolio::{allocator::DefaultAllocator, portfolio::MetaPortfolio, risk::DefaultRisk},
    statistic::summary::{
        trading::{Config as StatisticConfig, TradingSummary},
        Initialiser,
    },
    strategy::strategy::{Config as StrategyConfig, RSIStrategy},
};
use barter::{portfolio::repository::in_memory::InMemoryRepository, Market};
use barter_data::model::Candle;
use tokio::sync::mpsc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_cmd_tx, cmd_rx) = mpsc::channel(20);
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let event_tx = EventTx::new(event_tx);

    let engine_id = Uuid::new_v4();
    let market = Market::new("binance", "btc_usdt".to_owned());

    // portfolio
    let portfolio = Arc::new(Mutex::new(
        MetaPortfolio::builder()
            .engine_id(engine_id)
            .markets(vec![market.clone()])
            .starting_cash(10_000.0)
            .repository(InMemoryRepository::new())
            .allocation_manager(DefaultAllocator {
                default_order_value: 100.0,
            })
            .risk_manager(DefaultRisk {})
            .statistic_config(StatisticConfig {
                starting_equity: 10_000.0,
                trading_days_per_year: 365,
                risk_free_return: 0.0,
            })
            .build_and_init()
            .expect("failed to build & initialise MetaPortfolio"),
    ));

    // Build Trader(s)
    let mut traders = Vec::new();

    // Create channel for each Trader so the Engine can distribute Commands to it
    let (trader_command_tx, trader_command_rx) = mpsc::channel(10);

    traders.push(
        Trader::builder()
            .engine_id(engine_id)
            .market(market.clone())
            .command_rx(trader_command_rx)
            .event_tx(event_tx.clone())
            .portfolio(Arc::clone(&portfolio))
            .data(HistoricalCandleHandler::new(HistoricalDataLego {
                exchange: "binance",
                symbol: "btcusdt".to_string(),
                candles: load_candles_from_json().into_iter(),
            }))
            .strategy(RSIStrategy::new(StrategyConfig { rsi_period: 14 }))
            .execution(SimulatedExecution::new(ExecutionConfig {
                simulated_fees_pct: Fees {
                    exchange: 0.1,
                    slippage: 0.05,
                    network: 0.0,
                },
            }))
            .build()
            .expect("failed to build trader"),
    );

    let trader_command_txs = HashMap::from_iter([(market, trader_command_tx)]);

    // engine
    let engine = Engine::builder()
        .engine_id(engine_id)
        .command_rx(cmd_rx)
        .portfolio(portfolio)
        .traders(traders)
        .trader_command_txs(trader_command_txs)
        .statistics_summary(TradingSummary::init(StatisticConfig {
            starting_equity: 1000.0,
            trading_days_per_year: 365,
            risk_free_return: 0.0,
        }))
        .build()
        .expect("failed to build engine");

    // Listen to Engine Events & do something with them
    tokio::spawn(listen_to_events(event_rx));

    // --- Run Trading Session Until Remote Shutdown OR Data Feed ends naturally (ie/ backtest) ---
    engine.run().await;
    Ok(())
}

async fn listen_to_events(mut event_rx: mpsc::UnboundedReceiver<barter::event::Event>) {
    while let Some(event) = event_rx.recv().await {
        match event {
            Event::Market(_) => {
                // Market Event occurred in Engine
            }
            Event::Signal(signal) => {
                // Signal Event occurred in Engine
                println!("{signal:?}");
            }
            Event::SignalForceExit(_) => {
                // SignalForceExit Event occurred in Engine
            }
            Event::OrderNew(new_order) => {
                // OrderNew Event occurred in Engine
                println!("{new_order:?}");
            }
            Event::OrderUpdate => {
                // OrderUpdate Event occurred in Engine
            }
            Event::Fill(fill_event) => {
                // Fill Event occurred in Engine
                println!("{fill_event:?}");
            }
            Event::PositionNew(new_position) => {
                // PositionNew Event occurred in Engine
                println!("{new_position:?}");
            }
            Event::PositionUpdate(updated_position) => {
                // PositionUpdate Event occurred in Engine
                println!("{updated_position:?}");
            }
            Event::PositionExit(exited_position) => {
                // PositionExit Event occurred in Engine
                println!("{exited_position:?}");
            }
            Event::Balance(balance_update) => {
                // Balance update Event occurred in Engine
                println!("{balance_update:?}");
            }
        }
    }
}

fn load_candles_from_json() -> Vec<Candle> {
    let candles = fs::read_to_string("tests/candles.json").expect("failed to read file");

    serde_json::from_str(&candles).expect("failed to parse candles String")
}
