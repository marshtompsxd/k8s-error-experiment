// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use either::Either;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, DeleteParams, ObjectMeta, PostParams},
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), "default");
    let cm_name = "my-configmap".to_string();
    let cm = ConfigMap {
        metadata: ObjectMeta {
            name: Some(cm_name.clone()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    let pp = PostParams::default();
    // The creation succeeds
    match cm_api.create(&pp, &cm).await {
        Err(e) => {
            println!("This creation fails with:\n{}", e);
        }
        _ => {}
    }

    // This one will say confirm that configmap has gone
    let dp = DeleteParams::default();
    match cm_api.delete("my-configmap", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    // This one fails with NotFound
    match cm_api.delete("my-configmap", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    Ok(())
}
