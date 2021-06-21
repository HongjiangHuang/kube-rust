//! Create from https://github.com/kubernetes/client-go/blob/37ed584bedcad175bc0af93c02c356e4998dbeb9/tools/clientcmd/api/types.go
//! To reduce learning costs, the API is consistent with client-go

use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

// Where possible, json tags match the cli argument names.
// Top level config objects and all values required for proper functioning are not "omitempty".  
// Any truly optional piece of config is allowed to be omitted.
// Config holds the information needed to build connect to remote kubernetes clusters as a given user
// https://github.com/kubernetes/client-go/blob/37ed584bedcad175bc0af93c02c356e4998dbeb9/tools/clientcmd/api/v1/types.go#L95
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct KubeConfig {
    /// Legacy field. TODO Remove
    /// https://github.com/kubernetes/client-go/blob/37ed584bedcad175bc0af93c02c356e4998dbeb9/tools/clientcmd/api/v1/types.go#L33
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Legacy field. TODO Remove
    /// https://github.com/kubernetes/client-go/blob/37ed584bedcad175bc0af93c02c356e4998dbeb9/tools/clientcmd/api/v1/types.go#L38
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Preferences holds general information to be use for cli interactions
    #[serde(rename = "preferences")]
    #[serde(default = "default_preferences")]
    pub preferences: Preferences,

    /// Clusters is a vec of referencable names to cluster configs
    #[serde(rename = "clusters")]
    pub clusters: Vec<NamedCluster>,

    /// AuthInfos is a map of referencable names to user configs
    #[serde(rename = "users")]
    pub auth_infos: Vec<NamedAuthInfo>,

    /// Referencable names to context configs
    #[serde(rename = "contexts")]
    pub contexts: Vec<NamedContext>,

    /// The name of the context that you would like to use by default
    #[serde(rename = "current-context")]
    pub current_context: Option<String>,

    /// Additional information for extenders so that reads and writes don't clobber unknown fields.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<NamedExtension>>,
}

pub fn default_preferences() -> Preferences {
    let preferences = Preferences{
        colors: None,
        extensions: None
    };
    preferences
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Preferences {
    #[serde(rename = "colors", skip_serializing_if = "Option::is_none")]
    pub colors: Option<bool>,

    /// Extensions holds additional information. 
    /// This is useful for extenders so that reads and writes don't clobber unknown fields
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<NamedExtension>>,
}

/// Cluster contains information about how to communicate with a kubernetes cluster
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Server is the address of the kubernetes cluster (https://hostname:port).
    #[serde(rename = "server")]
    pub server: String,

    /// The tls_server_name is used to check server certificate. If TLSServerName is empty, the hostname used to contact the server is used.
    #[serde(rename = "tls-server-name", skip_serializing_if = "Option::is_none")]
    pub tls_server_name: Option<String>,

    #[serde(rename = "insecure-skip-tls-verify", skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,

    /// Certificate_authority is the path to a cert file for the certificate authority.
    #[serde(rename = "certificate-authority", skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,

    /// PEM-encoded certificate authority certificates. Overrides `certificate_authority`
    #[serde(rename = "certificate-authority-data", skip_serializing_if = "Option::is_none")]
    pub certificate_authority_data: Option<String>,

    /// ProxyURL is the URL to the proxy to be used for all requests made by this
	/// client. URLs with "http", "https", and "socks5" schemes are supported.  If
	/// this configuration is not provided or the empty string, the client
	/// attempts to construct a proxy configuration from http_proxy and
	/// https_proxy environment variables. If these environment variables are not
	/// set, the client does not attempt to proxy requests.
	///
	/// socks5 proxying does not currently support spdy streaming endpoints (exec,
	/// attach, port forward).
    #[serde(rename = "proxy-url", skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    /// Extensions holds additional information. This is useful for extenders so that reads and writes don't clobber unknown fields
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<NamedExtension>>,
}

/// AuthInfo contains information that describes identity information.  
/// This is use to tell the kubernetes cluster who you are.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthInfo {
    /// The client_certificate is the path to a client cert file for TLS.
    #[serde(rename = "client-certificate", skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,

    /// The client_certificate_data contains PEM-encoded data from a client cert file for TLS. Overrides client_certificate
    #[serde(rename = "client-certificate-data", skip_serializing_if = "Option::is_none")]
    pub client_certificate_data: Option<String>,

    /// The client_key is the path to a client key file for TLS.
    #[serde(rename = "client-key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    /// The client_key_data contains PEM-encoded data from a client key file for TLS. Overrides client_key
    #[serde(rename = "client-key-data", skip_serializing_if = "Option::is_none")]
    pub client_key_data: Option<String>,

    /// Token is the bearer token for authentication to the kubernetes cluster.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// TokenFile is a pointer to a file that contains a bearer token (as described above).  
    /// If both Token and TokenFile are present, Token takes precedence.
    #[serde(rename = "tokenFile", skip_serializing_if = "Option::is_none")]
    pub token_file: Option<String>,

    // Impersonate is the username to imperonate.  The name matches the flag.
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub impersonate: Option<String>,

	// ImpersonateGroups is the groups to imperonate.
    #[serde(rename = "as-groups", skip_serializing_if = "Option::is_none")]
    pub impersonate_groups: Option<Vec<String>>,

	// ImpersonateUserExtra contains additional information for impersonated user.
    #[serde(rename = "as-user-extra", skip_serializing_if = "Option::is_none")]
    pub impersonate_user_extra: Option<HashMap<String, Vec<String>>>,

    /// Username is the username for basic authentication to the kubernetes cluster.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Password is the password for basic authentication to the kubernetes cluster.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// AuthProvider specifies a custom authentication plugin for the kubernetes cluster.
    #[serde(rename = "auth-provider", skip_serializing_if = "Option::is_none")]
    pub auth_provider: Option<AuthProviderConfig>,

    /// Exec specifies a custom exec-based authentication plugin for the kubernetes cluster.
    #[serde(rename = "exec", skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecConfig>,

    // Extensions holds additional information. This is useful for extenders so that reads and writes don't clobber unknown fields
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<NamedExtension>>
}

