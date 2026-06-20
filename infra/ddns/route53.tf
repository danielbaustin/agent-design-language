data "aws_route53_zone" "target" {
  zone_id      = var.route53_hosted_zone_id
  private_zone = false
}
