# pulumi-actions
runs init, preview and apply on pulumi stacks right in your Github-Actions. Inspired from [Atlantis for Terraform](https://www.runatlantis.io/)

### PREVIEW Release
Currently, In this release; the following are supported only for `Pulumi Cloud` and `Typescript` runtime:
- Init the stack if it does not exists
- Preview the stack
- Apply the stack

### Future plans
- Add support for AWS Infra, though we can write and use AWS modules and all. But it won't be able to authenticate with AWS Account.
- Add support for AWS Backend (S3)
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
          preview the stack
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