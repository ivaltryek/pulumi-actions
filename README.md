# pulumi-actions
runs init, preview and apply on pulumi stacks right in your Github-Actions. Inspired from [Atlantis for Terraform](https://www.runatlantis.io/)

### PREVIEW Release
Currently, In this release; the following are supported only for `Pulumi Cloud` and `AWS S3` with `Typescript` runtime:
- Init the stack if it does not exists
- Preview the stack
- Apply the stack

### Future plans
Note: Strikethrough comments are already implemented.
- ~~Add support for AWS Infra, though we can write and use AWS modules and all. But it won't be able to authenticate with AWS Account.~~
- ~~Add support for AWS Backend (S3)~~
- Add other runtime support i.e python, go

## Usage
```
Usage: pulumi-actions [OPTIONS] --stack <STACK_NAME> --backend <BACKEND> --path <STACK_PATH> --runtime <RUNTIME> --passphrase <PASSPHRASE>

Options:
  -s, --stack <STACK_NAME>
          pulumi stack name
      --pulumi-cloud-token <PULUMI_CLOUD_TOKEN>
          Pulumi token - in case of using Pulumi cloud. (Optional)
      --s3-bucket <S3_BUCKET_NAME>
          S3 Bucket name - in case if backend is s3. exmaple: "s3://my-bucket"
  -b, --backend <BACKEND>
          backend type [possible values: pulumicloud, s3]
  -p, --path <STACK_PATH>
          Path to stack's index.ts
  -r, --runtime <RUNTIME>
          Path to stack's index.ts [possible values: typescript]
      --preview
          preview the stack
      --apply
          apply the stack
      --init
          init the stack if does not exist
      --passphrase <PASSPHRASE>
          stack passphrase
      --install-deps
          Install dependencies of language runtime, consider setting true only when you want to preview or apply the stack
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples

- For Github Actions usage
    ```yaml
    - name: preview stack
      uses: lowkey-who/pulumi-actions@main
       env:
        pulumi_cloud_token: ${{ secrets.PuluToken }}
        passphrase: ${{ secrets.Passphrase }}
       with:
        cmd: pulumi-actions --pulumi-cloud-token "$pulumi_cloud_token" -s test6 --backend pulumicloud --path "$GITHUB_WORKSPACE/examples/ts" --install-deps --preview --passphrase "$passphrase" --runtime typescript
    ```
- Deploy infrastructure on AWS Cloud
    ```yaml
    name: Comment Workflow

    on:
      issue_comment:
        types:
        - created

    jobs:
      comment_job:
        permissions:
          id-token: write
          contents: read
        name: pulumi-actions
        if: github.event.issue.pull_request != '' && contains(github.event.comment.body, 'pulumi-actions')   
        runs-on: ubuntu-latest

        steps:
        - name: Checkout code
          uses: actions/checkout@v3

        - name: Configure AWS Credentials
          uses: aws-actions/configure-aws-credentials@v2
          with:
              role-to-assume: <role-arn>
              aws-region: <your-aws-region>

        - name: Run Pulumi actions 
          uses: lowkey-who/pulumi-actions@main
          env:
            passphrase: ${{ secrets.Passphrase }}
          with:
            cmd: ${{ github.event.comment.body }}

    ```
  After setting up this workflow, make a PR on your repo and pass the somewhat similar command to work with s3 backend and AWS Cloud.

  ```
  pulumi-actions -s ci-test --init --backend s3 --s3-bucket "s3://my-bucket-name" --runtime typescript --path "$GITHUB_WORKSPACE/examples/" --passphrase "$passphrase"
  ```
  
  To work with `aws-actions/configure-aws-credentials@v2` github action, you'll have to configure OIDC setup with AWS. Here is the [doc](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/configuring-openid-connect-in-amazon-web-services) for that.

- Make this action run on PR when comment is issued.
  For this use-case check this workflow [example](https://github.com/lowkey-who/pulumi-actions/blob/main/.github/workflows/test-issue-comment.yaml)

  TL;DR Just comment the command that you need to run and action will run whatever fed into the comment. So make sure to check the comment else workflow might not run.

### NOTE: Please test the following examples either in Github Actions or mount the dir when running the docker command.
- Init the stack
    ```
    pulumi-actions -s test6 --backend pulumicloud --path "/examples/ts" --runtime typescript --pulumi-cloud-token "pul-xxxxxxxxxxxxx" --init --passphrase "abceDDddsfdsfsdfdsadasd"
    ```

- Preview the stack
    ```
    pulumi-actions -s test6 --backend pulumicloud --path "/examples/ts" --runtime typescript --pulumi-cloud-token "pul-xxxxxxxxxxxxx" --preview --install-deps --passphrase "abceDDddsfdsfsdfdsadasd"
    ```

- Apply the stack
    ```
    pulumi-actions -s test6 --backend pulumicloud --path "/examples/ts" --runtime typescript --pulumi-cloud-token "pul-xxxxxxxxxxxxx" --apply --install-deps --passphrase "abceDDddsfdsfsdfdsadasd"
    ```

> workflow [examples](https://github.com/lowkey-who/pulumi-actions/tree/main/.github/workflows)