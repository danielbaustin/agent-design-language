# V0.91.6 Workflow Metric Backfill

Issue: `#4441`

## Summary

This artifact is the bounded historical workflow-metrics backfill for `v0.91.6` only.
It stays separate from `#4431`, which owns forward authoritative capture.

## Survey Definition

- Surveyed set: issue-local task bundles present under the primary checkout local corpus `/.adl/v0.91.6/tasks/issue-*`.
- Exclusions: sprint umbrellas, sprint review packets, and non-issue artifacts outside the issue-task corpus.
- Issue metadata source: repo-native `adl/tools/pr.sh issue view <issue> --json`.
- Issue-local metrics source: `sor.md` Issue Metrics Truth section, with fallback to SOR execution start/end timestamps for elapsed-time derivation when explicit elapsed seconds are absent.
- Missing-data rule: values remain `unknown` or `not_collected`; they are never inferred from diff size, elapsed chat time, or subjective effort.

## Refresh Command

```bash
python3 adl/tools/build_v0916_workflow_metric_backfill_inventory.py
```

## Aggregate Counts

- Surveyed issues: `303`
- Closed issues: `288`
- Open issues: `15`
- Actual session elapsed explicit: `24`
- Actual session elapsed derived from SOR execution window: `255`
- Actual session elapsed unknown: `24`
- GitHub cycle time reconstructed from created/closed timestamps: `288`
- Actual total tokens explicit: `12`
- Actual total tokens unknown: `291`
- Actual total tokens not_collected: `0`
- Row-contract complete rows: `303`
- Row-contract partial rows: `0`
- Row-contract incomplete rows: `0`
- Full metrics known rows: `12`
- Timing recovered but token gap rows: `252`
- Cycle-only recovered rows: `24`
- Open-issue local-timing-only rows: `15`
- Open-issue sparse-metrics rows: `0`

## Output Files

- CSV inventory: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_INVENTORY_4441.csv`
- JSON issue-group summary: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json`
- Review note: `docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.md`

## Row-Contract Groups

- Row-contract complete rows: `#3902, #3922, #3925, #3927, #3934, #3935, #3945, #3946, #3963, #3965, #3966, #3967, #3968, #3969, #3970, #3971, #3972, #3973, #3974, #3975, #3976, #3977, #3978, #3979, #3980, #3981, #3982, #3983, #3984, #3985, #3986, #3987, #3988, #3989, #3990, #3991, #3992, #3993, #3994, #3995, #3996, #3997, #3998, #3999, #4000, #4001, #4002, #4003, #4004, #4005, #4006, #4007, #4008, #4009, #4010, #4011, #4012, #4013, #4014, #4015, #4016, #4017, #4018, #4019, #4020, #4021, #4022, #4023, #4024, #4025, #4026, #4027, #4028, #4029, #4030, #4031, #4032, #4033, #4034, #4035, #4036, #4037, #4038, #4039, #4040, #4041, #4044, #4047, #4048, #4049, #4051, #4053, #4055, #4064, #4066, #4069, #4074, #4076, #4077, #4078, #4083, #4084, #4085, #4086, #4087, #4088, #4089, #4094, #4095, #4096, #4097, #4105, #4106, #4107, #4109, #4111, #4113, #4116, #4121, #4126, #4129, #4136, #4141, #4142, #4143, #4144, #4145, #4146, #4149, #4154, #4155, #4156, #4157, #4158, #4160, #4162, #4163, #4164, #4165, #4166, #4167, #4177, #4178, #4179, #4180, #4181, #4182, #4183, #4185, #4190, #4196, #4199, #4212, #4213, #4214, #4215, #4216, #4217, #4218, #4219, #4220, #4223, #4225, #4229, #4231, #4234, #4235, #4236, #4237, #4241, #4242, #4243, #4244, #4245, #4246, #4247, #4248, #4250, #4251, #4252, #4253, #4254, #4255, #4256, #4257, #4262, #4264, #4276, #4277, #4278, #4279, #4280, #4281, #4284, #4286, #4292, #4294, #4295, #4296, #4298, #4299, #4300, #4303, #4305, #4306, #4308, #4309, #4310, #4311, #4312, #4313, #4314, #4315, #4316, #4317, #4318, #4319, #4320, #4321, #4322, #4324, #4325, #4329, #4330, #4331, #4332, #4341, #4343, #4356, #4357, #4368, #4369, #4370, #4371, #4372, #4373, #4374, #4375, #4376, #4378, #4383, #4388, #4389, #4390, #4391, #4392, #4393, #4394, #4395, #4396, #4397, #4398, #4406, #4412, #4413, #4416, #4417, #4418, #4419, #4420, #4421, #4425, #4429, #4431, #4432, #4433, #4434, #4435, #4436, #4437, #4438, #4441, #4442, #4443, #4444, #4448, #4449, #4450, #4453, #4454, #4457, #4459, #4470, #4474, #4475, #4476, #4479, #4481, #4483, #4484, #4485, #4486, #4487, #4488, #4489, #4493, #4499, #4502, #4503, #4504, #4505, #4507, #4509`
- Row-contract partial rows: `none`
- Row-contract incomplete rows: `none`

