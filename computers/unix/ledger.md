# ledger
Ledger is an accounting tool.

## Getting Started
Ledger has it's own format, which it refers to as `.dat`. These files are
ledger journal files, and it's recommended to keep the in version control.

## Accounts
At the highest level you have five sorts of accounts:
- Expenses: where money ges,
- Assets: where money sits
- Income: where money comes from
- Liabilities: money you owe
- Equity: the real value of your property

## Commodities
Currencies are commodities. By default Ledger will not convert currencies
(since it doesn't know their current values) so instead it will keep them as
separate.

## Balance
```sh
$ ledger bal   print balance

```
