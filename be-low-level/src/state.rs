use deadpool_redis::Pool;
use salvo::prelude::*;

use chrono::{Local, NaiveDate};

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: Pool,
}

// Implement Handler so we can .hoop(state)
#[async_trait]
impl Handler for AppState {
    async fn handle(
        &self,
        _req: &mut Request,
        dep: &mut Depot,
        _res: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        dep.inject(self.clone());
    }
}

pub struct GameState {
    pub wins_today: u32,
    pub last_reset_date: NaiveDate,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            wins_today: 0,
            last_reset_date: Local::now().date_naive(),
        }
    }
}
