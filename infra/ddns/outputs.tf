output "lambda_function_name" {
  description = "Name of the DDNS Lambda."
  value       = aws_lambda_function.ddns_updater.function_name
}

output "lambda_function_url" {
  description = "HTTPS endpoint that Wuji calls every five minutes."
  value       = aws_lambda_function_url.ddns_updater.function_url
}

output "allowed_hostnames" {
  description = "Hostnames this DDNS slice is allowed to manage."
  value       = var.allowed_hostnames
}

output "token_parameter_name" {
  description = "SSM parameter the Lambda reads for bearer-token validation."
  value       = aws_ssm_parameter.ddns_token.name
}

output "ddns_bearer_token" {
  description = "Bearer token Terraform wrote to SSM. Treat this output as secret material."
  value       = local.ddns_bearer_token_value
  sensitive   = true
}

output "cloudwatch_log_group_name" {
  description = "CloudWatch log group for DDNS execution logs."
  value       = aws_cloudwatch_log_group.ddns_lambda.name
}

output "terraform_artifact_policy" {
  description = "Reminder that tfplan, .terraform, and build outputs are local-only."
  value       = "Keep infra/ddns/tfplan, infra/ddns/.terraform/, and infra/ddns/build/ untracked."
}
