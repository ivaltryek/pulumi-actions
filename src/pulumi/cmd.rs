use super::helper::{run_cmd_print_output, set_env_var};
use clap::{Parser, ValueEnum};
use core::panic;
use std::option::Option;

const PULUMI_BIN_PATH: &str = "/root/.pulumi/bin/pulumi";

#[derive(Parser, Debug)]
#[command(author="Meet Vasani", version, about="runs pulumi actions", long_about = None)]
pub struct Args {
    #[arg(short = 's', long = "stack")]
    /// pulumi stack name
    pub stack_name: String,

    #[arg(long = "pulumi-cloud-token")]
    /// Pulumi token - in case of using Pulumi cloud. (Optional)
    pub pulumi_cloud_token: Option<String>,

    #[arg(long = "s3-bucket")]
    /// S3 Bucket name - in case if backend is s3. exmaple: "s3://my-bucket"
    pub s3_bucket_name: Option<String>,

    #[arg(short = 'b', long = "backend")]
    /// backend type
    pub backend: Backend,

    #[arg(short = 'p', long = "path")]
    /// Path to stack's index.ts
    pub stack_path: String,

    #[arg(short = 'r', long = "runtime")]
    /// Path to stack's index.ts
    pub runtime: Runtime,

    #[arg(long = "preview", default_value_t = false)]
    /// preview the stack
    pub preview: bool,

    #[arg(long = "apply", default_value_t = false)]
    /// preview the stack
    pub apply: bool,

    #[arg(long = "init", default_value_t = false)]
    /// init the stack if does not exist
    pub init: bool,

    #[arg(long = "passphrase")]
    /// stack passphrase
    pub passphrase: String,

    #[arg(long = "install-deps", default_value_t = false)]
    /// Install dependencies of language runtime,
    /// consider setting true only when you want to preview or apply the stack.
    pub install_deps: bool,
}

impl Args {
    // Initialise the struct
    pub fn new(
        stack_name: String,
        pulumi_cloud_token: Option<String>,
        s3_bucket_name: Option<String>,
        backend: Backend,
        stack_path: String,
        runtime: Runtime,
        preview: bool,
        apply: bool,
        init: bool,
        passphrase: String,
        install_deps: bool,
    ) -> Self {
        Self {
            stack_name,
            pulumi_cloud_token,
            s3_bucket_name,
            backend,
            stack_path,
            runtime,
            preview,
            apply,
            init,
            passphrase,
            install_deps,
        }
    }

    // notify user if certain backend type is set and it's required values are not set.
    pub fn notify_user_if_empty(&self) {
        match &self.backend {
            &Backend::Pulumicloud => {
                if !self.pulumi_cloud_token.is_some() {
                    panic!("Pulumi cloud token is required for backend type `pulumicloud`");
                } else {
                    self.set_token_as_env_var()
                }
            }
            &Backend::S3 => {
                if !self.s3_bucket_name.is_some() {
                    panic!("S3 bucket name is required for backend type `s3`");
                }
            }
        }
    }

    // set `pulumi_cloud_token` if it is set and `backend` is set to pulumicloud
    fn set_token_as_env_var(&self) {
        let token_key = String::from("PULUMI_ACCESS_TOKEN");
        match &self.pulumi_cloud_token {
            Some(value) => set_env_var(&token_key, &value),
            _ => {}
        }
    }

    // setup lang dependencies for pulumi to work.
    pub fn init_pkgs(&self) {
        match &self.runtime {
            &Runtime::Typescript => {
                let arg_str = format!("cd {} && pwd && npm install", &self.stack_path);
                run_cmd_print_output(&arg_str);
            }
        }
    }

    pub fn init_env_vars(&self) {
        let passphrase_token_key = String::from("PULUMI_CONFIG_PASSPHRASE");
        set_env_var(&passphrase_token_key, &self.passphrase)
    }

    pub fn init_pulumi_stack(&self) {
        let arg_str = format!(
            "cd {} && echo 'Current working dir: \n' && pwd && {} stack init -s {}",
            &self.stack_path, PULUMI_BIN_PATH, &self.stack_name
        );
        run_cmd_print_output(&arg_str)
    }

    pub fn run_pulumi_preview(&self) {
        let arg_str = format!(
            "cd {} && pwd && {} preview -s {} --non-interactive",
            &self.stack_path, PULUMI_BIN_PATH, &self.stack_name
        );
        run_cmd_print_output(&arg_str);
    }

    pub fn run_pulumi_apply(&self) {
        let arg_str = format!(
            "cd {} && {} up -s {} --non-interactive --yes",
            &self.stack_path, PULUMI_BIN_PATH, &self.stack_name
        );
        run_cmd_print_output(&arg_str);
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Backend {
    Pulumicloud, // app.pulumi.com
    S3,          // aws's s3 bucket
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Runtime {
    Typescript,
}
