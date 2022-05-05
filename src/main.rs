use angry_purple_tiger::AnimalName;
use axum::{
    extract::{Form, Query},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::Deserialize;
use sled;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/suggestion", get(suggestion))
        .route(
            "/registrations",
            get(check_registration).post(create_registration),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "GET  /suggestion\n\
    GET  /suggestion?seed=795d0d64-3939-4fa5-9b73-155dd8ba13f9\n\
    POST /registrations slug=angry-purple-tiger\n\
    GET  /registrations?slug=angry-purple-tiger\n"
}

async fn suggestion(Query(params): Query<SuggestionParams>) -> String {
    params
        .seed
        .parse::<AnimalName>()
        .expect("animal name")
        .to_string()
}

#[derive(Debug, Deserialize)]
struct SuggestionParams {
    #[serde(default = "uuid_string")]
    seed: String,
}

fn uuid_string() -> String {
    Uuid::new_v4().to_string()
}

async fn check_registration(Query(registration): Query<Registration>) -> StatusCode {
    let slug = registration.slug;
    let db = sled::open("/tmp/slugaas_db").expect("open");

    match db.contains_key(slug) {
        Ok(is_contained) => {
            if is_contained {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn create_registration(Form(registration): Form<Registration>) -> (StatusCode, String) {
    let slug = registration.slug;
    let db = sled::open("/tmp/slugaas_db").expect("open");

    if db.contains_key(&slug).unwrap_or(false) {
        return (StatusCode::BAD_REQUEST, "".to_string())
    }

    let (code, body) = match db.insert(&slug, "") {
        Err(why) => (StatusCode::INTERNAL_SERVER_ERROR, why.to_string()),
        Ok(_) => (StatusCode::ACCEPTED, slug),
    };
    db.flush().unwrap();

    (code, body)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Registration {
    slug: String,
}

#[tokio::test]
async fn test_suggestion_with_static() {
    let params = Query(SuggestionParams {
        seed: "795d0d64-3939-4fa5-9b73-155dd8ba13f9".to_string(),
    });
    assert_eq!("bumpy-velvet-manatee", suggestion(params).await);
}

#[tokio::test]
async fn test_suggestion_with_empty() {
    let params = Query(SuggestionParams {
        seed: "".to_string(),
    });
    assert_eq!("quaint-vermilion-gorilla", suggestion(params).await);
}
