# Terraform owns the DDNS bearer token so the whole slice can be created from a
# single apply. The token is still secret material and therefore persists in
# Terraform state, so state custody must be treated as part of the security
# boundary.
resource "random_password" "ddns_token" {
  count            = var.ddns_bearer_token == null ? 1 : 0
  length           = 48
  special          = true
  override_special = "-_"
}

locals {
  ddns_bearer_token_value = var.ddns_bearer_token != null ? var.ddns_bearer_token : random_password.ddns_token[0].result
}

resource "aws_ssm_parameter" "ddns_token" {
  name      = var.ddns_token_parameter_name
  type      = "SecureString"
  value     = local.ddns_bearer_token_value
  overwrite = true
  tags      = var.tags
}
