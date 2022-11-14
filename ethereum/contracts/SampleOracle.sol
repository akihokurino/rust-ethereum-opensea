// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SampleOracle is ChainlinkClient, Ownable {
    AggregatorV3Interface internal priceFeed;

    using Chainlink for Chainlink.Request;

    struct TimeResponse {
        string now;
        int256 timestamp;
    }
    TimeResponse public timeResponse;

    uint256 private fee;
    address private oracleAddress;
    bytes32 private getTimeJobId;

    /**
     * Network: Goerli
     * Aggregator: ETH/USD
     * Address: 0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
     *
     * Link Token: 0x326C977E6efc84E512bB9C30f76E30c160eD06FB
     */
    constructor() {
        priceFeed = AggregatorV3Interface(
            0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
        );

        fee = 1 * 10**18;
        oracleAddress = 0x326C977E6efc84E512bB9C30f76E30c160eD06FB; // TODO: Oracleのアドレスに変える
        getTimeJobId = ""; // TODO: 実際のJobIDに変える
        setChainlinkToken(0x326C977E6efc84E512bB9C30f76E30c160eD06FB);
    }

    function getChainlinkToken() public view returns (address) {
        return chainlinkTokenAddress();
    }

    // https://docs.chain.link/docs/consuming-data-feeds/
    function getLatestPrice() public view returns (int256) {
        (, int256 price, , , ) = priceFeed.latestRoundData();
        return price;
    }

    function createGetTimeRequestTo()
        public
        onlyOwner
        returns (bytes32 requestId)
    {
        Chainlink.Request memory req = buildChainlinkRequest(
            getTimeJobId,
            address(this),
            this.fulfillGetTimeRequest.selector
        );
        req.add("params", "sample time adapter");
        requestId = sendChainlinkRequestTo(oracleAddress, req, fee);
    }

    function fulfillGetTimeRequest(
        bytes32 requestId,
        string memory _now,
        int256 _timestamp
    ) public recordChainlinkFulfillment(requestId) {
        timeResponse = TimeResponse({now: _now, timestamp: _timestamp});
    }

    function cancelRequest(
        bytes32 requestId,
        bytes4 callbackFunctionId,
        uint256 expiration
    ) public onlyOwner {
        cancelChainlinkRequest(requestId, fee, callbackFunctionId, expiration);
    }

    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
        require(
            link.transfer(msg.sender, link.balanceOf(address(this))),
            "Unable to transfer"
        );
    }
}
