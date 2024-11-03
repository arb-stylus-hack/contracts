#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageString, StorageU256, StorageU8, StorageVec},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MatchType {
    Individual,
    Team,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MatchStatus {
    Open,
    Ready,
    InProgress,
    Completed,
    Cancelled,
}

impl MatchType {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> Option<MatchType> {
        match value {
            0 => Some(MatchType::Individual),
            1 => Some(MatchType::Team),
            _ => None,
        }
    }
}

impl MatchStatus {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> Option<MatchStatus> {
        match value {
            0 => Some(MatchStatus::Open),
            1 => Some(MatchStatus::Ready),
            2 => Some(MatchStatus::InProgress),
            3 => Some(MatchStatus::Completed),
            4 => Some(MatchStatus::Cancelled),
            _ => None,
        }
    }
}

#[storage]
#[entrypoint]
pub struct Match {
    creator: StorageAddress,
    challenger: StorageAddress,
    bet_amount: StorageU256,
    match_type: StorageU8,
    status: StorageU8,
    creator_ready: StorageBool,
    challenger_ready: StorageBool,
    creator_team_id: StorageU256,
    challenger_team_id: StorageU256,
}

#[storage]
pub struct UserProfile {
    in_game_id: StorageString,
    email: StorageString,
}

#[storage]
pub struct Team {
    name: StorageString,
    members: StorageVec<StorageAddress>,
    owner: StorageAddress,
}

#[public]
impl Match {}
