mod providers;
mod tags;
pub mod trace;

use tokio::sync::OnceCell;

use crate::monitoring::providers::Providers;

static PROVIDERS: OnceCell<Providers> = OnceCell::const_new();

pub async fn init() {
    PROVIDERS.get_or_init(Providers::init).await;
}

pub fn shutdown() {
    if let Some(providers) = PROVIDERS.get() {
        providers.shutdown();
    }
}
