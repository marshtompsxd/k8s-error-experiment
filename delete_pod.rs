// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use either::Either;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, PostParams},
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let p: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": "blog"
        },
        "spec": {
            "containers": [{
              "name": "blog",
              "image": "clux/blog:0.1.0"
            }],
        },
    }))?;

    let pp = PostParams::default();
    // The creation succeeds
    match pods.create(&pp, &p).await {
        Err(e) => {
            println!("This creation fails with:\n{}", e);
        }
        _ => {}
    }

    // This one will say delete has started for ...
    let dp = DeleteParams::default();
    match pods.delete("blog", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that pod has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    // This one fails with NotFound
    match pods.delete("blog", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that pod has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    Ok(())
}
