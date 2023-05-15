// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/metatx/ERC2771Context.sol";
import "hardhat/console.sol";

contract MetaTransactionWallet is Initializable, OwnableUpgradeable {
    using ECDSA for bytes32;

    address private _owner;
    address private _relayer;

    modifier onlyRelayer() {
        require(msg.sender == _relayer, "not an relayer");
        _;
    }

    function initialize(address owner, address relayer) public initializer {
        _owner = owner;
        _relayer = relayer;
        __Ownable_init();
    }

    function executeMetaTransaction(
        uint256 nonce,
        address destination,
        uint256 value,
        bytes memory data,
        bytes memory signature
    ) public onlyRelayer {
        bytes32 hash = keccak256(
            abi.encodePacked(nonce, destination, value, data)
        );
        bytes32 signedHash = ECDSA.toEthSignedMessageHash(hash);

        require(
            _owner == ECDSA.recover(signedHash, signature),
            "invalid signature"
        );

        (bool success, ) = destination.call{value: value}(data);
        require(success, "transaction failed");
    }

    receive() external payable {}
}

contract MetaTransactionalNft721 is ERC721Enumerable, ERC2771Context, Ownable {
    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;

    constructor(
        address relay
    ) ERC2771Context(relay) ERC721("MetaTransactionalNft", "MTN") {
        _localTokenId = 0;
    }

    modifier onlyRelayer() {
        require(isTrustedForwarder(msg.sender), "not an relayer");
        _;
    }

    function mint(
        address to,
        string memory contentHash
    ) public virtual onlyRelayer {
        console.log(_msgSender());
        _localTokenId += 1;
        _token2hash[_localTokenId] = contentHash;
        _mint(to, _localTokenId);
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        string memory contentHash = _token2hash[tokenId];
        return string(abi.encodePacked("ipfs://", contentHash));
    }

    function _msgSender()
        internal
        view
        override(Context, ERC2771Context)
        returns (address)
    {
        return ERC2771Context._msgSender();
    }

    function _msgData()
        internal
        view
        override(Context, ERC2771Context)
        returns (bytes calldata)
    {
        return ERC2771Context._msgData();
    }
}
