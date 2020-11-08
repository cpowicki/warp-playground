use tokio::sync::Mutex;
use warp::Filter;

use std::{convert::Infallible, sync::Arc};

mod actor;
mod context;

type AppContext = Arc<Mutex<context::Context>>;

pub fn init() -> AppContext {
    Arc::new(Mutex::new(context::Context::new()))
}

#[tokio::main]
async fn main() {
    let context: AppContext = init();
    warp::serve(actor::api::routes(context))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

pub fn with_context(
    context: AppContext,
) -> impl Filter<Extract = (AppContext,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}
