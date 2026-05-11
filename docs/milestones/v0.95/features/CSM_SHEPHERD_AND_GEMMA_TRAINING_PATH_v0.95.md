# v0.95 Feature: CSM Shepherd Model and Gemma Training Path

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Finish the MVP-facing evaluator and training path for the Shepherd-model /
Gemma line so the ANRM placement work from `v0.91.1` matures into a coherent
runtime-scaffold and training/evaluation story.

## Source Inputs

- `docs/milestones/v0.90.1/ideas/ANRM_SCAFFOLDED_SMALL_MODEL_EXPERIMENT.md`
- `docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded training/evaluator path for the Shepherd/Gemma line
- explicit relationship to ANRM placement, trace datasets, and runtime care
  semantics
- a truthful MVP home for the later small-model runtime story

## Non-goals

- claiming successful model training before evidence exists
- replacing the broader runtime substrate with one model family
- treating scaffold experiments as product completion by themselves

## Completion Target

`v0.95`
