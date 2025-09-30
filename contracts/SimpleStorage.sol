// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/**
 * @title SimpleStorage
 * @dev Optimized contract for Paseo TestNet deployment
 * Keeps bytecode minimal to fit within 100KB limit
 */
contract SimpleStorage {
    uint256 public value;
    address public owner;
    
    event ValueChanged(uint256 oldValue, uint256 newValue);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
    
    constructor() {
        owner = msg.sender;
        value = 42;
    }
    
    function setValue(uint256 _value) external {
        uint256 oldValue = value;
        value = _value;
        emit ValueChanged(oldValue, _value);
    }
    
    function getValue() external view returns (uint256) {
        return value;
    }
    
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}