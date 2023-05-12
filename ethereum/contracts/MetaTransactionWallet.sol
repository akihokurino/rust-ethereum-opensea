// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract MetaTransactionWallet {
    using ECDSA for bytes32;

    address public owner;

    constructor(address _owner) {
        owner = _owner;
    }

    function executeMetaTransaction(
        uint256 nonce,
        address destination,
        uint256 value,
        bytes memory data,
        bytes memory signature
    ) public {
        bytes32 hash = keccak256(
            abi.encodePacked(nonce, destination, value, data)
        );
        bytes32 signedHash = ECDSA.toEthSignedMessageHash(hash);

        require(
            owner == ECDSA.recover(signedHash, signature),
            "invalid signature"
        );

        (bool success, ) = destination.call{value: value}(data);
        require(success, "transaction failed");
    }

    receive() external payable {}
}
