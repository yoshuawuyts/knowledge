# terraform

```txt
$ terraform plan            # preview changes
$ terraform apply           # run changes
$ terraform show            # show current state
$ terraform plan -destroy   # preview infra deletion
$ terraform destroy         # delete infra
```

## variables
Variables are stored in a `variable.tf` file.
```hcl
variable "access_key" {}
variable "secret_key" {}
variable "region" {
    default = "us-east-1"
}
```
```hcl
provider "aws" {
    access_key = "${var.access_key}"
    secret_key = "${var.secret_key}"
    region = "${var.region}"
}
```

Variables can be sent from the command line:
```sh
$ terraform plan \
  -var 'access_key=foo' \
  -var 'secret_key=bar'
```

- https://www.terraform.io/intro/getting-started/variables.html

## AWS
### attach a keypair
```hcl
resource "aws_key_pair" "deployer" {
  key_name = "deployer-key"
  public_key = "ssh-rsa <key> email@example.com"
}
```

## Managing state files
`.tfstate` files are a bit finnicky. They contain plain-text secrets and that's
bad. Real bad. So you probably want to store them on something like `S3` so
they're locked down and nobody can touch them. Security is good. Also wanna
store them there because teammates might check out stuff and then everything
goes south so hey yeah don't do that.

You should have different files per environment. You should definitely be
separating your environments, unless you're like me and are broke and don't
have money to toss at the problem. But everyone else should. Probably. Maybe.
```txt
staging/       # files for the staging environment
  main.tf
  outputs.tf
  vars.tf
production/    # files for the production environment
  main.tf
  outputs.tf
  vars.tf
global/        # shared files like IAM rules, SNS topics, S3 buckets
  main.tf
  outputs.tf
  vars.tf
```

- http://stackoverflow.com/questions/33157516/best-practices-when-using-terraform/38749508#38749508
- http://stackoverflow.com/questions/38486335/should-i-commit-tfstate-files-to-git
