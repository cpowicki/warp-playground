use std::convert::Infallible;

use warp::Rejection;
use warp::{reply::json, Filter, Reply};

use crate::context::AppContext;
use crate::with_context;

fn get_context_data(context: AppContext) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("data")
      .and(warp::get())
      .and(with_context(context))
      .and_then(get_data)
}

fn post_context_data(context: AppContext) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("data" / u8)
      .and(warp::get())
      .and(with_context(context))
      .and_then(put_data)
}


async fn get_data(context: AppContext) -> Result<Box<dyn Reply>, Infallible> {
  let state = context.lock().await;

  let json = json(state.get_data());
  Ok(Box::new(json))
}

async fn put_data(value: u8, context: AppContext) -> Result<impl Reply, Infallible> {
  let mut state = context.lock().await;

  state.get_data_mut().add(value);

  Ok(warp::reply::reply())
}


pub fn routes(context: AppContext) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  get_context_data(context.clone()).or(post_context_data(context))
}