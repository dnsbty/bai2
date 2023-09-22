<br />

<div align="center">
    <h1>bai2</h1>
    <p><h3 align="center">A tool for parsing BAI2 files</h3></p>
</div>

<hr>

### Usage

```sh
cargo install bai2
```

To parse a bai2 file, just provide the filename!

```sh
bai2 my_file.bai
```

### Examples

Given a BAI2 file `test.bai` like below:

```
01,GSBI,ABC,200331,2300,1,,,2/
02,,GSBI,1,200331,2300,,,/
03,123456,USD,010,10000,,,/
16,495,1000,,I1220012,endtoendID,To: Payee account, Account: XXXXX-4454, Client Ref ID: endtoendID, GS ID:I1220012/
88,CREF: 9f4396bd-8c47-4893-8682-bd8ff006d140
49,11000,2/
98,12000,2,5/
99,22000,2,10/
```

#### View as JSON

```sh
bai2 test.bai
```

```json
{
  "continuations": [],
  "file_header": {
    "sender": "GSBI",
    "receiver": "ABC",
    "creation_date": "2020-03-31",
    "creation_time": "23:00:00",
    "file_id": "1",
    "physical_record_length": "",
    "block_size": "",
    "version_number": "2"
  },
  "groups": [
    {
      "header": {
        "receiver": "",
        "sender": "GSBI",
        "status": "1",
        "as_of_date": "2020-03-31",
        "as_of_time": "23:00:00",
        "currency_code": "",
        "as_of_date_modifier": ""
      },
      "control": {
        "total": "12000",
        "number_of_accounts": "2",
        "number_of_records": "5"
      },
      "accounts": [
        {
          "header": {
            "account_number": "123456",
            "currency_code": "USD",
            "type_code": "010",
            "amount": "10000",
            "item_code": "",
            "funds_type": ""
          },
          "control": {
            "total": "11000",
            "number_of_records": "2"
          },
          "transactions": [
            {
              "type_code": "495",
              "amount": "1000",
              "funds_type": "",
              "bank_reference_number": "I1220012",
              "customer_reference_number": "endtoendID",
              "text": "To: Payee account",
              "continuations": [
                {
                  "text": "CREF: 9f4396bd-8c47-4893-8682-bd8ff006d140"
                }
              ]
            }
          ],
          "continuations": []
        }
      ],
      "continuations": []
    }
  ],
  "file_control": {
    "total": "22000",
    "number_of_groups": "2",
    "number_of_records": "10"
  },
  "last_record_type": "File"
}
```

### Resources

I very heavily relied on documentation from these sources while writing this library:

- [Official BAI2 Spec](https://www.bai.org/docs/default-source/libraries/site-general-downloads/cash_management_2005.pdf)
- [Goldman Sachs BAI File
  Guide](https://developer.gs.com/docs/services/transaction-banking/bai-file/)
- [TD BAI Format](https://www.tdcommercialbanking.com/document/PDF/bai.pdf)

### Credits

The parser was very heavily inspired by [Leejay Hsu's nacha
tool](https://github.com/leejayhsu/nacha).
