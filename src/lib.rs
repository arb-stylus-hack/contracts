#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageString, StorageU256, StorageU8, StorageVec},
};

sol! {
    event UserProfileCreated(address indexed user, string riotId);
    event TeamCreated(uint256 indexed teamId, string name, address indexed owner);
    event TeamMemberAdded(uint256 indexed teamId, address indexed member);
    event TeamMemberRemoved(uint256 indexed teamId, address indexed member);
    event MatchCreated(uint256 indexed matchId, address indexed creator, uint256 betAmount, uint8 matchType);
    event MatchJoined(uint256 indexed matchId, address indexed challenger);
    event PlayerReady(uint256 indexed matchId, address indexed player);
    event MatchStarted(uint256 indexed matchId);
    event MatchCompleted(uint256 indexed matchId, address indexed winner);
    event MatchCancelled(uint256 indexed matchId);

    error ProfileAlreadyExists();
    error ProfileDoesNotExist();
    error InvalidTeamId();
    error NotTeamOwner();
    error InvalidMatchId();
    error NotParticipant();
    error MatchNotOpen();
    error MatchAlreadyHasChallenger();
    error MatchNotReady();
    error IncorrectBetAmount();
    error AlreadyReady();
    error MatchNotInProgress();
    error InvalidWinner();
    error OnlyCreatorCanCancel();
    error CanOnlyCancelOpenOrReadyMatches();
}

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
