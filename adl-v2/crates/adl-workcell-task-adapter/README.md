# ADL Workcell Task Adapter

This crate executes only conductor-approved task transport operations. It owns
no lifecycle, scheduling, merge, issue-creation, or transcript-retention
authority. Retained receipts contain identifiers, digests, status classes, and
evidence references, never private context bodies or transport error text.
