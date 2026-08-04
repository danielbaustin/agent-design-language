# Issue 5332 design

## Decision

Treat the Unity IL Post Processor `GetDomainName: -1` loop as a distinct,
repeatable startup state that can be classified from bounded log signatures and
then diagnosed through a controlled environment matrix.

The implementation must not use an arbitrary total-runtime ceiling. It should
fail once the same ILPP retry signature repeats without semantic import,
compile, validator, or terminal progress. Successful ILPP startup must continue
through the existing proof path.

## Ownership boundary

Issue #5332 owns:

- the ILPP retry-loop signature classifier;
- a controlled matrix isolating Unity version, host/domain identity, mutable
  HOME/TMP/XDG state, and wrapper staging;
- focused no-Unity fixtures for loop, progress, and normal-start behavior;
- registration of the classifier, wrapper, focused unit proof, and diagnosis
  packet in the repository validation-lane selector;
- retained root-cause or irreducible blocker evidence.

Issue #4741 owns editor liveness, execution-mode selection, staging lifecycle,
and the general progress watchdog. Issue #4739 owns Unity-MCP project and
endpoint alignment. #5332 does not own scene code, asset fallback, runtime
contract semantics, or investor rendering.

## Diagnostic matrix

Run the smallest safe matrix needed to distinguish:

1. approved host environment with the repository wrapper;
2. isolated mutable HOME/TMP/XDG state with the same project stage;
3. approved host/domain identity versus missing or malformed domain identity;
4. current approved Unity editor versus an issue-approved comparison editor,
   only if the first three cells do not isolate the cause.

Each cell records the exact Unity version, canonical staged project, relevant
non-secret environment shape, first and last semantic progress marker, ILPP
signature count, terminal classification, and retained log reference.

## Classifier design

- Enter `ilpp_retry_loop` only after the complete signature family repeats:
  connectivity retry, gRPC or type-initialization failure, and
  `GetDomainName: -1`.
- Reset the non-progress count after verified import, compile, validator, or
  terminal progress.
- Keep readonly-database text independent: it is fatal only when staged import
  never progresses.
- Emit one concise terminal classification and route ownership to #5332.
- Do not print credentials, complete environment dumps, or broad host process
  state.

## Focused proof

- Run the no-Unity classifier fixtures for loop, progress-reset, and normal
  startup behavior.
- Run the repository validation-lane selector test and verify every changed
  implementation, wrapper, unit, and diagnosis path maps to the focused Unity
  ILPP lane.
- Run only the smallest staged Unity reproduction cells needed to distinguish
  root cause after the no-Unity proof is green.
