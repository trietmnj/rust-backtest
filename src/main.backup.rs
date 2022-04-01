use std::{collections::HashMap, sync::Arc};

use barter::{
    data::handler::historical::{HistoricalCandleHandler, HistoricalDataLego},
    engine::{trader::Trader, Engine},
    event::EventTx,
    execution::{self, simulated::SimulatedExecution, Fees},
    portfolio::{
        allocator::DefaultAllocator, portfolio::MetaPortfolio,
        repository::in_memory::InMemoryRepository, risk::DefaultRisk,
    },
    statistic::summary::{
        trading::{self, TradingSummary},
        Initialiser,
    },
    strategy::strategy::{self, RSIStrategy},
    Market,
};

use barter_data::test_util;
use parking_lot;
use tokio::sync::mpsc;
use uuid::Uuid;

mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Logger & Load Config For Engine & Trader Instances Here

    let data_handler = data::store::new_mysql_store();
    // Create channel to distribute Commands to the Engine & it's Traders (eg/ Command::Terminate)
    let (_command_tx, command_rx) = mpsc::channel(20);

    // Create Event channel to listen to all Engine Events in real-time
    let (event_tx, _event_rx) = mpsc::unbounded_channel();
    let event_tx = EventTx::new(event_tx);

    // Generate unique identifier to associate an Engine's components
    let engine_id = Uuid::new_v4();

    // Create the Market(s) to be traded on (1-to-1 relationship with a Trader)
    let market = Market::new("binance", "btc_usdt".to_owned());

    // Build global shared-state MetaPortfolio (1-to-1 relationship with an Engine)
    let portfolio = Arc::new(parking_lot::Mutex::new(
        MetaPortfolio::builder()
            .engine_id(engine_id)
            .markets(vec![market.clone()])
            .starting_cash(10_000.0)
            .repository(InMemoryRepository::new())
            .allocation_manager(DefaultAllocator {
                default_order_value: 100.0,
            })
            .risk_manager(DefaultRisk {})
            .statistic_config(trading::Config {
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
                candles: vec![test_util::candle()].into_iter(),
            }))
            .strategy(RSIStrategy::new(strategy::Config { rsi_period: 14 }))
            .execution(SimulatedExecution::new(execution::simulated::Config {
                simulated_fees_pct: Fees {
                    exchange: 0.1,
                    slippage: 0.05,
                    network: 0.0,
                },
            }))
            .build()
            .expect("failed to build trader"),
    );

    // Build Engine (1-to-many relationship with Traders)

    // Create HashMap<Market, trader_command_tx> so Engine can route Commands to Traders
    let trader_command_txs = HashMap::from_iter([(market, trader_command_tx)]);

    let engine = Engine::builder()
        .engine_id(engine_id)
        .command_rx(command_rx)
        .portfolio(portfolio)
        .traders(traders)
        .trader_command_txs(trader_command_txs)
        .statistics_summary(TradingSummary::init(trading::Config {
            starting_equity: 1000.0,
            trading_days_per_year: 365,
            risk_free_return: 0.0,
        }))
        .build()
        .expect("failed to build engine");

    // Listen to Engine Events & do something with them
    // tokio::spawn(listen_to_events(event_rx));

    // --- Run Trading Session Until Remote Shutdown OR Data Feed ends naturally (ie/ backtest) ---
    engine.run().await;
    Ok(())
}

// fn listen_to_events(
//     event_rx: mpsc::UnboundedReceiver<barter::event::Event>,
// ) -> JoinHandle<mpsc::UnboundedReceiver<barter::event::Event>> {
//     print!("{}", event_rx);
//     return JoinHandle;
// }
