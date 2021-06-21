use crate::types::KubeConfig;
use serde_yaml;
use std::{fs, path::Path, u8};

pub fn load(data: &[u8]) -> Result<KubeConfig, serde_yaml::Error> {
    let result: Result<KubeConfig, _> = serde_yaml::from_slice(data);
    return result;
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<KubeConfig, serde_yaml::Error> {
    let data = fs::read(path).unwrap();
    return load(&data);
}

#[cfg(test)]
mod tests {
    use crate::{
        loader,
        types::{
            default_preferences, AuthInfo, Cluster, Context, KubeConfig, NamedAuthInfo,
            NamedCluster, NamedContext, NamedExtension,
        },
    };
    use std::{fs};

    impl PartialEq<KubeConfig> for KubeConfig {
        fn eq(&self, other: &KubeConfig) -> bool {
            let result1 = serde_json::to_string(self);
            let result2 = serde_json::to_string(other);

            if result1.is_ok() && result2.is_ok() {
                return result1.unwrap_or_default() == result2.unwrap_or_default();
            }

            return false;
        }
    }

    fn mock_kubeconfig() -> KubeConfig {
        let named_auth_info = NamedAuthInfo {
            name: String::from("minikube"),
            user: AuthInfo {
                client_certificate: Some(String::from(
                    "/Users/albert/.minikube/profiles/minikube/client.crt",
                )),
                client_certificate_data: None,
                client_key: Some(String::from(
                    "/Users/albert/.minikube/profiles/minikube/client.key",
                )),
                client_key_data: None,
                token: None,
                token_file: None,
                impersonate: None,
                impersonate_groups: None,
                impersonate_user_extra: None,
                username: None,
                password: None,
                auth_provider: None,
                exec: None,
                extensions: None,
            },
        };
        let named_cluster = NamedCluster {
            name: String::from("minikube"),
            cluster: Cluster {
                server: String::from("https://127.0.0.1:52147"),
                tls_server_name: None,
                insecure_skip_tls_verify: Some(true),
                certificate_authority: Some(String::from("/Users/albert/.minikube/ca.crt")),
                certificate_authority_data: None,
                proxy_url: None,
                extensions: Some(vec![NamedExtension {
                    name: String::from("cluster_info"),
                    extension: serde_json::json!({
                        "version": "v1.20.0",
                        "provider": "minikube.sigs.k8s.io",
                        "last-update": "Sun, 20 Jun 2021 18:10:48 CST",
                    }),
                }]),
            },
        };
        let named_context = NamedContext {
            name: String::from("minikube"),
            context: Context {
                cluster: String::from("minikube"),
                auth_info: String::from("minikube"),
                namespace: Some(String::from("default")),
                extensions: Some(vec![NamedExtension {
                    name: String::from("cluster_info"),
                    extension: serde_json::json!({
                        "version": "v1.20.0",
                        "provider": "minikube.sigs.k8s.io",
                        "last-update": "Sun, 20 Jun 2021 18:10:48 CST",
                    }),
                }]),
            },
        };
        KubeConfig {
            kind: Some(String::from("Config")),
            api_version: Some(String::from("v1")),
            preferences: default_preferences(),
            clusters: vec![named_cluster],
            auth_infos: vec![named_auth_info],
            contexts: vec![named_context],
            current_context: Some(String::from("minikube")),
            extensions: None,
        }
    }

    fn mock_kubeconfig_string() -> String {
        let cfgraw = r#"
        kind: Config
        apiVersion: v1
        preferences: {}
        clusters:
          - name: minikube
            cluster:
              server: "https://127.0.0.1:52147"
              insecure-skip-tls-verify: true
              certificate-authority: /Users/albert/.minikube/ca.crt
              extensions:
                - name: cluster_info
                  extension:
                    last-update: "Sun, 20 Jun 2021 18:10:48 CST"
                    provider: minikube.sigs.k8s.io
                    version: v1.20.0
        users:
          - name: minikube
            user:
              client-certificate: /Users/albert/.minikube/profiles/minikube/client.crt
              client-key: /Users/albert/.minikube/profiles/minikube/client.key
        contexts:
          - name: minikube
            context:
              cluster: minikube
              user: minikube
              namespace: default
              extensions:
                - name: cluster_info
                  extension:
                    last-update: "Sun, 20 Jun 2021 18:10:48 CST"
                    provider: minikube.sigs.k8s.io
                    version: v1.20.0
        current-context: minikube
        "#;
        return String::from(cfgraw);
    }

    #[tokio::test]
    async fn eq() {
        let kubeconfig = mock_kubeconfig();
        assert_eq!(kubeconfig.clone(), kubeconfig);
    }

    #[tokio::test]
    async fn load() {
        let kubeconfig_string = mock_kubeconfig_string();
        let kubeconfig = loader::load(kubeconfig_string.as_bytes()).unwrap();
        assert_eq!(kubeconfig, mock_kubeconfig());
    }

    #[tokio::test]
    async fn load_from_file() {
        let kubeconfig2_string = mock_kubeconfig_string();
        fs::write("./test-kube-config.yaml", kubeconfig2_string).unwrap();

        let kubeconfig = loader::load_from_file("./test-kube-config.yaml").unwrap();

        assert_eq!(kubeconfig, mock_kubeconfig());
    }
}
