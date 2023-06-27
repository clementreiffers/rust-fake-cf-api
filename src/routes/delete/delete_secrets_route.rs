use actix_web::{delete, web, HttpResponse};
use k8s_openapi::api::core::v1::Secret;
use kube::api::{DeleteParams, ListParams};
use kube::Api;
use serde_json::json;

use crate::routes::kube::create_kube_client;

#[delete("/client/v4/accounts/{accounts}/workers/scripts/{scripts}/secrets/{secrets}")]
pub async fn delete_secrets(path: web::Path<(String, String, String)>) -> HttpResponse {
    let (accounts, scripts, secrets) = path.into_inner();
    let secret_name = format!("{}.{}.{}", accounts, scripts, secrets);
    let mut success = false;

    let client = create_kube_client().await;
    let kube_secret: Api<Secret> = Api::namespaced(client.clone(), "default");
    let label_selector = format!("accounts={},scripts={}", accounts, scripts);
    let list_params = ListParams::default().labels(&label_selector);
    let secrets_list = kube_secret
        .list(&list_params)
        .await
        .expect("failed to get list");

    for items in secrets_list.items {
        let name = items.metadata.name.expect("failed to get name");
        if name == secret_name {
            let dp = DeleteParams::default();
            success = match kube_secret.delete(&*name, &dp).await {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }

    let response = json!({
        "result": null,
        "success": success,
        "errors": [],
        "messages": []
    });
    HttpResponse::Ok().body(response.to_string())
}
