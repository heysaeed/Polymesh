// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_capital_distribution
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-24, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `ubuntu-8gb-hel1-5`, CPU: `AMD EPYC Processor`

// Executed Command:
// target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=*
// -e=*
// --heap-pages
// 4096
// --db-cache
// 512
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./pallets/weights/src/
// --template
// ./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_capital_distribution using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_corporate_actions::distribution::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Proof Skipped: CapitalDistribution Distributions (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Proof Skipped: Portfolio PortfolioCustodian (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Proof Skipped: Portfolio Portfolios (max_values: None, max_size: None, mode: Measured)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Proof Skipped: CorporateAction CorporateActions (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset Tokens (r:1 w:0)
    // Proof Skipped: Asset Tokens (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Proof Skipped: Portfolio PortfolioAssetBalances (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Proof Skipped: Portfolio PortfolioLockedAssets (max_values: None, max_size: None, mode: Measured)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Proof Skipped: ProtocolFee Coefficient (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Proof Skipped: ProtocolFee BaseFees (max_values: None, max_size: None, mode: Measured)
    fn distribute() -> Weight {
        // Minimum execution time: 153_536 nanoseconds.
        Weight::from_ref_time(172_793_000)
            .saturating_add(DbWeight::get().reads(13))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: CapitalDistribution HolderPaid (r:1 w:1)
    // Proof Skipped: CapitalDistribution HolderPaid (max_values: None, max_size: None, mode: Measured)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Proof Skipped: CapitalDistribution Distributions (max_values: None, max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Proof Skipped: CorporateAction CorporateActions (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint SchedulePoints (r:1 w:0)
    // Proof Skipped: Checkpoint SchedulePoints (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset BalanceOf (r:3 w:2)
    // Proof Skipped: Asset BalanceOf (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset Tokens (r:1 w:0)
    // Proof Skipped: Asset Tokens (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Proof Skipped: Portfolio PortfolioLockedAssets (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset Frozen (r:1 w:0)
    // Proof Skipped: Asset Frozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Proof Skipped: Portfolio Portfolios (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Proof Skipped: Portfolio PortfolioAssetBalances (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Proof Skipped: Statistics AssetTransferCompliances (max_values: None, max_size: None, mode: Measured)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Proof Skipped: ComplianceManager AssetCompliances (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint CachedNextCheckpoints (r:1 w:0)
    // Proof Skipped: Checkpoint CachedNextCheckpoints (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Proof Skipped: Checkpoint CheckpointIdSequence (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1000]`.
    /// The range of component `w` is `[0, 1000]`.
    fn claim(t: u32, w: u32) -> Weight {
        // Minimum execution time: 375_502 nanoseconds.
        Weight::from_ref_time(469_029_637)
            // Standard Error: 8_792
            .saturating_add(Weight::from_ref_time(78_865).saturating_mul(t.into()))
            // Manually set weight for `w`
            .saturating_add(Weight::from_ref_time(17_224).saturating_mul(w.into()))
            .saturating_add(DbWeight::get().reads(19))
            .saturating_add(DbWeight::get().writes(7))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: CapitalDistribution HolderPaid (r:1 w:1)
    // Proof Skipped: CapitalDistribution HolderPaid (max_values: None, max_size: None, mode: Measured)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Proof Skipped: CapitalDistribution Distributions (max_values: None, max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Proof Skipped: CorporateAction CorporateActions (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint SchedulePoints (r:1 w:0)
    // Proof Skipped: Checkpoint SchedulePoints (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset BalanceOf (r:3 w:2)
    // Proof Skipped: Asset BalanceOf (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset Tokens (r:1 w:0)
    // Proof Skipped: Asset Tokens (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Proof Skipped: Portfolio PortfolioLockedAssets (max_values: None, max_size: None, mode: Measured)
    // Storage: Asset Frozen (r:1 w:0)
    // Proof Skipped: Asset Frozen (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Proof Skipped: Portfolio Portfolios (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Proof Skipped: Portfolio PortfolioAssetBalances (max_values: None, max_size: None, mode: Measured)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Proof Skipped: Statistics AssetTransferCompliances (max_values: None, max_size: None, mode: Measured)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Proof Skipped: ComplianceManager AssetCompliances (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint CachedNextCheckpoints (r:1 w:0)
    // Proof Skipped: Checkpoint CachedNextCheckpoints (max_values: None, max_size: None, mode: Measured)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Proof Skipped: Checkpoint CheckpointIdSequence (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1000]`.
    /// The range of component `w` is `[0, 1000]`.
    fn push_benefit(t: u32, w: u32) -> Weight {
        // Minimum execution time: 379_248 nanoseconds.
        Weight::from_ref_time(407_987_885)
            // Standard Error: 9_464
            .saturating_add(Weight::from_ref_time(130_798).saturating_mul(t.into()))
            // Standard Error: 9_464
            .saturating_add(Weight::from_ref_time(71_126).saturating_mul(w.into()))
            .saturating_add(DbWeight::get().reads(22))
            .saturating_add(DbWeight::get().writes(7))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Proof Skipped: CapitalDistribution Distributions (max_values: None, max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Proof Skipped: Portfolio PortfolioCustodian (max_values: None, max_size: None, mode: Measured)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Proof Skipped: Portfolio PortfolioLockedAssets (max_values: None, max_size: None, mode: Measured)
    fn reclaim() -> Weight {
        // Minimum execution time: 104_056 nanoseconds.
        Weight::from_ref_time(107_049_000)
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Proof Skipped: Identity KeyRecords (max_values: None, max_size: None, mode: Measured)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Proof Skipped: ExternalAgents GroupOfAgent (max_values: None, max_size: None, mode: Measured)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Proof Skipped: Permissions CurrentPalletName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Proof Skipped: Permissions CurrentDispatchableName (max_values: Some(1), max_size: None, mode: Measured)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Proof Skipped: CapitalDistribution Distributions (max_values: None, max_size: None, mode: Measured)
    // Storage: Timestamp Now (r:1 w:0)
    // Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Proof Skipped: Portfolio PortfolioLockedAssets (max_values: None, max_size: None, mode: Measured)
    fn remove_distribution() -> Weight {
        // Minimum execution time: 329_355 nanoseconds.
        Weight::from_ref_time(358_200_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(2))
    }
}
