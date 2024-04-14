
// getting started
//  - https://blog.logrocket.com/building-rest-api-rust-warp/
//  serde:
//    - https://serde.rs/
//  tokio:
//    - https://tokio.rs/tokio/tutorial/
//  warp:
//    - https://github.com/seanmonstar/warp
//    - https://docs.rs/warp/latest/warp/
//    - https://blog.logrocket.com/async-crud-web-service-rust-warp/
//    - https://dev.to/bnevilleoneill/create-an-async-crud-web-service-in-rust-with-warp-386f

// usage:
//   curl --location --request POST 'localhost:3030/v1/groceries' --header 'Content-Type: application/json' --data-raw '{"name": "apple", "quantity": 3}'
//   curl --location --request PUT 'localhost:3030/v1/groceries' --header 'Content-Type: application/json' --data-raw '{"name": "apple", "quantity": 5}'
//   curl --location --request GET 'localhost:3030/v1/groceries' --header 'Content-Type: application/json'
//   curl --location --request DELETE 'localhost:3030/v1/groceries' --header 'Content-Type: application/json' --data-raw '{"name": "apple"}'

use http::StatusCode;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use warp::{http, Filter};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
  grocery_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn update_grocery_list(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().insert(item.name, item.quantity);


        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

async fn delete_grocery_list_item(
    id: Id,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().remove(&id.name);


        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}

async fn get_grocery_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.grocery_list.read();
        Ok(warp::reply::json(&*r))
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let root = warp::path::end().map(|| "API root");

    let health = warp::path!("health")
        .map(|| StatusCode::OK);

    let get_items = warp::get()
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let add_items = warp::post()
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let update_item = warp::put()
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);

    let groceries_v1 = warp::path!("v1" / "groceries")
        .and(get_items.or(add_items).or(update_item).or(delete_item));

    let routes = root.or(health).or(groceries_v1);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
