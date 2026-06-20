variable "aws_region" {
  description = "AWS region for the DDNS Lambda and supporting resources."
  type        = string
  default     = "us-west-2"
}

variable "name_prefix" {
  description = "Stable prefix for DDNS resources."
  type        = string
  default     = "wuji-agent-logic"
}

variable "allowed_hostnames" {
  description = "Permitted hostnames the Lambda may update."
  type        = list(string)
  default     = ["wuji.agent-logic.ai"]
}

variable "route53_hosted_zone_id" {
  description = "Public Route 53 hosted zone id for agent-logic.ai. Keep the actual value outside the repository."
  type        = string
}

variable "ddns_token_parameter_name" {
  description = "SSM SecureString parameter name containing the shared bearer token."
  type        = string
  default     = "/agent-logic/ddns/wuji/token"
}

variable "ddns_bearer_token" {
  description = "Optional bearer token value to write into SSM. Leave null to let Terraform generate one."
  type        = string
  sensitive   = true
  default     = null
}

variable "dns_record_ttl" {
  description = "TTL, in seconds, for the managed Route 53 A record."
  type        = number
  default     = 60
}

variable "lambda_timeout_seconds" {
  description = "Execution timeout for the DDNS Lambda."
  type        = number
  default     = 10
}

variable "lambda_memory_size_mb" {
  description = "Memory size for the DDNS Lambda."
  type        = number
  default     = 256
}

variable "tags" {
  description = "Common tags for DDNS resources."
  type        = map(string)
  default = {
    ManagedBy = "terraform"
    Service   = "wuji-ddns"
  }
}
