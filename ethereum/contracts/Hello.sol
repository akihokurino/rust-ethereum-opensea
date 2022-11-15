// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Hello is Ownable {
    string public message;

    constructor() {
        message = "hello";
    }

    function setMessage(string memory _message) public virtual onlyOwner {
        message = string(abi.encodePacked("hello ", _message));
    }
}
