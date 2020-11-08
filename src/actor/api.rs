use std::convert::Infallible;

use warp::Rejection;
use warp::{hyper::StatusCode, reply::json, Filter, Reply};

use crate::{with_context, AppContext};

fn get_actor_id(
  context: AppContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("actor" / u32)
      .and(warp::get())
      .and(with_context(context))
      .and_then(get_actor)
}

fn post_actor(
    context: AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("actor")
        .and(warp::post())
        .and(with_context(context))
        .and_then(create_actor)
}

fn get_all_actors(
  context: AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("actor")
      .and(warp::get())
      .and(with_context(context))
      .and_then(get_all)
}

pub async fn get_actor(id: u32, context: AppContext) -> Result<Box<dyn Reply>, Infallible> {
  let state = context.lock().await;

  match state.get_actor(id) {
      Some(actor) => Ok(Box::new(json(actor))),
      None => Ok(Box::new(StatusCode::NOT_FOUND)),
  }
}

pub async fn create_actor(context: AppContext) -> Result<impl Reply, Infallible> {
  let mut state = context.lock().await;
  state.add_actor();

  Ok(warp::reply::reply())
}

pub async fn get_all(context: AppContext) -> Result<Box<dyn Reply>, Infallible> {
  let state = context.lock().await;
  let actors = json(&state.get_actors());

  Ok(Box::new(actors))
}

pub fn routes(context: AppContext) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  get_all_actors(context.clone())
    .or(get_actor_id(context.clone()))
    .or(post_actor(context))
}