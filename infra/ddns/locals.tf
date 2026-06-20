locals {
  name_prefix           = var.name_prefix
  lambda_function_name  = "${local.name_prefix}-ddns-updater"
  lambda_package_path   = "${path.module}/build/${local.lambda_function_name}.zip"
  log_group_name        = "/aws/lambda/${local.lambda_function_name}"
  allowed_hostnames_csv = join(",", var.allowed_hostnames)
}
