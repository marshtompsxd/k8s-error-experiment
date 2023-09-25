// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, DeleteParams, ObjectMeta, PostParams},
    Client, CustomResource, CustomResourceExt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::time::sleep;

#[derive(CustomResource, Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[kube(group = "nullable.se", version = "v1", kind = "ConfigMapGenerator")]
#[kube(status = "ConfigMapGeneratorStatus")]
#[kube(shortname = "cmg", namespaced)]
struct ConfigMapGeneratorSpec {
    content: String,
    expected_replicas: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct ConfigMapGeneratorStatus {
    replicas: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let dp = DeleteParams::default();
    let client = Client::try_default().await?;
    // let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());

    // let cmgcrd = ConfigMapGenerator::crd();

    // crd_api.create(&pp, &cmgcrd).await?;
    println!("{}", serde_yaml::to_string(&ConfigMapGenerator::crd())?);

    // println!("crd created");
    // sleep(Duration::from_secs(5)).await;

    let cmg_api = Api::<ConfigMapGenerator>::namespaced(client.clone(), "default");
    let cmg_name = "my-configmap-generator".to_string();
    let cmg = ConfigMapGenerator {
        metadata: ObjectMeta {
            name: Some(cmg_name.clone()),
            ..ObjectMeta::default()
        },
        spec: ConfigMapGeneratorSpec {
            content: "hello".to_string(),
            expected_replicas: 3,
        },
        status: Some(ConfigMapGeneratorStatus { replicas: 3 }),
    };

    cmg_api.create(&pp, &cmg).await?;

    let created_cmg = cmg_api.get(&cmg_name).await?;
    println!("created\n {:?}", created_cmg);
    println!(
        "rv is {}",
        created_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Updating lables...");
    let mut new_cmg: ConfigMapGenerator = created_cmg;
    new_cmg.metadata.labels = Some(BTreeMap::from([("app".to_string(), "my-app".to_string())]));
    cmg_api.replace(&cmg_name, &pp, &new_cmg).await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Updating status...");
    let mut new_cmg = updated_cmg;
    new_cmg.status = Some(ConfigMapGeneratorStatus { replicas: 2 });

    cmg_api.replace(&cmg_name, &pp, &new_cmg).await?;
    let updated_cmg = cmg_api.get(&cmg_name).await?;
    println!("updated\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating status...");
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await?;
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating status...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.status = Some(ConfigMapGeneratorStatus { replicas: 4 });
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating spec...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.spec = ConfigMapGeneratorSpec {
        content: "world".to_string(),
        expected_replicas: 3,
    };
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating spec...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.spec = ConfigMapGeneratorSpec {
        content: "world".to_string(),
        expected_replicas: 2,
    };
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating labels...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.labels = Some(BTreeMap::from([("app".to_string(), "my-app2".to_string())]));
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating finalizers...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.finalizers = Some(vec!["key/val".to_string()]);
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Updating finalizers...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.finalizers = Some(vec!["key/val".to_string()]);
    cmg_api.replace(&cmg_name, &pp, &new_cmg).await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Delete the object");
    cmg_api.delete(&cmg_name, &dp).await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("deleted\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating finalizers...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.finalizers = Some(vec!["key/val".to_string(), "key2/val2".to_string()]);
    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating uid...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.uid = Some("hhh".to_string());
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating namespace...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.namespace = Some("hhh".to_string());
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating name...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.name = Some("hhh".to_string());
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating name...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.name = None;
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating status...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.status = Some(ConfigMapGeneratorStatus { replicas: 2 });
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    println!("Status-Updating rv...");
    let mut new_cmg: ConfigMapGenerator = updated_cmg;
    new_cmg.metadata.resource_version = Some("123".to_string());
    match cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Err: {}", e),
    }
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("updated status\n {:?}", updated_cmg);
    println!(
        "rv is {}",
        updated_cmg.metadata.resource_version.as_ref().unwrap()
    );

    // println!("Updating finalizers...");
    // let mut new_cmg: ConfigMapGenerator = updated_cmg;
    // new_cmg.metadata.finalizers = Some(vec!["key/val".to_string(), "key2/val2".to_string()]);
    // cmg_api.replace(&cmg_name, &pp, &new_cmg).await?;
    // let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    // println!("updated\n {:?}", updated_cmg);
    // println!(
    //     "rv is {}",
    //     updated_cmg.metadata.resource_version.as_ref().unwrap()
    // );

    Ok(())
}
