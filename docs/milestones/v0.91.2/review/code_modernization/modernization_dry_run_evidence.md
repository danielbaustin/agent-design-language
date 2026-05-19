# Moderne / OpenRewrite Dry-Run Evidence

## Status

Bounded `WP-10` dry-run evidence packet.

This packet is proving with a real OpenRewrite Maven dry-run against a small
tracked Java fixture inside the review packet.

## Why A Fixture Was Used

This repository does not contain a natural Java/Maven modernization target.

Repo-scope checks performed for `WP-10`:

- searched for `*.java`
- searched for `pom.xml`
- searched for `build.gradle`
- searched for `build.gradle.kts`

Observed result:

- no existing production Java/Maven modernization target is present in the main
  ADL repository

To avoid fake "repo modernization" claims while still showing something real,
the proof uses one minimal tracked fixture at:

- `docs/milestones/v0.91.2/review/code_modernization/fixture/use_diamond_operator_demo/`

## What Is Proved

This packet proves both:

- the governed dry-run posture ADL would require before broader modernization
  mutation is accepted
- a real OpenRewrite Maven dry-run command against a bounded Java fixture

- the objective must be bounded first
- the recipe family must be justified
- the command posture must be explicit
- dry-run is the default proving mode
- review and reversibility remain first-class gates

## Executed Dry-Run Command Family

Executed OpenRewrite Maven dry-run:

```bash
mvn -Dmaven.repo.local=.m2-local -U org.openrewrite.maven:rewrite-maven-plugin:6.39.0:dryRun \
  -Drewrite.recipeArtifactCoordinates=org.openrewrite.recipe:rewrite-static-analysis:2.34.1 \
  -Drewrite.activeRecipes=org.openrewrite.staticanalysis.UseDiamondOperator \
  -Drewrite.exportDatatables=true
```

The fixture run was executed from:

- `docs/milestones/v0.91.2/review/code_modernization/fixture/use_diamond_operator_demo/`

## Why This Command Family

It preserves the intended `mind / hands` split:

- ADL selects and governs the recipe path
- the deterministic rewrite plugin performs the transformation work
- the resulting output remains reviewable as command output plus diff evidence

## Selected Demo Posture

For `WP-10`, the truthful posture is:

- `execution_mode`: dry-run executed
- `target_type`: tracked Java fixture
- `production_repo_execution`: not_run
- `reason`: no existing Java/Maven target exists in the main ADL repo
- `proof_classification`: bounded_real_dry_run_demo

## Observed Result

- OpenRewrite executed successfully against the fixture.
- Maven completed with `BUILD SUCCESS`.
- the packet now records the pinned replayable tool coordinates:
  - plugin: `org.openrewrite.maven:rewrite-maven-plugin:6.39.0`
  - recipe artifact: `org.openrewrite.recipe:rewrite-static-analysis:2.34.1`
- the warm-cache pinned replay completed in approximately `4.614 s`.
- the repo-relative `.m2-local/` replay also completed successfully and is the
  retained command shape for portable proof.
- A dry-run patch was produced under the fixture at
  `target/rewrite/rewrite.patch`.
- The recipe identified the verbose generic constructor forms and proposed
  replacement with the Java diamond operator in one tracked Java file.

Tracked proof captures:

- `modernization_execution_command.md`
- `modernization_execution_log.txt`
- `modernization_rewrite.patch`

## Reviewer Expectations

A reviewer should be able to inspect this packet and answer:

1. what command family would be used?
2. why was dry-run chosen first?
3. why was a bounded fixture used instead of the main repo?
4. what would have to happen before mutation could be accepted in a real target
   repository?

## Non-Claims

- This packet does not claim the main ADL repository was modernized.
- This packet does not claim a production repository was modernized.
- This packet does not claim a live Moderne SaaS run.
