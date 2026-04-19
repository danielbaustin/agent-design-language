# Vision - v0.90.2

## One Sentence

Run the first bounded CSM world and make it reviewable, observable, and
harder to fool.

## Why This Milestone Exists

v0.90.1 should create the first bounded Runtime v2 foundation prototype and the
CSM Observatory planning lane. That is not enough. A polis substrate must run in
bounded form and show what happens when things go wrong: illegal state
transitions, duplicate activation attempts, replay gaps, incomplete snapshots,
unsafe resumes, and adversarial pressure.

v0.90.2 exists to make the first small CSM world run, then turn that run into
something reviewers can trust under stress.

## North Star

A reviewer should be able to inspect one first-run packet and answer:

- Did `proto-csm-01` boot?
- Which citizens were admitted?
- Which governed episode ran?
- How did resource pressure affect scheduling?
- What did the Freedom Gate allow, defer, or refuse?
- Did snapshot, rehydrate, and wake preserve continuity?
- What did the Observatory packet/report show?
- Which invariants are enforced?
- What happens when an invariant fails?
- Is the violation artifact stable and complete enough to review?
- When is recovery allowed?
- When is quarantine required?
- Can an operator inspect, pause, resume, terminate, or quarantine with evidence?
- Which security-boundary proof defends the polis without turning security roles
  into the core Runtime v2 ontology?

## What Success Feels Like

v0.90.2 succeeds when Runtime v2 no longer feels like a diagram. It should feel
like a small city whose gates, clocks, ledgers, watchtowers, citizens, and
emergency rooms actually work.

## What This Must Not Become

This milestone must not become the first birthday, the moral civilization
milestone, or the complete security ecology milestone. It is hardening and proof
work around the first bounded CSM run.
