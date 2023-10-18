# okx-rs
Unofficial Rust Library for the OKX V5 API.

## Usage
```toml
[dependencies]
okx-rs = { git = "https://github.com/roytang121/okx-rs" }
```

<!-- extracted from https://www.okx.com/docs-v5/en/ -->
## V5 API Features
### Trading Account
<details>
  <summary>Rest</summary>

- [ ] REST / Get balance
- [ ] REST / Get positions
- [ ] REST / Get positions history
- [ ] REST / Get account and position risk
- [ ] REST / Get bills details (last 7 days)
- [ ] REST / Get bills details (last 3 months)
- [ ] REST / Get account configuration
- [ ] REST / Set position mode
- [ ] REST / Set leverage
- [ ] REST / Get maximum buy/sell amount or open amount
- [ ] REST / Get maximum available tradable amount
- [ ] REST / Increase/decrease margin
- [ ] REST / Get leverage
- [ ] REST / Get leverage estimated info
- [ ] REST / Get the maximum loan of instrument
- [ ] REST / Get fee rates
- [ ] REST / Get interest accrued data
- [ ] REST / Get interest rate
- [ ] REST / Set greeks (PA/BS)
- [ ] REST / Isolated margin trading settings
- [ ] REST / Get maximum withdrawals
- [ ] REST / Get account risk state
- [ ] REST / Manual borrow and repay in Quick Margin Mode
- [ ] REST / Get borrow and repay history in Quick Margin Mode
- [ ] REST / VIP loans borrow and repay
- [ ] REST / Get borrow and repay history for VIP loans
- [ ] REST / Get VIP interest accrued data
- [ ] REST / Get VIP interest deducted data
- [ ] REST / Get VIP loan order list
- [ ] REST / Get VIP loan order detail
- [ ] REST / Get borrow interest and limit
- [ ] REST / Position builder
- [ ] REST / Get Greeks
- [ ] REST / Get PM position limitation
- [ ] REST / Set risk offset type
- [ ] REST / Activate option
- [ ] REST / Set auto loan
- [ ] REST / Set account mode
- [ ] REST / Reset MMP Status
- [ ] REST / Set MMP
- [ ] REST / GET MMP Config

</details>

<details>
  <summary>Websocket</summary>

- [ ] WS / Account channel
- [ ] WS / Positions channel
- [ ] WS / Balance and position channel
- [ ] WS / Position risk warning
- [ ] WS / Account greeks channel

</details>

### Orderbook Trading
<details>
  <summary>Endpoints</summary>

