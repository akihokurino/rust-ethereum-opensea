// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

library StringsLibrary {
    function addressToString(
        address _address
    ) internal pure returns (string memory) {
        bytes32 value = bytes32(uint256(uint160(_address)));
        bytes memory alphabet = "0123456789abcdef";
        bytes memory result = new bytes(42);

        result[0] = "0";
        result[1] = "x";
        for (uint256 i = 0; i < 20; i++) {
            result[2 + i * 2] = alphabet[uint8(value[i + 12] >> 4)];
            result[3 + i * 2] = alphabet[uint8(value[i + 12] & 0x0f)];
        }
        return string(result);
    }

    function uint256ToString(
        uint256 _value
    ) internal pure returns (string memory) {
        if (_value == 0) {
            return "0";
        }
        uint256 j = _value;
        uint256 length;
        while (j != 0) {
            length++;
            j /= 10;
        }
        bytes memory result = new bytes(length);
        uint256 k = length;
        while (_value != 0) {
            result[--k] = bytes1(uint8(48 + (_value % 10)));
            _value /= 10;
        }
        return string(result);
    }
}
