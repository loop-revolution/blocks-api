use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql_subscription, Response};
use block_tools::{get_pool, optional_env_db};
use loop_api::{
	graphql::{build_schema, ContextData, Schema},
	sentry::sentry,
};
use std::{convert::Infallible, env};
use warp::{
	http::{header, Method, Response as HttpResponse},
	Filter,
};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
	// Logging
	env::set_var(
		"RUST_LOG",
		"loop_api=info,loop_api=debug,block-tools=info,block-tools=debug",
	);
	pretty_env_logger::init();
	let log = warp::log::custom(|info| {
		info!("{} in {:?}", info.status(), info.elapsed());
	});

	// Setup sentry guards (if DSN is provided)
	let _guard = sentry();

	let db_url = optional_env_db();

	// Establish a connection to the DB
	let pool = db_url.map(|url| get_pool(&url));

	// Connect the GraphQL Resolvers
	let schema = build_schema();

	// The route for GraphQL Requests
	let graphql_post = warp::header::optional::<String>("authorization")
		.and(async_graphql_warp::graphql(schema.clone()))
		.and_then(
			move |token: Option<String>,
			      (schema, mut request): (Schema, async_graphql::Request)| {
				// Add the database to the GraphQL Context
				let pool = pool.clone();
				async move {
					if let Some(pool) = pool {
						request = request.data(ContextData {
							pool,
							auth_token: token,
						});
					}
					// Execute the request & return it
					let resp = schema.execute(request).await;
					Ok::<_, Infallible>(Response::from(resp))
				}
			},
		);

	let graphql_playground = warp::path::end().and(warp::get()).map(|| {
		HttpResponse::builder()
			.header("content-type", "text/html")
			.body(playground_source(
				GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
			))
	});

	// Allow access from other domains
	let cors = build_cors();

	let port = get_port();
	let routes = graphql_subscription(schema)
		.or(graphql_playground)
		.or(graphql_post)
		.with(log)
		.with(cors);

	log::info!("API running on http://0.0.0.0:{}", port);

	warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

/// Use PORT variable default to 4000
fn get_port() -> u16 {
	match env::var("PORT") {
		Ok(str) => str.parse().expect("PORT variable was invalid"),
		_ => 4000,
	}
}

fn build_cors() -> warp::filters::cors::Builder {
	warp::cors()
		.allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
		.allow_any_origin()
		.allow_headers(&[header::CONTENT_TYPE, header::AUTHORIZATION])
}
