// SPDX-License-Identifier: MIT
pragma solidity 0.8.7;

contract Flashloan {
    uint256 public value;

    function setNumber (uint256 _value) public {
        value = _value;
    }

    function getNumber () public view returns (uint256) {
        return value;
    }
}

