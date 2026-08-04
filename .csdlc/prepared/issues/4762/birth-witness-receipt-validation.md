# #4762 Birth Witness Receipt Validation

## Purpose

Validate that the retained #4762 birth-witness and receipt package is concrete,
redaction-safe, handoff-consumable, and honest about non-claims.

## Command

```bash
ruby .csdlc/prepared/issues/4762/validate_birth_receipt_package.rb
```

## Expected Result

The validator must print `#4762 birth witness receipt package: PASS`.

## Scope

The validator checks retained JSON artifacts, required witness and negative
case identifiers, referenced source paths, claim boundaries, and exact handoff
consumer paths. It does not prove that a v0.92 birthday happened.
