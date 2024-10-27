# TOKIFY Protocol
>Tokenize, Invest, Earn

## Table of Contents

- [TL;DR](#tldr)
  - [Key Components](#key-components)
  - [Vision](#vision)
- [Technical Overview](#technical-overview)
  - [TOKIFY Legal Framework Overview](#tokify-legal-framework-overview)
  - [Economic Processes](#economic-processes)
    - [RWA Creation](#rwa-creation)
    - [Rent Payments](#rent-payments)
- [Technical Documentation](#technical-documentation)
  - [Contracts](#contracts)
    - cw-registry - coming soon..
    - [cw-dao](#cw-dao)
    - [cw-property](#cw-property)
    - cw-payments - coming soon..
    - cw-dao-voting - coming soon..
  - [Packages](#packages)
    - [cw-404](#cw-404)
  - [Frontend](#frontend)
- [Links](#links)


## TL;DR
**TOKIFY Protocol** is a smart contract-based platform on the MANTRA Chain that enables the tokenization of real-world assets (RWAs) through smart contracts and decentralized governance. Asset owners can tokenize physical assets like real estate, allowing fractional ownership and providing investors with liquid, accessible stakes in traditionally illiquid markets.

#### Key Components

1. **Smart Contract-Based Tokenization**: Assets are converted into RWA tokens, representing fractional ownership, enhancing liquidity and accessibility.
2. **Decentralized Governance**: A DAO framework enables transparent governance, with stakeholders voting on asset management and upgrades.
3. **Legal Compliance**: In collaboration with Dubai International Financial Center (DIFC), TOKIFY uses Special Purpose Vehicles (SPVs) to ensure assets comply with local regulations, protecting investor rights.

#### Vision
TOKIFY leverages MANTRA Chain’s smart contracts to democratize access to high-value assets, combining blockchain transparency with robust legal protections to redefine asset ownership in the digital economy.


## Technical Overview

TOKIFY integrates on-chain voting, asset management, and payment processing, enabling decentralized investment and management of RWAs.


1. **DAO Creation**:
   - The **DIFC Founder** initiates the process by creating a DAO via the **TOKIFY Factory**.
   - This step also registers new RWAs in the **DAO Contract (asset registry)**.

2. **Voting and Governance**:
   - The **DAO Contract** connects with a **voting token (CW4)**, enabling DAO members to participate in proposal votes.
   - These proposals are managed by the **vote contract (CW3)**.

3. **Asset Management**:
   - Upon vote approval, the DAO instantiates an **RWA contract (CW404)**.
   - This contract maps the asset to the DAO and allows **investors** to hold RWA tokens, representing their stake in the asset.

4. **Tenant Payments and Rewards**:
   - **Tenants** pay rent using **payment tokens (CW20)** through an **offramp/onramp service**.
   - This service facilitates the conversion between fiat and crypto tokens as needed.
   - Tenant payments are distributed as rewards to **investors** holding RWA tokens.

5. **Reward Claiming**:
   - **Investors** can claim rewards through the system, reinforcing their stake in the DAO and contributing to the asset's ongoing governance and revenue generation.
![alt text](./concept/00_technical_overview.png)


### TOKIFY Legal Framework Overview

TOKIFY's legal framework ensures that RWAs are digitally represented and managed within the Dubai International Financial Center's (DIFC) regulatory framework, bridging real-world legal entities and blockchain-based governance.


1. **Asset Ownership**:
   - The **DIFC Founder** initially owns the asset, then creates a **Special Purpose Vehicle (SPV)**, a holding company that instantiates and owns the asset through an **Asset LLC**.
   
2. **Management and Representation**:
   - The **Asset LLC** is managed by the SPV and legally represents the asset on the blockchain via the **CW404 RWA Contract**.
   - This contract manages digital ownership and compliance, embedding legal contracts and compliance reports.

3. **DAO Governance**:
   - A **DAO Contract** within the TOKIFY Protocol oversees the RWA contract, linking real-world asset ownership with digital token management.

4. **Investor Participation**:
   - **Investors** hold **CW404 tokens** that represent digital ownership, granting them a legal claim on the asset managed by the Asset LLC.

![alt text](./concept/01_legal_framework.png)


### Economic Processes

RWAs are digitally represented and managed within the DIFC's regulatory framework, bridging real-world legal entities and blockchain-based governance.


#### RWA Creation

1. **DAO Approval**:
   - A **DAO contract** makes a governance decision to initiate a new RWA, creating an **RWA contract** and publishing necessary legal documents.

2. **Token Issuance**:
   - The **RWA contract** issues RWA tokens to the **DIFC Founder**, representing ownership of the asset.
   
3. **Investor Participation**:
   - **Investors** can purchase RWA tokens using payment tokens, transferring ownership from the DIFC Founder to investors.
   - A transaction fee of 0.01% is applied for each transfer.

4. **Protocol Treasury**:
   - The collected fees (0.01%) from token transactions are allocated to the **TOKIFY protocol treasury** to support platform operations.

![alt text](./concept/11_rwa_creation_process.png)

#### Rent Payments
Rent payments are distributed to investors, with fees supporting the DAO and the TOKIFY protocol.

1. Tenants pay rent in the form of a payment token to the RWA contract.
2. The RWA contract deducts a 2% property management fee and transfers it to the DAO contract.
3. The RWA contract also deducts a 0.01% fee, which goes to the TOKIFY protocol treasury.
4. The remaining rent is distributed as partial rent to investors, who can then claim rewards.

![alt text](./concept/12_rent_payment_process.png)


## Technical Documentation

### Contracts Overview

**IMPORTANT: The smart contracts in this repository are currently unaudited and in active development.**

The contracts provided in this repository are part of the TOKIFY platform and are still in the development phase. They have not undergone a formal security audit. As such:

1. These contracts are not intended for production use at this time.
2. We strongly advise against using these contracts in any live or production environment.
3. TOKIFY and its developers do not assume any responsibility or liability for any issues, losses, or damages that may arise from the use of these contracts outside of the TOKIFY platform.
4. The contracts are provided "as is" without any warranties or guarantees of any kind, either expressed or implied.

We are actively working on improving and securing these contracts. A formal audit and further testing will be conducted before any production release. Until then, please consider these contracts as experimental and use them at your own risk.

For updates on the status of these contracts or for any questions, please refer to our official communication channels or contact the TOKIFY team directly.



#### [CW-DAO](./contracts/cw-dao)

The smart contract represents the DAO (e.g. a REIT - Real Estate Investment Trust). It must contain within it a registry that keeps track of all properties (cw-property/CW404 contracts) managed by that DAO. It must also keep track of proposals and execute any actions for the outcomes of votes (CW3/CW4 contracts).

#### [CW-PROPERTY](./contracts/cw-property)

Each property managed by a DAO is represented by a smart contract (cw-property) which uses the CW404 standard and extends it. For example: a property worth $100,000 can be represented by an NFT contract that has 100 shares, each share is initially sold for the price of $1000 or some token equivalent. This contract must also contain a payment splitting extension to allow the contract to receive payments which are then split according to the share ownership.

One benefit of using a CW404 is that shares do not have to be owned entirely but rather can be fractionalized and sold in smaller increments.

Rent is paid into the property’s contract in the form of CW20 tokens (see Payments Ramp below). The contract must keep track of rent accrual by each shareholder since shares can be transferred at any time.

#### [CW-404](./packages/cw404)

The CW404 standard is a hybrid token standard that combines features of both fungible (CW20) and non-fungible (CW721) tokens. It allows for the creation of tokens that can behave as both divisible assets and unique collectibles. This standard is particularly useful for representing fractional ownership of real-world assets (RWAs) in a flexible and efficient manner.

Key features of the CW404 standard include:
1. Divisibility: Tokens can be divided into smaller units, similar to fungible tokens.
2. Uniqueness: Each token can have distinct properties, like non-fungible tokens.
3. Automatic minting and burning: The contract automatically mints or burns tokens based on transfer amounts, maintaining the correct balance between fungible and non-fungible representations.
4. Gas efficiency: Optimized for reduced gas costs in token transfers and management.

The CW404 standard is ideal for the TOKIFY platform as it enables the seamless representation and management of fractional ownership in real estate and other RWAs, while providing the flexibility needed for various investment scenarios.

> Note: The CW404 standard is not a functional imperative for the architecture of Tokify. It can be replaced with a CW20, although some future development will require the use of CW404 and we wanted to start working on a standard package for CW404 that can be used by other projects in the community. Our implementation of CW404 is not yet complete and is still under development.


### Running Tests

The contracts are written in Rust and compiled to WebAssembly (WASM) for use on the MANTRA Chain.

They can be found in the `/contracts` folder. Each contract contains within it a set of tests using `multitest`. 

Simply run `cargo test` from the main directory to run all tests or `cd` into a specific contract folder to run its tests.

### Building Contracts

To build the contracts, run `cargo wasm` from each contract's folder. This will compile the contract and output the WASM files to the `/target` folder.

To generate the schema, run `cargo schema` from the main directory. This will generate the schema in the `/schema` folder.

To optimize the contracts and build all of them in a single step, run the following command from the main directory:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```

This will generate artifacts in the `/artifacts` folder and it contains the `.wasm` file for each contract.
These files can be uploaded to any network which supports `wasmd` version `0.51+` (`cosmwasm_1_4` or higher).


## Links

- [Send us an email](mailto:tokifyprotocol@gmail.com)
- TOKIFY Whitepaper - coming soon...
- [TOKIFY Telegram](https://t.me/TokiFyProtocol)
- [TOKIFY X / Twitter](https://x.com/TokiFyProtocol)
- TOKIFY Website - coming soon...
- [FAQ](./documentation/faq.md)
