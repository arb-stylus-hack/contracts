// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MatchUpContract {
    enum MatchType { Individual, Team }
    enum MatchStatus { Open, Ready, InProgress, Completed, Cancelled }

    struct UserProfile {
        string riotId;
    }

    struct Match {
        address creator;
        address challenger;
        uint256 betAmount;
        MatchStatus status;
        bool creatorReady;
        bool challengerReady;
    }

    mapping(address => UserProfile) public userProfiles;
    mapping(uint256 => Match) public matches;
    uint256 public matchCount;
    address public platformWallet;
    uint256 public constant PLATFORM_FEE = 2; // 2% fee

    event UserProfileCreated(address user, string riotId);
    event MatchCreated(uint256 matchId, address creator, uint256 betAmount, MatchType matchType);
    event MatchJoined(uint256 matchId, address challenger);
    event PlayerReady(uint256 matchId, address player);
    event MatchStarted(uint256 matchId);
    event MatchCompleted(uint256 matchId, address winner);
    event MatchCancelled(uint256 matchId);

    constructor(address _platformWallet) {
        platformWallet = _platformWallet;
    }

    modifier onlyParticipant(uint256 _matchId) {
        require(msg.sender == matches[_matchId].creator || msg.sender == matches[_matchId].challenger, "Not a participant");
        _;
    }

    function createUserProfile(string memory _riotId) external {
        require(bytes(userProfiles[msg.sender].riotId).length == 0, "Profile already exists");
        userProfiles[msg.sender] = UserProfile(_riotId);
        emit UserProfileCreated(msg.sender, _riotId);
    }

    function createMatch(MatchType _matchType, uint256 _betAmount) external {
        require(_betAmount > 0, "Bet amount must be greater than 0");
        require(bytes(userProfiles[msg.sender].riotId).length > 0, "Create a user profile first");

        matchCount++;
        matches[matchCount] = Match({
            creator: msg.sender,
            challenger: address(0),
            betAmount: _betAmount,
            status: MatchStatus.Open,
            creatorReady: false,
            challengerReady: false
        });

        emit MatchCreated(matchCount, msg.sender, _betAmount, _matchType);
    }

    function joinMatch(uint256 _matchId) external {
        Match storage game = matches[_matchId];
        require(game.status == MatchStatus.Open, "Match is not open");
        require(game.challenger == address(0), "Match already has a challenger");
        require(bytes(userProfiles[msg.sender].riotId).length > 0, "Create a user profile first");

        game.challenger = msg.sender;
        game.status = MatchStatus.Ready;

        emit MatchJoined(_matchId, msg.sender);
    }

    function readyUp(uint256 _matchId) external payable onlyParticipant(_matchId) {
        Match storage game = matches[_matchId];
        require(game.status == MatchStatus.Ready, "Match is not ready");
        require(msg.value == game.betAmount, "Incorrect bet amount");

        if (msg.sender == game.creator) {
            require(!game.creatorReady, "Creator already ready");
            game.creatorReady = true;
        } else {
            require(!game.challengerReady, "Challenger already ready");
            game.challengerReady = true;
        }

        emit PlayerReady(_matchId, msg.sender);

        if (game.creatorReady && game.challengerReady) {
            game.status = MatchStatus.InProgress;
            emit MatchStarted(_matchId);
        }
    }

    function declareWinner(uint256 _matchId, address _winner) external onlyParticipant(_matchId) {
        Match storage game = matches[_matchId];
        require(game.status == MatchStatus.InProgress, "Match is not in progress");
        require(_winner == game.creator || _winner == game.challenger, "Invalid winner");

        game.status = MatchStatus.Completed;

        uint256 totalPot = game.betAmount * 2;
        uint256 platformFee = (totalPot * PLATFORM_FEE) / 100;
        uint256 winnerPrize = totalPot - platformFee;

        payable(platformWallet).transfer(platformFee);
        payable(_winner).transfer(winnerPrize);

        emit MatchCompleted(_matchId, _winner);
    }

    function cancelMatch(uint256 _matchId) external {
        Match storage game = matches[_matchId];
        require(msg.sender == game.creator, "Only creator can cancel");
        require(game.status == MatchStatus.Open || game.status == MatchStatus.Ready, "Can only cancel open or ready matches");

        game.status = MatchStatus.Cancelled;

        if (game.creatorReady) {
            payable(game.creator).transfer(game.betAmount);
        }
        if (game.challengerReady) {
            payable(game.challenger).transfer(game.betAmount);
        }

        emit MatchCancelled(_matchId);
    }

    function getMatch(uint256 _matchId) external view returns (Match memory) {
        return matches[_matchId];
    }

    function getUserProfile(address _user) external view returns (UserProfile memory) {
        return userProfiles[_user];
    }
}
