#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_types::sol_data::Bool;
// use ethers::core::k256::elliptic_curve::consts::U8;
use stylus_sdk::{
    alloy_primitives::{Address, U256, U8},
    alloy_sol_types::sol,
    evm, msg,
    prelude::*,
    storage::{StorageAddress, StorageMap, StorageString, StorageU256},
};

const PLATFORM_WALLET: &str = "0x9dcBe706E49b055Ca1BD5cc4741bfbbA34bc83FD";

sol_storage! {
    #[entrypoint]
    pub struct MatchUpContract {
        address creator;
        address challenger;
        uint256 bet_amount;
        uint8 status;
        bool creator_ready;
        bool challenger_ready;
        mapping(address => string) user_profiles;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MatchStatus {
    Open = 0,
    InProgress = 1,
    Completed = 2,
    Cancelled = 3,
}

impl MatchStatus {
    pub fn from_u8(value: u8) -> Option<MatchStatus> {
        match value {
            0 => Some(MatchStatus::Open),
            1 => Some(MatchStatus::InProgress),
            2 => Some(MatchStatus::Completed),
            3 => Some(MatchStatus::Cancelled),
            _ => None,
        }
    }
}

// Define events similar to Solidity
sol! {
    event UserProfileCreated(address indexed user, string riotId);
    event MatchCreated(uint256 matchId, address creator, uint256 betAmount, uint8 matchType);
    event MatchJoined(uint256 matchId, address challenger);
    event PlayerReady(uint256 matchId, address player);
    event MatchStarted(uint256 matchId);
    event MatchCompleted(uint256 matchId, address winner);
    event MatchCancelled(uint256 matchId);
}

// Functions to manage contract logic
impl MatchUpContract {
    pub fn initialize(&mut self, bet_amount: U256, username: String) {
        self.creator.set(msg::sender());
        self.bet_amount.set(bet_amount);
        self.status.set(U8::from(MatchStatus::Open as u8));
        self.challenger.set(Address::ZERO);
        self.creator_ready.set(false);
        self.challenger_ready.set(false);
        self.user_profiles.setter(msg::sender()).set_str(username);
    }

    pub fn accept_match(&mut self, username: String) {
        self.challenger.set(msg::sender());
        self.challenger_ready.set(true);
        self.user_profiles.setter(msg::sender()).set_str(username);
    }

    #[payable]
    pub fn ready_up(&mut self) {
        if (msg::value() != self.bet_amount.get()) {
            panic!("Incorrect bet amount");
        }
        if msg::sender() == self.creator.get() {
            if !self.creator_ready.get() {
                self.creator_ready.set(true);
            } else {
                panic!("Player already ready");
            }
        } else if msg::sender() == self.challenger.get() {
            if !self.challenger_ready.get() {
                self.challenger_ready.set(true);
            } else {
                panic!("Player already ready");
            }
        }
        if self.creator_ready.get() && self.challenger_ready.get() {
            self.status.set(U8::from(MatchStatus::InProgress as u8));
        }
    }

    pub fn withdraw(&mut self) {
        if self.status.get() == U8::from(MatchStatus::InProgress as u8) {
            panic!("Match is in progress");
        }
        if self.status.get() == U8::from(MatchStatus::Cancelled as u8) {
            panic!("Match is cancelled");
        }
        if msg::sender() == self.creator.get() {
            evm::transfer(msg::sender(), self.bet_amount.get()).unwrap();
        } else if msg::sender() == self.challenger.get() {
            evm::transfer(msg::sender(), self.bet_amount.get()).unwrap();
        }
    }
}
