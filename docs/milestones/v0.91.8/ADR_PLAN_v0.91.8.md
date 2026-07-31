# v0.91.8 ADR Plan

Issue `#5728` records the minimum durable ADR set for architecture decisions
already accepted by merged v0.91.8 work.

| Topic | Owning evidence | ADR | Disposition |
| --- | --- | ---: | --- |
| ADL v2 modular clean-room architecture | #5336, #5339, #5338, #5340, #5384 | 0052 | accepted |
| Portable records signing and external trust | #5342 | 0053 | accepted |
| Runtime v3 guardian-owned kernel and API boundary | #5341, #5361, #5590 | 0054 | accepted |
| Runtime v3 unified durable state | #5663, #5698 | 0055 | accepted |
| C-SDLC v2 final authority and v1 sunset | #5358, #5541 | 0056 | accepted |
| ADL v2 generation selector and rollback | #5350, #5344, #5343 | 0057 | accepted |
| Memory Palace acceptance | #5007, WP-21 #5362 | 0051 | deferred pending implementation evidence |

OpenAPI, WSS, TLS, Observatory, and OTel details remain consequences or
interfaces of ADR 0054 and existing ADR 0048 unless a future incompatible
authority change requires a separate decision. They are not duplicated here.