/// Context is a tuple of references to a cluster (how do I communicate with a kubernetes cluster), 
/// a user (how do I identify myself), and a namespace (what subset of resources do I want to work with)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// Cluster is the name of the cluster for this context
    #[serde(rename = "cluster")]
    pub cluster: String,

    /// AuthInfo is the name of the authInfo for this context
    #[serde(rename = "user")]
    pub auth_info: String,

    /// Namespace is the default namespace to use on unspecified requests
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Extensions holds additional information. 
    /// This is useful for extenders so that reads and writes don't clobber unknown fields
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<NamedExtension>>,
}

/// NamedCluster relates nicknames to cluster information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedCluster {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "cluster")]
    pub cluster: Cluster,
}

/// NamedContext relates nicknames to context information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedContext {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "context")]
    pub context: Context,
}

/// NamedAuthInfo associates name with authentication.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedAuthInfo {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "user")]
    pub user: AuthInfo,
}

/// NamedExtention associates name with extension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedExtension {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "extension")]
    pub extension: serde_json::Value,
}

/// AuthProviderConfig holds the configuration for a specified auth provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthProviderConfig {
    #[serde(rename = "name")]
    pub name: String,
    
    #[serde(rename = "config")]
    pub config: HashMap<String, String>,
}

/// ExecConfig specifies a command to provide client credentials. The command is exec'd
/// and outputs structured stdout holding credentials.
/// 
/// See the client.authentication.k8s.io API group for specifications of the exact input
/// and output format
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecConfig {
    /// Command to execute.
    #[serde(rename = "command")]
    pub command: String,

    /// Arguments to pass to the command when executing it.
    #[serde(rename = "args")]
    pub args: Option<Vec<String>>,

	/// Env defines additional environment variables to expose to the process. 
    /// These are unioned with the host's environment, as well as variables client-go uses
	/// to pass argument to the plugin.
    #[serde(rename = "env")]
    pub env: Option<Vec<ExecEnvVar>>,

    /// Preferred input version of the ExecInfo. 
    /// 
    /// The returned ExecCredentials MUST use the same encoding version as the input.
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// This text is shown to the user when the executable doesn't seem to be
	/// present. For example, `brew install foo-cli` might be a good InstallHint for
	/// foo-cli on Mac OS systems.
    // #[serde(rename = "installHint", skip_serializing_if = "Option::is_none")]
    #[serde(rename = "install_hint")]
    pub install_hint: String,

    /// ProvideClusterInfo determines whether or not to provide cluster information,
	/// which could potentially contain very large CA data, to this exec plugin as a
	/// part of the KUBERNETES_EXEC_INFO environment variable. By default, it is set
	/// to false. Package k8s.io/client-go/tools/auth/exec provides helper methods for
	/// reading this environment variable.
    #[serde(rename = "provideClusterInfo")]
    pub provide_cluster_info: bool,

    /// InteractiveMode determines this plugin's relationship with standard input. Valid
	/// values are "Never" (this exec plugin never uses standard input), "IfAvailable" (this
	/// exec plugin wants to use standard input if it is available), or "Always" (this exec
	/// plugin requires standard input to function). See ExecInteractiveMode values for more
	/// details.
	///
	/// If APIVersion is client.authentication.k8s.io/v1alpha1 or
	/// client.authentication.k8s.io/v1beta1, then this field is optional and defaults
	/// to "IfAvailable" when unset. Otherwise, this field is required.
    #[serde(rename = "interactiveMode", skip_serializing_if = "Option::is_none")]
    pub interactive_mode: Option<String>
}

/// ExecEnvVar is used for setting environment variables when executing an exec-based
/// credential plugin.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecEnvVar {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String
}