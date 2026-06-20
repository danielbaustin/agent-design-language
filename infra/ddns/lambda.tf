data "archive_file" "ddns_lambda" {
  type        = "zip"
  source_file = "${path.module}/lambda/handler.py"
  output_path = local.lambda_package_path
}

resource "aws_cloudwatch_log_group" "ddns_lambda" {
  name              = local.log_group_name
  retention_in_days = 14
  tags              = var.tags
}

resource "aws_lambda_function" "ddns_updater" {
  function_name    = local.lambda_function_name
  role             = aws_iam_role.ddns_lambda.arn
  runtime          = "python3.12"
  handler          = "handler.lambda_handler"
  filename         = data.archive_file.ddns_lambda.output_path
  source_code_hash = data.archive_file.ddns_lambda.output_base64sha256
  timeout          = var.lambda_timeout_seconds
  memory_size      = var.lambda_memory_size_mb

  environment {
    variables = {
      DDNS_ALLOWED_HOSTNAMES = local.allowed_hostnames_csv
      DDNS_HOSTED_ZONE_ID    = var.route53_hosted_zone_id
      DDNS_RECORD_TTL        = tostring(var.dns_record_ttl)
      DDNS_TOKEN_PARAMETER   = aws_ssm_parameter.ddns_token.name
    }
  }

  depends_on = [
    aws_cloudwatch_log_group.ddns_lambda,
  ]

  tags = var.tags
}

resource "aws_lambda_function_url" "ddns_updater" {
  function_name      = aws_lambda_function.ddns_updater.function_name
  authorization_type = "NONE"

  cors {
    allow_methods = ["POST"]
    allow_origins = ["*"]
  }
}

resource "aws_lambda_permission" "ddns_function_url_public_invoke" {
  statement_id           = "AllowPublicFunctionUrlInvoke"
  action                 = "lambda:InvokeFunctionUrl"
  function_name          = aws_lambda_function.ddns_updater.function_name
  principal              = "*"
  function_url_auth_type = "NONE"
}

resource "terraform_data" "ddns_function_public_invoke_via_url" {
  input = {
    function_name = aws_lambda_function.ddns_updater.function_name
    region        = var.aws_region
    statement_id  = "AllowPublicInvokeViaFunctionUrl"
  }

  # Provider compatibility bridge: this apply path expects local aws CLI and jq
  # until the provider can express invoked_via_function_url directly.
  provisioner "local-exec" {
    command = <<-EOT
      set -eu
      if aws --region ${self.input.region} lambda get-policy --function-name ${self.input.function_name} >/tmp/wuji_ddns_lambda_policy.json 2>/dev/null && \
         jq -e '.Policy | fromjson | .Statement[] | select(.Sid == "'"${self.input.statement_id}"'")' /tmp/wuji_ddns_lambda_policy.json >/dev/null 2>&1; then
        exit 0
      fi
      aws --region ${self.input.region} lambda add-permission \
        --function-name ${self.input.function_name} \
        --statement-id ${self.input.statement_id} \
        --action lambda:InvokeFunction \
        --principal '*' \
        --invoked-via-function-url
    EOT
  }

  provisioner "local-exec" {
    when    = destroy
    command = <<-EOT
      aws --region ${self.input.region} lambda remove-permission \
        --function-name ${self.input.function_name} \
        --statement-id ${self.input.statement_id} >/dev/null 2>&1 || true
    EOT
  }

  depends_on = [
    aws_lambda_function.ddns_updater,
    aws_lambda_function_url.ddns_updater,
  ]
}
