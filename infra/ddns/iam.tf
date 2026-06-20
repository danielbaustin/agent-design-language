data "aws_caller_identity" "current" {}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "ddns_lambda" {
  name               = "${local.name_prefix}-ddns-lambda"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
  tags               = var.tags
}

data "aws_iam_policy_document" "ddns_lambda" {
  statement {
    sid = "ReadTokenParameter"

    actions = [
      "ssm:GetParameter",
    ]

    resources = [
      "arn:aws:ssm:${var.aws_region}:${data.aws_caller_identity.current.account_id}:parameter${var.ddns_token_parameter_name}",
    ]
  }

  statement {
    sid = "ManageSingleHostedZoneRecords"

    actions = [
      "route53:ChangeResourceRecordSets",
      "route53:ListResourceRecordSets",
    ]

    resources = [
      "arn:aws:route53:::hostedzone/${var.route53_hosted_zone_id}",
    ]
  }

  statement {
    sid = "WriteBoundedLambdaLogs"

    actions = [
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]

    resources = [
      "${aws_cloudwatch_log_group.ddns_lambda.arn}:*",
    ]
  }
}

resource "aws_iam_role_policy" "ddns_lambda" {
  name   = "${local.name_prefix}-ddns-lambda"
  role   = aws_iam_role.ddns_lambda.id
  policy = data.aws_iam_policy_document.ddns_lambda.json
}
