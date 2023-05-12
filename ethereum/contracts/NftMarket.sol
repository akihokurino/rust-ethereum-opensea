// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "hardhat/console.sol";
import "./Strings.sol";

contract NftMarket is Initializable, OwnableUpgradeable {
    using SafeMath for uint256;
    using StringsLibrary for address;
    using StringsLibrary for uint256;

    struct NFT {
        address contractAddress;
        uint256 tokenId;
        address payable seller;
        uint256 price;
    }
    struct NFTWithURI {
        address contractAddress;
        uint256 tokenId;
        address payable seller;
        uint256 price;
        string tokenURI;
    }

    event SellOrder(
        address contractAddress,
        uint256 tokenId,
        address seller,
        uint256 price
    );
    event CancelOrder(address contractAddress, uint256 tokenId, address seller);
    event BuyOrder(
        address contractAddress,
        uint256 tokenId,
        address buyer,
        uint256 price
    );

    mapping(string => NFT) private sellOrders;
    string[] public sellOrderKeys;

    function initialize() public initializer {
        __Ownable_init();
    }

    function sellOrderKey(
        address contractAddress,
        uint256 tokenId
    ) private pure returns (string memory) {
        return
            string(
                abi.encodePacked(
                    contractAddress.addressToString(),
                    "#",
                    tokenId.uint256ToString()
                )
            );
    }

    function sellOrderKeyIndex(
        string memory key
    ) private view returns (int256) {
        for (uint256 i = 0; i < sellOrderKeys.length; i++) {
            if (
                keccak256(abi.encodePacked(sellOrderKeys[i])) ==
                keccak256(abi.encodePacked(key))
            ) {
                return int256(i);
            }
        }
        return -1;
    }

    function removeSellOrderKey(uint256 i) private {
        require(i < sellOrderKeys.length, "index out of bounds");
        sellOrderKeys[i] = sellOrderKeys[sellOrderKeys.length - 1];
        sellOrderKeys.pop();
    }

    function getSellOrderKeys() public view returns (string[] memory) {
        return sellOrderKeys;
    }

    function getAllSellOrders() public view returns (NFTWithURI[] memory) {
        NFTWithURI[] memory nftsWithURIs = new NFTWithURI[](
            sellOrderKeys.length
        );

        for (uint256 i = 0; i < sellOrderKeys.length; i++) {
            string memory key = sellOrderKeys[i];
            NFT memory order = sellOrders[key];

            string memory uri = IERC721Metadata(order.contractAddress).tokenURI(
                order.tokenId
            );
            nftsWithURIs[i] = NFTWithURI(
                order.contractAddress,
                order.tokenId,
                order.seller,
                order.price,
                uri
            );
        }

        return nftsWithURIs;
    }

    function getSellOrder(
        address contractAddress,
        uint256 tokenId
    ) public view returns (NFTWithURI memory) {
        string memory key = sellOrderKey(contractAddress, tokenId);
        int256 keyIndex = sellOrderKeyIndex(key);

        require(keyIndex >= 0, "sell order does not exist");

        NFT memory order = sellOrders[key];

        string memory uri = IERC721Metadata(order.contractAddress).tokenURI(
            order.tokenId
        );
        return
            NFTWithURI(
                order.contractAddress,
                order.tokenId,
                order.seller,
                order.price,
                uri
            );
    }

    function sellOrder(
        address contractAddress,
        uint256 tokenId,
        uint256 price
    ) public {
        require(
            IERC721(contractAddress).ownerOf(tokenId) == msg.sender,
            "token not owned"
        );

        string memory key = sellOrderKey(contractAddress, tokenId);
        int256 keyIndex = sellOrderKeyIndex(key);

        require(keyIndex == -1, "already sell ordered");
        require(price > 0, "price must be greater than 0");

        address payable seller = payable(msg.sender);
        NFT memory order = NFT(contractAddress, tokenId, seller, price);

        sellOrders[key] = order;
        sellOrderKeys.push(key);

        emit SellOrder(contractAddress, tokenId, seller, price);
    }

    function cancelOrder(address contractAddress, uint256 tokenId) public {
        require(
            IERC721(contractAddress).ownerOf(tokenId) == msg.sender,
            "token not owned"
        );

        string memory key = sellOrderKey(contractAddress, tokenId);
        int256 keyIndex = sellOrderKeyIndex(key);

        require(keyIndex >= 0, "sell order does not exist");

        NFT storage order = sellOrders[key];
        require(order.seller == msg.sender, "you not owned sell order");

        emit CancelOrder(contractAddress, tokenId, msg.sender);

        order.contractAddress = address(0);
        order.tokenId = 0;
        order.seller = payable(address(0));
        order.price = 0;
        removeSellOrderKey(uint256(keyIndex));
    }

    function cancelOrderByAdmin(
        address contractAddress,
        uint256 tokenId
    ) public onlyOwner {
        string memory key = sellOrderKey(contractAddress, tokenId);
        int256 keyIndex = sellOrderKeyIndex(key);

        require(keyIndex >= 0, "sell order does not exist");

        NFT storage order = sellOrders[key];

        emit CancelOrder(contractAddress, tokenId, msg.sender);

        order.contractAddress = address(0);
        order.tokenId = 0;
        order.seller = payable(address(0));
        order.price = 0;
        removeSellOrderKey(uint256(keyIndex));
    }

    function buyOrder(address contractAddress, uint256 tokenId) public payable {
        require(
            IERC721(contractAddress).ownerOf(tokenId) != msg.sender,
            "token owned"
        );

        string memory key = sellOrderKey(contractAddress, tokenId);
        int256 keyIndex = sellOrderKeyIndex(key);

        require(keyIndex >= 0, "sell order does not exist");

        NFT storage order = sellOrders[key];
        require(order.seller != msg.sender, "you are seller");
        require(msg.value >= order.price, "not enough funds to purchase NFT");
        require(
            IERC721(contractAddress).ownerOf(tokenId) == order.seller,
            "seller is not owned"
        );

        address payable seller = order.seller;
        seller.transfer(msg.value);

        IERC721(order.contractAddress).transferFrom(
            seller,
            msg.sender,
            tokenId
        );

        emit BuyOrder(
            order.contractAddress,
            order.tokenId,
            msg.sender,
            order.price
        );

        order.contractAddress = address(0);
        order.tokenId = 0;
        order.seller = payable(address(0));
        order.price = 0;
        removeSellOrderKey(uint256(keyIndex));
    }
}
