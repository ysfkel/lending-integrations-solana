# Solana DeFi Market Integration with Solend

This project demonstrates an integration with the Solend lending protocol on Solana, allowing users to interact with lending and borrowing functionalities through a custom program called `defi-market`.

## Overview

The `defi-market` program acts as an intermediary, enabling users to:

*   **Deposit Reserve Liquidity:** Supply assets to a Solend reserve.
*   **Deposit Obligation Collateral:** Provide collateral to Solend for borrowing.
*   **Borrow Obligation Liquidity:** Borrow assets from a Solend reserve, backed by collateral.
*   **Repay Obligation Liquidity:** Repay borrowed assets to Solend.
*   **Initialize Obligation:** Create an obligation account on Solend to manage collateral and borrowed assets.

## Architecture

The project consists of:

*   **`defi-market` Program:** A Solana program (written in Rust with Anchor) that exposes instructions to interact with Solend's lending program via CPI (Cross-Program Invocations).
*   **`tests`:** A set of integration tests (also in Rust with Anchor) that demonstrate the functionality of the `defi-market` program.
*   **`constants.rs`:** Defines constants for interacting with devnet solend like lending program id, lending market id ...
*   **`Anchor.toml`:** Anchor configuration file defining the program ID, cluster, and wallet settings.
*   **Cargo.toml:** configurations for the project.

## Key Concepts Demonstrated

This project showcases a solid understanding of:

*   **Solana Program Development:** Writing Solana programs using Anchor framework.
*   **Cross-Program Invocations (CPI):** Interacting with the Solend program from a custom program.
*   **Solend Protocol Integration:** Understanding the core concepts of Solend (reserves, obligations, collateral, borrowing).
*   **Account Management:** Creating and managing accounts for obligations.
*   **Token Handling:** Using `anchor-spl` to interact with SPL tokens and associated token accounts (ATAs).
*   **Integration Testing:** Writing integration tests to verify program behavior and CPI calls.
* **Rent management:** the project use `rent::id` to use sysvar rent.

## Instructions Implemented

The `defi-market` program defines the following instructions:

*   **`init_obligation`:** Initializes an obligation account on Solend.
*   **`deposit_reserve_liquidity`:** Deposits liquidity into a Solend reserve.
*   **`deposit_obligation_collateral`:** Deposits collateral into a Solend obligation.
*   **`borrow_obligation_liquidity`:** Borrows liquidity from a Solend reserve.
* **`repay_obligation_liquidity`:** repays liquidity to a solend reserve.

## How to Run the Tests

1. **Run the tests**: `anchor test`

## Important considerations

* **Devnet addresses** : this project is using devnet solend addresses, if you want to use it on mainnet you have to change the `constants.rs` to mainnet adresses.
* **solana test validator**: You have to clone the solend program id to your local test validator.

## Future Improvements

*   **Error Handling:** Implement more robust error handling and reporting.
*   **More Test Cases:** Add more test cases to cover edge cases and different scenarios.
*   **UI:** Develop a user interface to interact with the program.
* **Mainnet**: adapt to work with mainnet solend pool.

## Conclusion

This project provides a functional example of integrating with Solend using a custom Solana program. The code is well-structured, tested, and demonstrates a strong understanding of Solana development principles.
