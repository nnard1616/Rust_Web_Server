use warp::Filter;

#[tokio::main]
async fn main() {

    /* let hello = warp::get()
        .map(|| format!("Hello, World!")); */
    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127,0,0,1], 1337))
        .await;
}

