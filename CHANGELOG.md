# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-11-06

### Changed

- All enums have been updated to serialize to snake_case
- The transaction_type has been updated from an object to instead be two fields
  on the transaction.

Previous example:
```json
{
    "amount": 100,
    "bank_reference_number": "1234",
    "continuations": [],
    "customer_reference_number": "1234",
    "funds_type": "ValueDated",
    "text": "Example text",
    "transaction_type": {
        "Credit": "MiscellaneousAchCredit"
    }
}
```

With these changes:

```json
{
    "amount": 100,
    "bank_reference_number": "1234",
    "continuations": [],
    "customer_reference_number": "1234",
    "direction": "credit",
    "funds_type": "value_dated",
    "text": "Example text",
    "transaction_type": "miscellaneous_ach_credit"
}
```

### Added

- This changelog, so that changes for future versions will be tracked

## [0.1.0] - 2023-10-03
