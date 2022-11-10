// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./RustToken1155.sol";

contract RustToken1155_V2 is RustToken1155 {
    string private _version;

    function initializeV2(string memory versionSuffix) public {
        _version = string(abi.encodePacked("v2 ", versionSuffix));
    }

    function hello() public view virtual returns (string memory) {
        return string(abi.encodePacked("hello ", _version));
    }
}
