# Kernel Services And Control Plane

## Purpose

Define the bounded Runtime v2 kernel service loop and operator controls.

## Required Services

- clock service
- identity/admission guard
- scheduler
- resource ledger
- trace writer
- snapshot manager
- invariant checker
- operator control interface

## Operator Controls

The operator should be able to:

- inspect manifold status
- inspect citizen status
- pause the manifold
- resume the manifold
- request snapshot
- terminate the manifold
- inspect last invariant/security failures

## Proof Surface

The control plane must emit a report showing:

- command requested
- pre-state
- post-state
- affected service
- trace event ref
- allowed/refused/deferred outcome

## Boundary

This is not autonomous release approval, governance voting, or social-contract
execution. Those remain later work.
