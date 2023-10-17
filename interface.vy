
# External Interfaces
interface Multisig:
    def __default__(): payable
    def submitTransaction(_to: address, _val: uint256, _data: bytes32, _fnSel: String[32]): nonpayable
    def confirmTransaction(txIndex: uint256): nonpayable
    def executeTransaction(txIndex: uint256) -> Bytes[32]: payable
    def revokeConfirmation(_txIndex: uint256): nonpayable
    def getOwners() -> DynArray[address, 10]: view
    def getTransactionCount() -> uint256: view
    def getTransaction(_txIndex: uint256) -> (address, uint256, bytes32, String[32], bool, uint256): view
    def isConfirmed(arg0: uint256, arg1: address) -> bool: view

