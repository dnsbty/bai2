# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2023-12-05

### Deleted

- The control fields have been removed from the file, group, and account
  structs. These fields are meant for checking the file's correctness, and the
  vectors of groups, accounts, and transactions should be used as the source of
  truth rather than these fields. Because of that I removed the fields.

### Changed

- Changed the transaction type to an enum and updated the serialization for it
  as well.
- Changed the amount type to an enum and updated the serialization for it as
  well.

Here is an overall diff of the output resulting from the changes made in this
update:

```diff
{
  ...
  "groups": [
    {
      ...
      "accounts": [
        {
          "amounts": [
            {
-             "amount_type": "current_ledger",
-             "amount_type_code": "030",
+             "amount_type": {
+               "code": "030",
+               "type": "status",
+               "subtype": "current_ledger"
+             },
            }
          ],
          ...
-         "number_of_records": 8,
          "transactions": [
            {
              ...
-             "direction": "debit",
              ...
-             "transaction_type": "preauthorized_ach_debit",
-             "transaction_type_code": "455",
+             "transaction_type": {
+               "code": "455",
+               "direction": "debit",
+               "type": "preauthorized_ach_debit"
+             },
              ...
            }
          ],
-         "total": 119528685,
          ...
        }
      ]
      ...
-     "number_of_accounts": 3,
-     "number_of_records": 48,
      ...
-     "total": 278988489,
      ...
    }
  ],
- "number_of_groups": 1,
- "number_of_records": 50,
- "total": 278988489,
  ...
}
```

## [0.3.0] - 2023-11-17

On top of the public changes listed below, this version also includes an
overhaul of the parsing logic. Parsing will now occur over two steps: scanning
will create a tree of file lines and then parsing will read the scanned tree to
create the actual file. While this change doesn't directly show up in the public
API, it leads to more accurate parsing by properly handling continuations, and
more closely aligns this parser to the official BAI2 specification.

### Added

- Transaction details now have an availability field that contains the amount
  that is available at each day provided
- Original transaction type codes and amount type codes are exposed in addition
  to their enum equivalents, which is especially helpful for custom codes

### Deleted

- Continuations are now a private detail, and will no longer show up in the
  returned fields.
- File physical record length and block size are no longer present since those
  are details necessary only for reading the file

### Changed

- File header and file control fields are now combined into a single JSON object
  at the top level of the output
- Group header and control fields are now combined into a single JSON object
- Account header and control fields are now combined into a single JSON object
- Accounts now have an `amounts` field that contains information for each of the
  amounts provided in the account information to better support the BAI2 spec
- Account currency codes now default to the group currency code if none is
  provided, and group currency codes default to USD as per the BAI2 spec
- Transaction detail text is now an array of strings, where the text from the
  transaction detail line and each continuation is a separate string in the
  array

### Security

- Update rustix from 0.38.14 to 0.38.24

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
