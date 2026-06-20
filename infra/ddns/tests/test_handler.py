import json
import os
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "lambda"))

import handler  # noqa: E402


class FakeSsmClient:
    def __init__(self, token):
        self.token = token

    def get_parameter(self, Name, WithDecryption):
        return {"Parameter": {"Value": self.token}}


class FakeRoute53Client:
    def __init__(self, current_ip=None):
        self.current_ip = current_ip
        self.changes = []

    def list_resource_record_sets(self, HostedZoneId, StartRecordName, StartRecordType, MaxItems):
        if self.current_ip is None:
            return {"ResourceRecordSets": []}
        return {
            "ResourceRecordSets": [
                {
                    "Name": StartRecordName,
                    "Type": "A",
                    "ResourceRecords": [{"Value": self.current_ip}],
                }
            ]
        }

    def change_resource_record_sets(self, HostedZoneId, ChangeBatch):
        self.changes.append({"HostedZoneId": HostedZoneId, "ChangeBatch": ChangeBatch})


class HandlerTests(unittest.TestCase):
    def setUp(self):
        self.env = patch.dict(
            os.environ,
            {
                "DDNS_ALLOWED_HOSTNAMES": "wuji.agent-logic.ai",
                "DDNS_HOSTED_ZONE_ID": "ZTEST123",
                "DDNS_RECORD_TTL": "60",
                "DDNS_TOKEN_PARAMETER": "/agent-logic/ddns/wuji/token",
            },
            clear=False,
        )
        self.env.start()

    def tearDown(self):
        self.env.stop()

    def test_rejects_missing_token(self):
        response = handler.handle_event(
            self.make_event(headers={}),
            FakeRoute53Client(),
            FakeSsmClient("super-secret"),
        )
        self.assertEqual(response["statusCode"], 401)
        self.assertEqual(json.loads(response["body"])["error"], "unauthorized")

    def test_rejects_disallowed_hostname(self):
        response = handler.handle_event(
            self.make_event(hostname="nessus.agent-logic.ai"),
            FakeRoute53Client(),
            FakeSsmClient("super-secret"),
        )
        self.assertEqual(response["statusCode"], 400)
        self.assertEqual(json.loads(response["body"])["error"], "hostname_not_allowed")

    def test_no_change_is_idempotent(self):
        route53 = FakeRoute53Client(current_ip="203.0.113.10")
        response = handler.handle_event(
            self.make_event(source_ip="203.0.113.10"),
            route53,
            FakeSsmClient("super-secret"),
        )
        payload = json.loads(response["body"])
        self.assertEqual(response["statusCode"], 200)
        self.assertFalse(payload["changed"])
        self.assertEqual(route53.changes, [])

    def test_changed_ip_upserts_record(self):
        route53 = FakeRoute53Client(current_ip="203.0.113.10")
        response = handler.handle_event(
            self.make_event(source_ip="203.0.113.11"),
            route53,
            FakeSsmClient("super-secret"),
        )
        payload = json.loads(response["body"])
        self.assertEqual(response["statusCode"], 200)
        self.assertTrue(payload["changed"])
        self.assertEqual(len(route53.changes), 1)
        record = route53.changes[0]["ChangeBatch"]["Changes"][0]["ResourceRecordSet"]
        self.assertEqual(record["Name"], "wuji.agent-logic.ai")
        self.assertEqual(record["ResourceRecords"][0]["Value"], "203.0.113.11")

    def test_logs_stay_sanitized(self):
        route53 = FakeRoute53Client(current_ip="203.0.113.10")
        with self.assertLogs(handler.LOGGER, level="INFO") as captured:
            handler.handle_event(
                self.make_event(source_ip="203.0.113.10"),
                route53,
                FakeSsmClient("super-secret"),
            )
        emitted = "\n".join(captured.output)
        self.assertNotIn("super-secret", emitted)
        self.assertNotIn("Authorization", emitted)
        self.assertIn("wuji.agent-logic.ai", emitted)

    def test_ipv6_request_is_rejected_cleanly(self):
        response = handler.handle_event(
            self.make_event(source_ip="2001:db8::1"),
            FakeRoute53Client(),
            FakeSsmClient("super-secret"),
        )
        self.assertEqual(response["statusCode"], 400)
        self.assertEqual(json.loads(response["body"])["error"], "ipv6_not_supported_in_phase1")

    def test_invalid_base64_body_is_rejected_cleanly(self):
        event = self.make_event()
        event["isBase64Encoded"] = True
        event["body"] = "%%%not-base64%%%"
        response = handler.handle_event(
            event,
            FakeRoute53Client(),
            FakeSsmClient("super-secret"),
        )
        self.assertEqual(response["statusCode"], 400)
        self.assertEqual(json.loads(response["body"])["error"], "invalid_base64_body")

    @staticmethod
    def make_event(hostname="wuji.agent-logic.ai", source_ip="203.0.113.10", headers=None):
        if headers is None:
            headers = {"Authorization": "Bearer super-secret"}
        return {
            "headers": headers,
            "requestContext": {
                "http": {
                    "sourceIp": source_ip,
                }
            },
            "body": json.dumps({"hostname": hostname}),
        }


if __name__ == "__main__":
    unittest.main()
