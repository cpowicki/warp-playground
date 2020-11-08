use context::AppContext;
use warp::Filter;

use std::convert::Infallible;

mod actor;
mod data;
mod context;

#[tokio::main]
async fn main() {
    let context = AppContext::init();

    let routes = actor::api::routes(context.clone()).or(data::api::routes(context));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

pub fn with_context(
    context: AppContext,
) -> impl Filter<Extract = (AppContext,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}
