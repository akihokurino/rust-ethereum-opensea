// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;
import "@chainlink/contracts/src/v0.7/Operator.sol";

contract Oracle is Operator {
    /**
     * Network: Goerli
     * Link Token: 0x326C977E6efc84E512bB9C30f76E30c160eD06FB
     */
    constructor()
        Operator(0x326C977E6efc84E512bB9C30f76E30c160eD06FB, msg.sender)
    {}
}
