// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use either::Either;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, Patch, PatchParams, PostParams},
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

    let cm_owner_name = "owner-configmap".to_string();
    let cm_owner = ConfigMap {
        metadata: ObjectMeta {
            name: Some(cm_owner_name.clone()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    let cm_owner_name_2 = "another-owner-configmap".to_string();
    let cm_owner_2 = ConfigMap {
        metadata: ObjectMeta {
            name: Some(cm_owner_name_2.clone()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    let cm_after_create = cm_api.create(&PostParams::default(), &cm).await.unwrap();
    println!("created cm is {:?}", cm_after_create);

    let owner_cm_after_create = cm_api
        .create(&PostParams::default(), &cm_owner)
        .await
        .unwrap();
    println!(
        "uid of owner is {:?}",
        owner_cm_after_create.clone().metadata.uid.unwrap()
    );

    let another_owner_after_create = cm_api
        .create(&PostParams::default(), &cm_owner_2)
        .await
        .unwrap();
    println!(
        "uid of another owner is {:?}",
        another_owner_after_create.clone().metadata.uid.unwrap()
    );

    let patch1 = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "ownerReferences": [{
                "apiVersion": "v1",
                "blockOwnerDeletion": true,
                "controller": true,
                "kind": "ConfigMap",
                "name": owner_cm_after_create.metadata.name.unwrap(),
                "uid": owner_cm_after_create.metadata.uid.unwrap(),
            }]
        },
    });

    match cm_api
        .patch(
            "my-configmap",
            &PatchParams::apply("myapp"),
            &Patch::Apply(&patch1),
        )
        .await
    {
        Ok(o) => println!("patched object is {:?}", o),
        Err(e) => println!("patch has failed with {}", e),
    }

    let patch2 = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "ownerReferences": [{
                "apiVersion": "v1",
                "blockOwnerDeletion": true,
                "controller": true,
                "kind": "ConfigMap",
                "name": another_owner_after_create.metadata.name.unwrap(),
                "uid": another_owner_after_create.metadata.uid.unwrap(),
            }]
        },
    });

    match cm_api
        .patch(
            "my-configmap",
            &PatchParams::apply("myapp"),
            &Patch::Apply(&patch2),
        )
        .await
    {
        Ok(o) => println!("patched object is {:?}", o),
        Err(e) => println!("patch has failed with {}", e),
    }

    match cm_api
        .patch(
            "my-configmap",
            &PatchParams::apply("myapp"),
            &Patch::Merge(&patch1),
        )
        .await
    {
        Ok(o) => println!("patched object is {:?}", o),
        Err(e) => println!("patch has failed with {}", e),
    }

    match cm_api
        .patch(
            "my-configmap",
            &PatchParams::apply("myapp"),
            &Patch::Strategic(&patch2),
        )
        .await
    {
        Ok(o) => println!("patched object is {:?}", o),
        Err(e) => println!("patch has failed with {}", e),
    }

    use std::collections::HashMap;

    // Mutating one map
    fn merge1(map1: &mut HashMap<(), ()>, map2: HashMap<(), ()>) {
        map1.extend(map2);
    }

    // Without mutation
    fn merge2(map1: HashMap<(), ()>, map2: HashMap<(), ()>) -> HashMap<(), ()> {
        map1.into_iter().chain(map2).collect()
    }

    // If you only have a reference to the map to be merged in
    fn merge_from_ref(map: &mut HashMap<(), ()>, map_ref: &HashMap<(), ()>) {
        map.extend(map_ref.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    Ok(())
}
