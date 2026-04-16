#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def profile_dir() -> Path:
    return Path(__file__).resolve().parent / "profiles"


def load_profile(name: str) -> dict:
    path = profile_dir() / f"{name}.json"
    if not path.is_file():
        raise SystemExit(f"unknown profile: {name}")
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict | list) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def cmd_list_devices(args: argparse.Namespace) -> int:
    devices = {
        "schema_version": "adl.midi_device_inventory.v1",
        "devices": [
            {
                "device_id": "mvave-chocolate-fixture",
                "name": "MVAVE Chocolate Fixture",
                "transport": "usb-midi-fixture",
                "supports_input": True,
                "supports_output": False
            }
        ]
    }
    write_json(Path(args.out), devices)
    return 0


def cmd_bind_profile(args: argparse.Namespace) -> int:
    profile = load_profile(args.profile)
    binding = {
        "schema_version": "adl.midi_profile_binding.v1",
        "device_id": args.device_id,
        "profile_id": profile["profile_id"],
        "transport": profile["transport"],
        "mapping": profile["mapping"]
    }
    write_json(Path(args.out), binding)
    return 0


def cmd_listen(args: argparse.Namespace) -> int:
    binding = json.loads(Path(args.binding).read_text(encoding="utf-8"))
    fixture = json.loads(Path(args.fixture).read_text(encoding="utf-8"))
    mapping = binding["mapping"]
    normalized = []
    for event in fixture["events"]:
        note = str(event["note"])
        action = mapping.get(note, "unmapped")
        normalized.append(
            {
                "index": event["index"],
                "device_id": binding["device_id"],
                "profile_id": binding["profile_id"],
                "raw_note": event["note"],
                "velocity": event["velocity"],
                "action": action,
                "section_hint": event["section_hint"]
            }
        )
    payload = {
        "schema_version": "adl.midi_event_log.v1",
        "binding": {
            "device_id": binding["device_id"],
            "profile_id": binding["profile_id"]
        },
        "events": normalized
    }
    write_json(Path(args.event_log), payload)
    return 0


def cmd_send(args: argparse.Namespace) -> int:
    payload = {
        "schema_version": "adl.midi_send_ack.v1",
        "device_id": args.device_id,
        "note": args.note,
        "velocity": args.velocity,
        "status": "recorded"
    }
    write_json(Path(args.out), payload)
    return 0


def cmd_get_event_log(args: argparse.Namespace) -> int:
    payload = json.loads(Path(args.log).read_text(encoding="utf-8"))
    summary = {
        "schema_version": "adl.midi_event_summary.v1",
        "event_count": len(payload["events"]),
        "actions": [event["action"] for event in payload["events"]]
    }
    write_json(Path(args.out), summary)
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_devices = subparsers.add_parser("list-devices")
    list_devices.add_argument("--out", required=True)
    list_devices.set_defaults(func=cmd_list_devices)

    bind_profile = subparsers.add_parser("bind-profile")
    bind_profile.add_argument("--profile", required=True)
    bind_profile.add_argument("--device-id", required=True)
    bind_profile.add_argument("--out", required=True)
    bind_profile.set_defaults(func=cmd_bind_profile)

    listen = subparsers.add_parser("listen")
    listen.add_argument("--binding", required=True)
    listen.add_argument("--fixture", required=True)
    listen.add_argument("--event-log", required=True)
    listen.set_defaults(func=cmd_listen)

    send = subparsers.add_parser("send")
    send.add_argument("--device-id", required=True)
    send.add_argument("--note", required=True, type=int)
    send.add_argument("--velocity", required=True, type=int)
    send.add_argument("--out", required=True)
    send.set_defaults(func=cmd_send)

    get_event_log = subparsers.add_parser("get-event-log")
    get_event_log.add_argument("--log", required=True)
    get_event_log.add_argument("--out", required=True)
    get_event_log.set_defaults(func=cmd_get_event_log)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
