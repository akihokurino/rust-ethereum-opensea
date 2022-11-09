// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./RustToken1155.sol";

contract RustToken1155_V2 is RustToken1155 {
    function initializeV2() public initializer {}

    function hello() public pure returns (string memory) {
        return "hello v2";
    }
}