- [ ] POST / Place order
- [ ] POST / Place multiple orders
- [ ] POST / Cancel order
- [ ] POST / Cancel multiple orders
- [ ] POST / Amend order
- [ ] POST / Amend multiple orders
- [ ] POST / Close positions
- [ ] GET / Order details
- [ ] GET / Order List
- [ ] GET / Order history (last 7 days)
- [ ] GET / Order history (last 3 months)
- [ ] GET / Transaction details (last 3 days）
- [ ] GET / Transaction details (last 3 months)
- [ ] GET / Easy convert currency list
- [ ] POST / Place easy convert
- [ ] GET / Easy convert history
- [ ] GET / One-click repay currency list
- [ ] POST / Trade one-click repay
- [ ] GET / One-click repay history
- [ ] POST / Mass cancel order
- [ ] POST / Cancel All After
- [ ] WS / Order channel
- [ ] WS / Place order
- [ ] WS / Place multiple orders
- [ ] WS / Cancel order
- [ ] WS / Cancel multiple orders
- [ ] WS / Amend order
- [ ] WS / Amend multiple orders
- [ ] WS / Mass cancel order
- [ ] POST / Place algo order
- [ ] POST / Cancel algo order
- [ ] POST / Amend algo order
- [ ] POST / Cancel advance algo order
- [ ] GET / Algo order details
- [ ] GET / Algo order list
- [ ] GET / Algo order history
- [ ] WS / Algo orders channel
- [ ] WS / Advance algo orders channel
- [ ] POST / Place grid algo order
- [ ] POST / Amend grid algo order
- [ ] POST / Stop grid algo order
- [ ] POST / Close position for contract grid
- [ ] POST / Cancel close position order for contract grid
- [ ] POST / Instant trigger grid algo order
- [ ] GET / Grid algo order list
- [ ] GET / Grid algo order history
- [ ] GET / Grid algo order details
- [ ] GET / Grid algo sub orders
- [ ] GET / Grid algo order positions
- [ ] POST / Spot/Moon grid withdraw income
- [ ] POST / Compute margin balance
- [ ] POST / Adjust margin balance
- [ ] GET / Grid AI parameter (public)
- [ ] POST / Compute min investment (public)
- [ ] GET / RSI back testing (public)
- [ ] WS / Spot grid algo orders channel
- [ ] WS / Contract grid algo orders channel
- [ ] WS / Moon grid algo orders channel
- [ ] WS / Grid positions channel
- [ ] WS / Grid sub orders channel
- [ ] POST / Place recurring buy order
- [ ] POST / Amend recurring buy order
- [ ] POST / Stop recurring buy order
- [ ] GET / Recurring buy order list
- [ ] GET / Recurring buy order history
- [ ] GET / Recurring buy order details
- [ ] GET / Recurring buy sub orders
- [ ] WS / Recurring buy orders channel
- [ ] GET / Existing leading positions
- [ ] GET / Leading position history
- [ ] POST / Place leading stop order
- [ ] POST / Close leading position
- [ ] GET / Leading instruments
- [ ] POST / Amend leading instruments
- [ ] GET / Profit sharing details
- [ ] GET / Total profit sharing
- [ ] GET / Unrealized profit sharing details
- [ ] GET / Tickers
- [ ] GET / Ticker
- [ ] GET / Order book
- [ ] GET / Order lite book
- [ ] GET / Candlesticks
- [ ] GET / Candlesticks history
- [ ] GET / Trades
- [ ] GET / Trades history
- [ ] GET / Option trades by instrument family
- [ ] GET / Option trades
- [ ] GET / 24H total volume
- [ ] WS / Tickers channel
- [ ] WS / Candlesticks channel
- [ ] WS / Trades channel
- [ ] WS / All trades channel
- [ ] WS / Order book channel
- [ ] WS / Option trades channel

</details>

### Funding Account
<details>
  <summary>Rest</summary>

- [ ] REST / Get currencies
- [ ] REST / Get balance
- [ ] REST / Get non-tradable assets
- [ ] REST / Get account asset valuation
- [ ] REST / Funds transfer
- [ ] REST / Get funds transfer state
- [ ] REST / Asset bills details
- [ ] REST / Lightning deposits
- [ ] REST / Get deposit address
- [ ] REST / Get deposit history
- [ ] REST / Withdrawal
- [ ] REST / Lightning withdrawals
- [ ] REST / Cancel withdrawal
- [ ] REST / Get withdrawal history
- [ ] REST / Get deposit withdraw status
- [ ] REST / Small assets convert
- [ ] REST / Get exchange list (public)
- [ ] REST / Get convert currencies
- [ ] REST / Get convert currency pair
- [ ] REST / Estimate quote
- [ ] REST / Convert trade
- [ ] REST / Get convert history

</details>

<details>
  <summary>Websocket</summary>

- [ ] WS / Deposit info channel
- [ ] WS / Withdrawal info channel

</details>

### Public data
<details>
  <summary>Rest</summary>

- [x] REST / Get instruments
- [x] REST / Get delivery/exercise history
- [x] REST / Get open interest
- [x] REST / Get funding rate
- [x] REST / Get funding rate history
- [x] REST / Get limit price
- [ ] REST / Get option market data
- [ ] REST / Get estimated delivery/exercise price
- [x] REST / Get discount rate and interest-free quota
- [x] REST / Get system time
- [x] REST / Get mark price
- [x] REST / Get position tiers
- [ ] REST / Get interest rate and loan quota
- [ ] REST / Get interest rate and loan quota for VIP loans
- [x] REST / Get underlying
- [x] REST / Get insurance fund
- [ ] REST / Unit convert
- [ ] REST / Get option tick bands
- [x] REST / Get index tickers
- [x] REST / Get index candlesticks
- [x] REST / Get index candlesticks history
- [x] REST / Get mark price candlesticks
- [x] REST / Get mark price candlesticks history
- [ ] REST / Get oracle
- [ ] REST / Get exchange rate
- [x] REST / Get index components

</details>
<details>
  <summary>Websockets</summary>

- [x] WS / Instruments channel
- [ ] WS / Open interest channel
- [ ] WS / Funding rate channel
- [ ] WS / Price limit channel
- [ ] WS / Option summary channel
- [ ] WS / Estimated delivery/exercise price channel
- [x] WS / Mark price channel
- [x] WS / Index tickers channel
- [ ] WS / Mark price candlesticks channel
- [ ] WS / Index candlesticks channel
- [ ] WS / Liquidation orders channel

</details>
  
### Financial Product
<details>
  <summary>Earn</summary>

- [ ] GET / offers
- [ ] POST / Purchase
- [ ] POST / Redeem
- [ ] POST / Cancel purchases/redemptions
- [ ] GET / Active orders
- [ ] GET / Order history

</details>

<details>
  <summary>Savings</summary>

- [ ] GET / Saving balance
- [ ] POST / Savings purchase/redemption
- [ ] POST / Set lending rate
- [ ] GET / Lending history
- [ ] GET / public borrow info (public)
- [ ] GET / Public borrow history (public)

</details>

### Trading Statistics

<details>
  <summary>Rest</summary>

- [ ] REST / Get support coin
- [ ] REST / Get taker volume
- [ ] REST / Get margin lending ratio
- [ ] REST / Get long/short ratio
- [ ] REST / Get contracts open interest and volume
- [ ] REST / Get options open interest and volume
- [ ] REST / Get put/call ratio
- [ ] REST / Get open interest and volume (expiry)
- [ ] REST / Get open interest and volume (strike)
- [ ] REST / Get taker flow

</details>

### Sub-account
<details>
  <summary>Rest</summary>

- [ ] REST / Get sub-account list
- [ ] REST / Reset the API Key of a sub-account
- [ ] REST / Get sub-account trading balance
- [ ] REST / Get sub-account funding balance
- [ ] REST / Get sub-account maximum withdrawals
- [ ] REST / Get history of sub-account transfer
- [ ] REST / Get history of managed sub-account transfer
- [ ] REST / Master accounts manage the transfers between sub-accounts
- [ ] REST / Set permission of transfer out
- [ ] REST / Get custody trading sub-account list
- [ ] REST / Get the user's affiliate rebate information
- [ ] REST / Set sub-accounts VIP loan allocation
- [ ] REST / Get sub-account borrow interest and limit

</details>

### Status
<details>
  <summary>Rest</summary>

- [ ] REST / Status

</details>

<details>
  <summary>Websocket</summary>

- [ ] WS / Status channel
</details>
