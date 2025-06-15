// pub use crate::ss::{MarketData, Signal};
// pub trait Strategy{
//     /// Init Strategy
//     fn on_init(&mut self);

//     /// Receive market updates.
//     fn on_tick(&mut self, data: &MarketData) -> Optional<Signal>;

//     /// Received K-line bars (such as 1 min, 5 min)
//     fn on_bar(&mut self, data: &MarketData) -> Optional<Signal>;

//     /// Optional: Periodic checks(such as periodic clearing) receive candlestick bars(such as 1 minute, 5 minutes)
//     fn on_timer(&mut self);

//     /// Optional: Strategy ended, clean up
//     fn on_exit(&mut self);
// }
