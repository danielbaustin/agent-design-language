# Starharvest Design Note

## Source Brief

`Make a cozy asteroid farming game.`

## Visual Thesis

A warm, lantern-lit orchard in deep space: dark navy void, apricot ship glow,
mint crop luminescence, rounded asteroid planters, and one premium airy control
rail instead of crowded HUD chrome.

## Content Plan

- primary playfield with asteroid gardens and ship
- top-line run HUD with timer, score, seeds, hull, and combo
- right rail for objective, selected asteroid, upgrades, and run outcome
- bottom hint bar for controls

## Interaction Thesis

- gentle inertial ship drift makes space feel soft rather than twitchy
- beam flashes make planting, tending, and harvesting visible
- drifting shard hazards preserve tension without breaking the cozy tone

## Gameplay Loop

1. Drift near an asteroid.
2. Plant an empty garden or tend a growing one.
3. Harvest ripe crops for stardust and seeds.
4. Buy a small upgrade if the score allows it.
5. Reach the target before time, oxygen, or hull runs out.

## Scope Cuts

- no external art pipeline
- no multiplayer
- no persistent save
- no audio engine
- no procedural map generation
- no mobile touch control tuning in this slice

## Honest Claim Boundary

The design is intentionally strong and demo-ready, but still bounded to one
small browser artifact rather than a general proof of five-minute software
creation.

