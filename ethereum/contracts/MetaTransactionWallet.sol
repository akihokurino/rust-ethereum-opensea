// SPDX-License-Identifier: MIT
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/metatx/MinimalForwarder.sol
// https://github.com/dea-sg/meta-tx/blob/main/contracts/metatx/ForwarderUpgradeable.sol
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/metatx/ERC2771Context.sol";
import "hardhat/console.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/draft-EIP712Upgradeable.sol";

contract MetaTransactionWallet is
    Initializable,
    OwnableUpgradeable,
    EIP712Upgradeable
{
    using ECDSAUpgradeable for bytes32;

    struct ForwardRequest {
        address from;
        address to;
        uint256 value;
        uint256 gas;
        uint256 nonce;
        bytes data;
    }

    bytes32 private constant _TYPEHASH =
        keccak256(
            "ForwardRequest(address from,address to,uint256 value,uint256 gas,uint256 nonce,bytes data)"
        );

    mapping(address => uint256) private nonces;
    address private _relayer;

    modifier onlyRelayer() {
        require(msg.sender == _relayer, "not an relayer");
        _;
    }

    function initialize(address relayer) public initializer {
        _relayer = relayer;
        __Ownable_init();
        __EIP712_init_unchained("MetaTransactionWallet", "0.0.1");
    }

    function execute(
        ForwardRequest calldata req,
        bytes calldata signature
    ) public onlyRelayer returns (bool success, bytes memory returndata) {
        require(verify(req, signature), "signature does not match request");
        nonces[req.from] = req.nonce + 1;

        (success, returndata) = req.to.call{gas: req.gas, value: req.value}(
            abi.encodePacked(req.data, req.from)
        );
        require(success, "call error");

        // Validate that the relayer has sent enough gas for the call.
        // See https://ronan.eth.link/blog/ethereum-gas-dangers/
        if (gasleft() <= req.gas / 63) {
            // We explicitly trigger invalid opcode to consume all gas and bubble-up the effects, since
            // neither revert or assert consume all gas since Solidity 0.8.0
            // https://docs.soliditylang.org/en/v0.8.0/control-structures.html#panic-via-assert-and-error-via-require
            // solhint-disable-next-line no-inline-assembly
            assembly {
                invalid()
            }
        }

        return (success, returndata);
    }

    function getNonce(address from) external view returns (uint256) {
        return nonces[from];
    }

    function verify(
        ForwardRequest calldata req,
        bytes calldata signature
    ) private view returns (bool) {
        address signer = _hashTypedDataV4(
            keccak256(
                abi.encode(
                    _TYPEHASH,
                    req.from,
                    req.to,
                    req.value,
                    req.gas,
                    req.nonce,
                    keccak256(req.data)
                )
            )
        ).recover(signature);

        require(nonces[req.from] == req.nonce, "illegal nonce");
        require(signer == req.from, "illegal signer");
        return true;
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
