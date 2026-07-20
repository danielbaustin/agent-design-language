# #5495 Publication metadata review guard

The typed publication path may commit lifecycle cards, issue projections, prepared JSON requests, and publication intent after review. These paths are metadata only when their shapes are explicitly recognized. Source files, retained design artifacts, arbitrary requests, and unknown paths remain substantive.

The review guard derives an automatic non-substantive proof only when the exact reviewed commit and current commit differ solely by recognized lifecycle metadata. An explicit malformed proof is never upgraded by this automatic path. Merged reconciliation continues to validate repository, PR, base, head, draft state, and reviewed commit identity.
