// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./RustToken721.sol";

contract RustToken721_V2 is RustToken721 {
    function initializeV2() public initializer {}

    function hello() public pure returns (string memory) {
        return "hello v2";
    }
}
