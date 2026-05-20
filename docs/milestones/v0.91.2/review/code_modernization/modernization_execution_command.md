# Moderne / OpenRewrite Execution Command

## Executed Command

```bash
mvn -Dmaven.repo.local=.m2-local -U org.openrewrite.maven:rewrite-maven-plugin:6.39.0:dryRun \
  -Drewrite.recipeArtifactCoordinates=org.openrewrite.recipe:rewrite-static-analysis:2.34.1 \
  -Drewrite.activeRecipes=org.openrewrite.staticanalysis.UseDiamondOperator \
  -Drewrite.exportDatatables=true
```

## Working Directory

`docs/milestones/v0.91.2/review/code_modernization/fixture/use_diamond_operator_demo/`

## Why This Command

- uses the real OpenRewrite Maven plugin dry-run goal documented by OpenRewrite
- pins the observed plugin and recipe versions for replayable proof
- activates one bounded static-analysis recipe
- avoids mutating source files in place
- produces reviewable patch output under the Maven target tree

## Why `maven.repo.local` Was Set

- the ADL repository is not a Maven project, so this proof runs in a bounded
  tracked fixture
- the local Maven cache was redirected to a fixture-local `.m2-local/` path so
  the dry-run could execute cleanly without depending on a host-specific cache

## Observed Result

- build result: `BUILD SUCCESS`
- elapsed time on first pinned replay with a warm cache: approximately
  `4.614 s`
- recipe run: `org.openrewrite.staticanalysis.UseDiamondOperator`
- changed file count: `1`
- generated patch:
  `target/rewrite/rewrite.patch`
- tracked packet copy:
  `docs/milestones/v0.91.2/review/code_modernization/modernization_rewrite.patch`

## Authority Posture

- dry-run only
- no automatic patch acceptance
- no automatic merge
- fixture scope only