## Metric Availability Groups

- Full metrics known rows: `#4378, #4397, #4398, #4417, #4431, #4432, #4448, #4453, #4459, #4481, #4507, #4509`
- Timing recovered but token gap rows: `#3922, #3925, #3927, #3934, #3945, #3946, #3963, #3965, #3966, #3967, #3968, #3969, #3970, #3971, #3972, #3973, #3974, #3975, #3979, #3985, #3986, #3987, #3988, #3989, #3990, #3991, #3992, #3993, #3994, #3995, #3996, #3997, #3998, #3999, #4000, #4001, #4002, #4003, #4004, #4005, #4006, #4007, #4008, #4009, #4010, #4011, #4012, #4013, #4014, #4015, #4016, #4017, #4018, #4019, #4020, #4021, #4022, #4023, #4024, #4025, #4026, #4027, #4028, #4029, #4030, #4031, #4032, #4033, #4034, #4035, #4036, #4037, #4038, #4039, #4040, #4041, #4044, #4047, #4048, #4049, #4053, #4055, #4064, #4066, #4069, #4074, #4076, #4077, #4078, #4083, #4084, #4085, #4086, #4087, #4088, #4089, #4094, #4095, #4096, #4097, #4105, #4106, #4107, #4109, #4111, #4113, #4116, #4121, #4126, #4129, #4136, #4141, #4149, #4155, #4156, #4157, #4158, #4160, #4162, #4163, #4164, #4165, #4166, #4167, #4177, #4178, #4179, #4180, #4181, #4182, #4183, #4185, #4190, #4199, #4213, #4214, #4215, #4216, #4217, #4218, #4219, #4220, #4223, #4225, #4229, #4231, #4234, #4235, #4236, #4237, #4241, #4242, #4243, #4244, #4245, #4246, #4247, #4248, #4250, #4251, #4253, #4255, #4257, #4262, #4264, #4276, #4277, #4278, #4279, #4280, #4281, #4284, #4286, #4292, #4294, #4295, #4296, #4298, #4299, #4300, #4303, #4305, #4306, #4308, #4309, #4310, #4311, #4312, #4313, #4314, #4315, #4316, #4317, #4318, #4319, #4322, #4324, #4325, #4329, #4330, #4331, #4332, #4341, #4343, #4356, #4357, #4368, #4383, #4388, #4389, #4390, #4391, #4392, #4393, #4394, #4395, #4396, #4406, #4412, #4413, #4416, #4418, #4419, #4420, #4421, #4425, #4429, #4442, #4443, #4444, #4449, #4450, #4454, #4457, #4470, #4474, #4475, #4476, #4479, #4483, #4484, #4485, #4486, #4487, #4488, #4489, #4493, #4499, #4502, #4503, #4504, #4505`
- Cycle-only recovered rows: `#3902, #3935, #4051, #4142, #4143, #4144, #4145, #4146, #4154, #4196, #4212, #4252, #4254, #4256, #4320, #4321, #4369, #4370, #4371, #4372, #4373, #4374, #4375, #4376`
- Open-issue local-timing-only rows: `#3976, #3977, #3978, #3980, #3981, #3982, #3983, #3984, #4433, #4434, #4435, #4436, #4437, #4438, #4441`

## Spot-Check Rows

| Issue | Row Contract | Metrics Known | Actual Session Elapsed | GitHub Cycle Time | Actual Total Tokens | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| #4393 | complete | timing_recovered_token_gap | 1461 (derived) | 72250 (derived) | unknown (unknown) | low |
| #4431 | complete | full_metrics_known | 1203 (explicit) | 3193 (derived) | 652773 (explicit) | high |
| #4479 | complete | timing_recovered_token_gap | 1920 (explicit) | 16130 (derived) | unknown (unknown) | low |

## Notes

- `actual_session_elapsed_seconds` records issue-local execution time when explicit SOR metrics exist, otherwise a derived execution-window value when SOR start/end timestamps exist.
- `github_cycle_time_seconds` records reconstructed GitHub issue calendar duration only when repo-native `createdAt` and `closedAt` are both available.
- `actual_total_tokens` stays explicit only when issue-local evidence recorded it truthfully.

_Generated from the bound issue worktree using the primary checkout local-state corpus as the survey root._
