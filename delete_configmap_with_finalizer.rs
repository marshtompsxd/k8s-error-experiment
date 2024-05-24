// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use std::thread::sleep;

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
            finalizers: Some(vec!["my/someFinalizer".to_string()]),
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

    println!(
        "rv is {}",
        cm_api
            .get(&cm_name)
            .await?
            .metadata
            .resource_version
            .unwrap()
    );

    // This one will say delete has started for ConfigMap
    let dp = DeleteParams::default();
    match cm_api.delete("my-configmap", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    println!(
        "rv is {}",
        cm_api
            .get(&cm_name)
            .await?
            .metadata
            .resource_version
            .unwrap()
    );

    sleep(std::time::Duration::from_secs(5));
    // This one says delete has started for ConfigMap
    match cm_api.delete("my-configmap", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    println!(
        "rv is {}",
        cm_api
            .get(&cm_name)
            .await?
            .metadata
            .resource_version
            .unwrap()
    );

    sleep(std::time::Duration::from_secs(5));
    let mut obj_update_ds = cm_api.get(&cm_name).await?;
    obj_update_ds.metadata.deletion_timestamp = Some(
        k8s_openapi::apimachinery::pkg::apis::meta::v1::Time(chrono::Utc::now()),
    );
    match cm_api.replace("my-configmap", &pp, &obj_update_ds).await {
        Ok(p) => {
            println!("ConfigMap is updated to {:?}", p)
        }
        Err(e) => {
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    println!(
        "rv is {}",
        cm_api
            .get(&cm_name)
            .await?
            .metadata
            .resource_version
            .unwrap()
    );

    Ok(())
}
