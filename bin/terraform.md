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
