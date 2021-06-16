use async_graphql::*;
use futures::executor::block_on;

struct Query;

#[Object]
impl Query {
    /// Returns greeting message
    async fn hello(&self, name: String) -> Result<String> {
        if name.len() > 0 {
            let mut hello = String::from("Hello ");
            hello.push_str(&name);
            Ok(hello)
        } else {
            Err("Please provide a name".into())
        }
    }
}

async fn handle_request() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ hello(name: \"Antoni\") }").await;

    println!("{}", serde_json::to_string(&res).unwrap())
}

fn main() {
    let future = handle_request();
    block_on(future);
}
