#!/usr/bin/env python3
"""Focused proof for the typed GitHub issue-read backfill route."""

from __future__ import annotations

import importlib.util
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parents[2]
MODULE_PATH = ROOT / "adl/tools/build_v0916_workflow_metric_backfill_inventory.py"
SPEC = importlib.util.spec_from_file_location("workflow_metric_backfill", MODULE_PATH)
assert SPEC and SPEC.loader
BACKFILL = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = BACKFILL
SPEC.loader.exec_module(BACKFILL)


class TypedIssueReadBackfillTest(unittest.TestCase):
    def test_collect_rows_uses_typed_issue_read_and_preserves_cycle_time(self) -> None:
        run_root = ROOT / ".adl" / "runs"
        run_root.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="metric-backfill-test-", dir=run_root) as raw:
            fixture = Path(raw)
            task = fixture / ".adl/v0.91.6/tasks/issue-4441__typed-route"
            task.mkdir(parents=True)
            (task / "sor.md").write_text(
                """Execution:\n- Start Time: 2026-01-01T00:00:00Z\n- End Time: 2026-01-01T00:00:12Z\n\n- Actual elapsed seconds: `12`\n- Actual total tokens: `34`\n""",
                encoding="utf-8",
            )
            bodies = fixture / ".adl/v0.91.6/bodies"
            bodies.mkdir(parents=True)
            (bodies / "issue-4441-typed-route.md").write_text(
                '# Typed route\n', encoding="utf-8"
            )
            request_log = fixture / "request.json"
            fake = fixture / "csdlc-github-issue"
            fake.write_text(
                """#!/usr/bin/env python3
import json, os, pathlib, sys
request = json.loads(pathlib.Path(sys.argv[sys.argv.index('--request') + 1]).read_text())
pathlib.Path(os.environ['REQUEST_LOG']).write_text(json.dumps(request))
print(json.dumps({
  'schema': 'csdlc.github_action_result.v1',
  'repository': request['repository'],
  'action': 'issue_read',
  'operation_key': None,
  'issue': {
    'schema': 'csdlc.github_issue.v1',
    'repository': request['repository'],
    'number': request['issue'],
    'title': 'Typed route',
    'body': '',
    'state': 'closed',
    'created_at': '2026-01-01T00:00:00Z',
    'closed_at': '2026-01-01T01:00:00Z',
    'labels': [], 'assignees': [], 'milestone': None,
    'marker_present': False
  },
  'comment_id': None, 'pr_state': None, 'reconciled': True
}))
""",
                encoding="utf-8",
            )
            fake.chmod(0o755)

            with mock.patch.dict(
                os.environ,
                {
                    "ADL_GITHUB_REPO": "owner/repo",
                    "ADL_CSDLC_GITHUB_ISSUE_CMD": str(fake),
                    "REQUEST_LOG": str(request_log),
                },
                clear=False,
            ):
                rows = BACKFILL.collect_rows(ROOT, fixture, None)

            self.assertEqual(len(rows), 1)
            self.assertEqual(rows[0]["github_cycle_time_seconds"], "3600")
            self.assertEqual(
                rows[0]["github_cycle_time_source"],
                "csdlc_github_issue_created_closed",
            )
            request = json.loads(request_log.read_text(encoding="utf-8"))
            self.assertEqual(
                request,
                {"repository": "owner/repo", "action": "issue_read", "issue": 4441},
            )
            common_git_dir = Path(
                BACKFILL.run(
                    ["git", "rev-parse", "--git-common-dir"], cwd=ROOT
                ).strip()
            )
            pending = list(
                (common_git_dir / "csdlc-v2/requests").glob(
                    "workflow-metric-4441-*.json"
                )
            )
            self.assertEqual(pending, [])

            markdown = fixture / "inventory.md"
            BACKFILL.write_markdown(
                markdown,
                rows,
                BACKFILL.summarize(rows),
                ROOT,
                fixture,
            )
            self.assertIn(
                "typed `csdlc-github-issue run --request <issue-read-request.json>`",
                markdown.read_text(encoding="utf-8"),
            )


if __name__ == "__main__":
    unittest.main()
