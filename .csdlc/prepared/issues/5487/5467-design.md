# #5467 Design

Repair the local CI contract so it reaches every backend-snapshot assertion and behaviorally proves hosted, Spot-selected, and invalid backend routing without invoking AWS.

The contract covers the SSM `GetParameter`, EBS `AttachVolume`, and IAM `DeleteRolePolicy` policy-owner paths. Local fixtures interpret workflow selection and prove each route deterministically; they do not call cloud commands, credentials, or remote validation workflows.

Acceptance requires the contract to fail if any snapshot assertion is removed or bypassed, to require exact invalid-backend status `2`, and to prove the three backend inputs have deterministic local outcomes.
