# Yield Nest - Zero Coupon bonds on Injective  

## Overview  

Yield Nest is a protocol that implements a Fixed Rate Lending protocol through the emission of Zero Coupon bonds.  
It is deployed on the Injective testnet blockchain and makes use of its native modules, including the TokenFactory and the Exchange modules.  


Fixed Income markets represent one of the most liquid markets in traditional finance (around 3x bigger than equity markets).
This interest for yield-bearing products is also found in cryptocurrency markets, most notably with liquidity pools for AMMs such as Osmosis or Uniswap, or even as staking (which is an essential part of governance in PoS blockchains).  
The main issue with these products is that they typically provide non-fixed interest rates (and even impermanent loss for liquidity provision in AMM) which makes it hard to estimate returns.  

Yield Nest addresses this by providing a way to create Zero-coupon bonds which enable users to optimize their capital with clear return targets. The use of Fungible Tokens as well as the reliance on native DEX on the Injective blockchain makes these debt products liquid unlike staking.  



This is a submission for the [Injective Illuminate Hackathon.](https://dorahacks.io/hackathon/illuminate/detail). The buidl can be accessed directly [here.](https://dorahacks.io/buidl/8497)  

Yield Nest has been deployed to testnet and can be [accessed through a dedicated UI.](https://yield-nest-ui.vercel.app/) 


## Key Features 

Yield Nest is a flexible protocol and enables the following:   
1- Creation of new debt products with different expiration dates and/or different collaterals.    

2- Debt positions are represented by Fungible tokens created using the TokenFactory module. As highlighted above these can be traded directly on native markets on Injective, but they can also be sent to other IBC-enabled blockchains such as Osmosis and be used in other protocols.    

3- Under-collateralized positions can be liquidated by any user once the Loan-to-value rises too much (this is set at 70% in our testnet deployment). Liquidations prevents bad debt, while liquidators receive a fee. 

4- Debt positions are created as zero-coupon bonds, which means the interest-rate is fixed at debt minting time 


## Creating a new Debt market   

1- A new debt token is created using the TokenFactory module, along with its metadata.  
2- A relevant market is created on the exchange module, either through a governance proposal or an immediate creation.  
3- Both the debt token and its market are registered on the smart contract for fast access using the RegisterDebtToken and RegisterMarket admin messages.  
4- The admin or market maker provides the initial liquidity on the market, both buy side and sell side
5- Users are now able to mint new debt and trade the debt asset on the market  


## Minting debt tokens  

New debt can be minted for an existing denomination using the Mint message.  

```rust
pub enum ExecuteMsg {
    ...
    Mint {
        target_denom: String,
        quantity: Uint128,
    },
    ...
}
```

The message must be sent with adequate collateral (Loan_to_value = debt_value / collateral must be lower than max LTV threshold).  
Only denominations for which the contract is the admin, and which haven't expired can be minted.  

The list of available denominations can be fetched using the GetDebtTokens message.

```rust 
pub enum QueryMsg {
    ...
    #[returns(GetDebtTokensResponse)]
    GetDebtTokens {},
    ...
}

```

While the parameters of the contract, including liquidation LTV threshold, can be fetched with the following message.  

```rust
pub enum QueryMsg {
    ...
    #[returns(GetProtocolSettingsResponse)]
    GetProtocolSettings {},
    ...
}
```


## Reimbursing debt  

Debt can be reimbursed at any times using the Repay message.  

```rust
pub enum ExecuteMsg {
    ...
    /// Repay debt position
    Repay {
        position_id: u64,
    },
    ...
}
```

The message must contain funds to repay the debt position, aka the debt asset that was minted on position creation.  
Partial repayment is allowed   


## Liquidation  

An existing debt position can be liquidated if its LTV is above the specified threshold for liquidation (70% in our testnet deployment).  

```rust
pub enum ExecuteMsg {
    ...
    /// Repay debt position
    Repay {
        position_id: u64,
    },
    ...
}
```

Existing debt positions can be accessed using the following query:  

```rust
pub enum QueryMsg {
    ...
    #[returns(GetBatchMintPositionsResponse)]
    GetBatchMintPositions { start_id: u64, count: u64 },
    ...
}
```

This will return information about existing debt positions, including their id, collateral value, debt value and current LTV. 


## Testnet   


Yield Nest has been deployed on the Injective testnet (injective-888).   
We've also created a sample debt asset (USDT_31DEC23) that uses INJ as a collateral, as well as a market to trade the debt itself (USDT_31DEC23/USDT)

```
Yield Nest address:  
inj19q4flnf78evuhvzcfqhq8x9e800rjraj2whanu   

Debt token denomination (USDT_31DEC23):    
factory/inj1m8vmsa84ha7up6cx3v7y7jj9egzl3u3vyzqml0/test_denom  

Market id for USDT_31DEC23/USDT:  
0xfd359c044664481b486665d3f7c5798faf6c2e5b88ca46399ddef71d9ee31fe3  
```

We recommend using our dedicated UI to interact with the contract (see Overview). You can also interact directly with the contract using the messages previously outlined.  



