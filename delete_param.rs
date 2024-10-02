// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use either::Either;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, DeleteParams, ObjectMeta, PostParams, Preconditions},
    Client,
};

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

    let cm_from_get = cm_api.get(&cm_name).await.unwrap();
    let rv = cm_from_get
        .metadata
        .resource_version
        .unwrap()
        .parse::<i32>()
        .unwrap();

    // Use a wrong resource version in the precondition
    let dp = DeleteParams::default().preconditions(Preconditions {
        resource_version: Some((rv - 1).to_string()),
        uid: None,
    });
    match cm_api.delete("my-configmap", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    // Use a wrong uid in the precondition
    let dp2 = DeleteParams::default().preconditions(Preconditions {
        resource_version: None,
        uid: Some("hhh".to_string()),
    });
    match cm_api.delete("my-configmap", &dp2).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    // Use a correct resource version in the precondition
    let dp3 = DeleteParams::default().preconditions(Preconditions {
        resource_version: Some(rv.to_string()),
        uid: None,
    });
    match cm_api.delete("my-configmap", &dp3).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that configmap has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    Ok(())
}
