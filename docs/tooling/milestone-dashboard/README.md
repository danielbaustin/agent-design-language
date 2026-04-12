# Milestone Dashboard

This directory contains a reusable static HTML milestone dashboard.

## Purpose

The dashboard gives one operator-facing view of:
- milestone status
- sprint posture
- work-package coverage
- canonical source docs
- issue map
- immediate next actions and watchlist items

## Current dataset

The first bundled dataset is `v0.88`, drawn from:
- `docs/milestones/v0.88/README.md`
- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/SPRINT_v0.88.md`

## Files

- `index.html` — static dashboard shell
- `style.css` — visual system and responsive layout
- `dashboard.js` — milestone dataset and rendering logic

## Usage

Open `index.html` in a browser.

To adapt it for another milestone, update the `milestoneData` object in `dashboard.js`.
