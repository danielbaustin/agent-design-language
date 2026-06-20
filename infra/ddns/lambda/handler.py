import base64
import binascii
import hmac
import ipaddress
import json
import logging
import os
from typing import Dict, Optional


LOGGER = logging.getLogger(__name__)
LOGGER.setLevel(logging.INFO)


def lambda_handler(event, context):
    import boto3

    return handle_event(event, boto3.client("route53"), boto3.client("ssm"))


def handle_event(event, route53_client, ssm_client):
    config = load_config()
    auth_header = get_header(event, "authorization")
    expected_token = fetch_token(ssm_client, config["token_parameter_name"])
    if not token_matches(auth_header, expected_token):
        log_event("rejected", hostname=None, old_ip=None, new_ip=None, detail="unauthorized")
        return response(401, {"ok": False, "error": "unauthorized"})

    try:
        hostname = extract_hostname(event)
    except ValueError as exc:
        log_event("rejected", hostname=None, old_ip=None, new_ip=None, detail=str(exc))
        return response(400, {"ok": False, "error": str(exc)})
    if hostname not in config["allowed_hostnames"]:
        log_event("rejected", hostname=hostname, old_ip=None, new_ip=None, detail="hostname_not_allowed")
        return response(400, {"ok": False, "error": "hostname_not_allowed"})

    try:
        source_ip = extract_source_ip(event)
    except ValueError as exc:
        log_event("rejected", hostname=hostname, old_ip=None, new_ip=None, detail=str(exc))
        return response(400, {"ok": False, "error": str(exc)})

    current_ip = get_current_a_record(route53_client, config["hosted_zone_id"], hostname)
    if current_ip == source_ip:
        log_event("no_change", hostname=hostname, old_ip=current_ip, new_ip=source_ip, detail=None)
        return response(
            200,
            {
                "ok": True,
                "hostname": hostname,
                "current_ip": current_ip,
                "changed": False,
            },
        )

    upsert_a_record(
        route53_client,
        hosted_zone_id=config["hosted_zone_id"],
        hostname=hostname,
        ip_address=source_ip,
        ttl=config["record_ttl"],
    )
    log_event("updated", hostname=hostname, old_ip=current_ip, new_ip=source_ip, detail=None)
    return response(
        200,
        {
            "ok": True,
            "hostname": hostname,
            "previous_ip": current_ip,
            "current_ip": source_ip,
            "changed": True,
        },
    )


def load_config() -> Dict[str, object]:
    allowed_hostnames = tuple(
        hostname.strip()
        for hostname in os.environ["DDNS_ALLOWED_HOSTNAMES"].split(",")
        if hostname.strip()
    )
    return {
        "allowed_hostnames": allowed_hostnames,
        "hosted_zone_id": os.environ["DDNS_HOSTED_ZONE_ID"],
        "record_ttl": int(os.environ.get("DDNS_RECORD_TTL", "60")),
        "token_parameter_name": os.environ["DDNS_TOKEN_PARAMETER"],
    }


def fetch_token(ssm_client, parameter_name: str) -> str:
    parameter = ssm_client.get_parameter(Name=parameter_name, WithDecryption=True)
    return parameter["Parameter"]["Value"]


def get_header(event, expected_name: str) -> Optional[str]:
    headers = event.get("headers") or {}
    for key, value in headers.items():
        if key.lower() == expected_name:
            return value
    return None


def token_matches(auth_header: Optional[str], expected_token: str) -> bool:
    if not auth_header:
        return False
    prefix = "Bearer "
    if not auth_header.startswith(prefix):
        return False
    return hmac.compare_digest(auth_header[len(prefix) :], expected_token)


def extract_hostname(event) -> Optional[str]:
    body = event.get("body")
    if not body:
        raise ValueError("missing_body")
    if event.get("isBase64Encoded"):
        try:
            body = base64.b64decode(body).decode("utf-8")
        except (binascii.Error, UnicodeDecodeError) as exc:
            raise ValueError("invalid_base64_body") from exc
    try:
        payload = json.loads(body)
    except json.JSONDecodeError as exc:
        raise ValueError("invalid_json_body") from exc
    hostname = payload.get("hostname")
    if not hostname:
        raise ValueError("missing_hostname")
    return hostname


def extract_source_ip(event) -> Optional[str]:
    ip_candidate = (
        event.get("requestContext", {})
        .get("http", {})
        .get("sourceIp")
    )
    if ip_candidate is None:
        raise ValueError("missing_source_ip")
    parsed = ipaddress.ip_address(ip_candidate)
    if parsed.version != 4:
        raise ValueError("ipv6_not_supported_in_phase1")
    return str(parsed)


def get_current_a_record(route53_client, hosted_zone_id: str, hostname: str) -> Optional[str]:
    response_payload = route53_client.list_resource_record_sets(
        HostedZoneId=hosted_zone_id,
        StartRecordName=hostname,
        StartRecordType="A",
        MaxItems="1",
    )
    for record_set in response_payload.get("ResourceRecordSets", []):
        if normalize_record_name(record_set.get("Name")) != hostname:
            continue
        if record_set.get("Type") != "A":
            continue
        records = record_set.get("ResourceRecords", [])
        if not records:
            return None
        return records[0]["Value"]
    return None


def normalize_record_name(record_name: Optional[str]) -> Optional[str]:
    if record_name is None:
        return None
    return record_name.rstrip(".")


def upsert_a_record(route53_client, hosted_zone_id: str, hostname: str, ip_address: str, ttl: int) -> None:
    route53_client.change_resource_record_sets(
        HostedZoneId=hosted_zone_id,
        ChangeBatch={
            "Comment": "ADL Wuji DDNS update",
            "Changes": [
                {
                    "Action": "UPSERT",
                    "ResourceRecordSet": {
                        "Name": hostname,
                        "Type": "A",
                        "TTL": ttl,
                        "ResourceRecords": [{"Value": ip_address}],
                    },
                }
            ],
        },
    )


def log_event(result: str, hostname: Optional[str], old_ip: Optional[str], new_ip: Optional[str], detail: Optional[str]) -> None:
    payload = {
        "result": result,
        "hostname": hostname,
        "old_ip": old_ip,
        "new_ip": new_ip,
    }
    if detail:
        payload["detail"] = detail
    LOGGER.info(json.dumps(payload, sort_keys=True))


def response(status_code: int, payload: Dict[str, object]) -> Dict[str, object]:
    return {
        "statusCode": status_code,
        "headers": {
            "content-type": "application/json",
        },
        "body": json.dumps(payload, sort_keys=True),
    }
