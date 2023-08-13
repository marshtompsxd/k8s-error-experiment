// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use either::Either;
use k8s_openapi::api::apps::v1::{StatefulSet, StatefulSetSpec};
use k8s_openapi::api::core::v1::{Pod, PodTemplateSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;
use kube::{
    api::{Api, DeleteParams, ObjectMeta, PostParams},
    Client,
};
use serde_json::json;
use std::thread;
use std::time;
use std::{collections::BTreeMap, vec};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
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

    let sts_name = "my-statefulset".to_string();
    let sts = StatefulSet {
        metadata: ObjectMeta {
            name: Some(sts_name.clone()),
            ..ObjectMeta::default()
        },
        spec: Some(StatefulSetSpec {
            replicas: Some(1),
            selector: metav1::LabelSelector {
                match_labels: Some(BTreeMap::from([(
                    "app".to_string(),
                    "my-statefulset".to_string(),
                )])),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    name: Some("blog".to_string()),
                    labels: Some(BTreeMap::from([(
                        "app".to_string(),
                        "my-statefulset".to_string(),
                    )])),
                    ..ObjectMeta::default()
                }),
                spec: p.spec.clone(),
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    let sts_api: Api<StatefulSet> = Api::default_namespaced(client);

    let pp = PostParams::default();
    // The creation succeeds
    match sts_api.create(&pp, &sts).await {
        Err(e) => {
            println!("This creation fails with:\n{}", e);
        }
        _ => {}
    }

    thread::sleep(time::Duration::from_secs(10));

    // This one will say delete has started for ...
    let dp = DeleteParams::default();
    match sts_api.delete("my-statefulset", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that sts has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    // This one fails with NotFound
    match sts_api.delete("my-statefulset", &dp).await {
        Ok(e) => match e {
            Either::Left(p) => println!("delete has started for {:?}", p),
            Either::Right(s) => println!("confirm that sts has gone {:?}", s),
        },
        Err(e) => println!("delete has failed with {}", e),
    }

    Ok(())
}
