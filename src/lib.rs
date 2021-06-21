pub mod types;
pub mod config;
pub mod loader;

// #[cfg(test)]
// mod tests {
//     use tokio;
//     use kubernetes_client::apis::{ configuration::Configuration, apps_v1_api };
//     use reqwest;
//     use reqwest::ClientBuilder;

//     #[tokio::test]
//     async fn it_works() {
//         let config = Configuration{
//             base_path: "http://127.0.0.1:8080".to_owned(),
//             user_agent: Some("OpenAPI-Generator/v1.21.1/rust".to_owned()),
//             client: ClientBuilder::new().danger_accept_invalid_certs(true)
//                 .build().unwrap(),
//             basic_auth: None,
//             oauth_access_token: None,
//             bearer_access_token: None,
//             api_key: None,
//         };
//         let name = "nginx-deployment-basic";
//         let namespace = "default";
//         apps_v1_api::delete_namespaced_deployment(&config, name, namespace, None, None, Option::Some(6), None, None, None).await.unwrap();
//     }
// }