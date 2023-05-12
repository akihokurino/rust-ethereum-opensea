// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract MultiSigWallet {
    using EnumerableSet for EnumerableSet.AddressSet;
    using SafeMath for uint256;

    EnumerableSet.AddressSet private _owners;
    uint256 private _required;

    struct Transaction {
        address destination;
        uint256 value;
        bytes data;
        bool executed;
    }

    mapping(uint256 => Transaction) public transactions;
    mapping(uint256 => mapping(address => bool)) public confirmations;
    uint256 public transactionCount;

    event OwnerAdded(address indexed owner);
    event OwnerRemoved(address indexed owner);
    event RequirementChanged(uint256 required);
    event Confirmation(address indexed sender, uint256 indexed transactionId);
    event Execution(uint256 indexed transactionId);
    event ExecutionFailure(uint256 indexed transactionId);
    event Submission(uint256 indexed transactionId);

    modifier onlyOwner() {
        require(_owners.contains(msg.sender), "not an owner");
        _;
    }

    modifier transactionExists(uint256 transactionId) {
        require(
            transactions[transactionId].destination != address(0),
            "transaction does not exist"
        );
        _;
    }

    modifier notConfirmed(uint256 transactionId) {
        require(
            !confirmations[transactionId][msg.sender],
            "transaction already confirmed"
        );
        _;
    }

    constructor(address[] memory initialOwners, uint256 requiredConfirmations) {
        require(
            initialOwners.length >= requiredConfirmations &&
                requiredConfirmations > 0,
            "invalid number of required confirmations"
        );
        for (uint256 i = 0; i < initialOwners.length; i++) {
            _owners.add(initialOwners[i]);
            emit OwnerAdded(initialOwners[i]);
        }
        _required = requiredConfirmations;
        emit RequirementChanged(_required);
    }

    function getOwners() public view returns (address[] memory) {
        address[] memory ownersList = new address[](_owners.length());
        for (uint256 i = 0; i < _owners.length(); i++) {
            ownersList[i] = _owners.at(i);
        }
        return ownersList;
    }

    function getTransaction(
        uint256 transactionId
    )
        public
        view
        returns (
            address destination,
            uint256 value,
            bytes memory data,
            bool executed
        )
    {
        Transaction storage txn = transactions[transactionId];
        return (txn.destination, txn.value, txn.data, txn.executed);
    }

    function getRequired() public view returns (uint256) {
        return _required;
    }

    function addOwner(address owner) public onlyOwner {
        require(!_owners.contains(owner), "owner already exists");
        _owners.add(owner);
        emit OwnerAdded(owner);
    }

    function removeOwner(address owner) public onlyOwner {
        require(_owners.contains(owner), "not an owner");
        _owners.remove(owner);
        emit OwnerRemoved(owner);
    }

    function changeRequirement(uint256 requiredConfirmations) public onlyOwner {
        require(
            requiredConfirmations > 0 &&
                requiredConfirmations <= _owners.length(),
            "invalid number of required confirmations"
        );
        _required = requiredConfirmations;
        emit RequirementChanged(_required);
    }

    function submitTransaction(
        address destination,
        uint256 value,
        bytes memory data
    ) public onlyOwner {
        uint256 transactionId = addTransaction(destination, value, data);
        confirmTransaction(transactionId);
    }

    function confirmTransaction(
        uint256 transactionId
    )
        public
        onlyOwner
        transactionExists(transactionId)
        notConfirmed(transactionId)
    {
        confirmations[transactionId][msg.sender] = true;
        emit Confirmation(msg.sender, transactionId);
        executeTransaction(transactionId);
    }

    function addTransaction(
        address destination,
        uint256 value,
        bytes memory data
    ) private returns (uint256 transactionId) {
        transactionId = transactionCount;
        transactions[transactionId] = Transaction({
            destination: destination,
            value: value,
            data: data,
            executed: false
        });
        transactionCount += 1;
        emit Submission(transactionId);
    }

    function executeTransaction(
        uint256 transactionId
    ) private onlyOwner transactionExists(transactionId) {
        if (isConfirmed(transactionId)) {
            Transaction storage txn = transactions[transactionId];
            txn.executed = true;
            (bool success, ) = txn.destination.call{value: txn.value}(txn.data);
            require(success, "failed execute transaction");
            emit Execution(transactionId);
        }
    }

    function isConfirmed(uint256 transactionId) private view returns (bool) {
        uint256 count = 0;
        for (uint256 i = 0; i < _owners.length(); i++) {
            if (confirmations[transactionId][_owners.at(i)]) {
                count += 1;
                if (count == _required) {
                    return true;
                }
            }
        }
        return false;
    }

    receive() external payable {}
}
