# Ayclend

## Overview

Ayclend is a decentralized liquidity aggregation protocol built on the Solana blockchain that allows users to access a range of lending markets through a single platform, supporting cryptocurrencies such as SOL, USDC, USDT, wBTC (Portal), ETH (Portal), and BONK. The platform pools liquidity from various sources, offering competitive interest rates to lenders and lower interest rates to borrowers.

## Installation
> :warning: ayclend only compiles on the x86_64 architecture. This is to
> ensure struct sizes are always backwards compatible between the SVM and local
> development. Ensure the x86_64 arch is enabled before compiling the project.

## Architecture

Ayclend's protocol is made up of several key components, each playing a critical role in providing users with a reliable and efficient platform for managing their liquidity.

At the heart of the Ayclend protocol is the ayclend group. This group is a core component that enables users to manage risk and pool their resources to access lending markets more efficiently. Each ayclend group has a lending pool with unlimited banks. Within the lending pool, users can borrow and lend assets, which are then used to generate interest and distribute it among the members of the group. The ayclend group is responsible for managing the risk associated with these activities and ensuring that the borrowing and lending activities are within acceptable risk parameters.

Each bank within the lending pool has its own mint account and a custom oracle, currently limited to Pyth but will soon support Switchboard. This allows Ayclend to tap into multiple sources of liquidity and provide users with access to a diverse range of lending markets. Users can contribute liquidity to the lending pool and earn interest on their contributions. Users can also borrow from the pool to manage their own liquidity needs.

Ayclend accounts are used by users to interact with the protocol. Each ayclend account belongs to a single group and can borrow up to 16 assets simultaneously, providing users with greater flexibility in managing their liquidity. Users can deposit assets into their ayclend account and use them to borrow other assets or lend them to the lending pool. The account balance and borrowing capacity are continuously updated based on user activity and the risk associated with their borrowing and lending activities.

To maintain account health, Ayclend uses a deterministic risk engine that monitors user activity and ensures that borrowing and lending activities are within acceptable risk parameters. The risk engine uses a variety of metrics, including asset prices, volatility, and liquidity, to determine the appropriate risk parameters for each user's ayclend account. If a user's account falls below the minimum required health factor, they may be subject to liquidation to protect the integrity of the lending pool and other users' accounts.

Overall, Ayclend's architecture is designed to provide users with a powerful and flexible platform for managing their liquidity. By leveraging ayclend groups, multiple banks, ayclend accounts, and a robust risk management system, the platform is able to offer competitive interest rates and reliable access to a wide range of lending markets.

```
                     ┌────────────┐       ┌────────────┐       ┌───────────┐       ┌──────────┐
                     │ Ayclend   |       │ Lending    │       │           │       │ Price    │
                     │ Group      │1─────1│ Pool       │1─────n│ Bank      │m─────n│ Oracle   │
                     │            │       │            │       │           │       │          │
                     └────────────┘       └────────────┘       └───────────┘       └──────────┘
                           1                    1
                           │                    │
                           │                    │
                           n                    1
┌───────────┐        ┌────────────┐       ┌────────────┐
│           │        │ Margin     │       │ Lending    │
│ Signer    │1──────n│ Account    │1─────1│ Account    │
│           │        │            │       │            │
└───────────┘        └────────────┘       └────────────┘
```

## Risk Management

One of the key features of Ayclend is its risk management system. Risk is managed at the ayclend group level, where each bank defines its own risk parameters and uses asset and liability weights to determine loan-to-value ratios (LTVs). Assets can be isolated to reduce the risk of contagion, and real-time risk monitoring is used to assess changing market conditions and adjust risk parameters as needed. Ayclend's risk management system is transparent and deterministic, providing users with clear information about their risk exposure. If a user's account falls below the minimum required health factor, they may be subject to liquidation to protect the integrity of the lending pool and other users' accounts.

Key points:

- Ayclend has a robust risk management system.
- Risk is managed at the ayclend group level.
- Each bank defines its own risk parameters.
- Assets can be isolated to reduce contagion risk.
- Real-time risk monitoring is used to assess changing market conditions.
- Ayclend's risk management system is transparent and deterministic.
- Liquidation may occur if a user's account falls below the minimum required health factor.

## Building
./scripts/build-program.sh ayclend devnet

## Testing
cargo test
Be sure to use an x86 toolchain when compiling and running the tests.
