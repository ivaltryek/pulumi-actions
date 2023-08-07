use clap::Parser;
use pulumi::cmd::Args;

mod pulumi;
fn main() {
    // grab user inputs
    let inputs: Args = Args::parse();

    // init stack with values
    let args = Args::new(
        inputs.stack_name,
        inputs.pulumi_cloud_token,
        inputs.s3_bucket_name,
        inputs.backend,
        inputs.stack_path,
        inputs.runtime,
        inputs.preview,
        inputs.apply,
        inputs.init,
        inputs.passphrase,
        inputs.install_deps,
    );

    // load environment vars
    args.init_env_vars();

    // Check if arguments are correct
    args.notify_user_if_empty();

    // if --install-deps arg is set; intall the dependencies.
    if args.install_deps {
        args.init_pkgs();
    }
    // If --init is set; run pulumi init stack
    if args.init {
        args.init_pulumi_stack();
    }
    // If --preview is set; run `pulumi preview`
    if args.preview {
        println!("Running the preview of the stack");
        args.run_pulumi_preview();
    }

    // if --apply is set; run `pulumi up`
    if args.apply {
        println!("Applying stack");
        args.run_pulumi_apply();
    }
}
