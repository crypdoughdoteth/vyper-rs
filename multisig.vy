# @version ^0.3.3
# Multiple Signature Contract

event Deposit:
    sender: indexed(address)
    amount: uint256
    bal: uint256
event SubmitTransaction:
    owner: indexed(address)
    txIndex: indexed(uint256)
    to: indexed(address)
    value: uint256
    data: bytes32
event ConfirmTransaction:
    owner: indexed(address)
    txIndex: indexed(uint256)
event RevokeConfirmation:
    owner: indexed(address)
    txIndex: indexed(uint256)
event ExecuteTransaction:
    owner: indexed(address)
    txIndex: indexed(uint256)

owners: DynArray[address, 10]
isOwner: HashMap[address, bool]
numConfirmationsRequired: constant(uint256) = 1


struct Transaction: 
    to: address
    val: uint256
    data: bytes32
    fnSel: String[32]
    executed: bool
    numConfirmations: uint256


isConfirmed: public(HashMap[uint256, HashMap[address, bool]])

transactions: DynArray[Transaction, 100000]


@external
def __init__ (_owners: DynArray[address,10]):
    assert len(_owners) > 0, "Error: No Owners"
    assert numConfirmationsRequired > 0 and numConfirmationsRequired <= len(_owners)
    for o in _owners:
        assert o != empty(address)
        assert self.isOwner[o] == False, "Owner Not Unique"
        self.isOwner[o] = True
        self.owners.append(o)

@external
@payable
def __default__():
    log Deposit(msg.sender, msg.value, self.balance)

@external
def submitTransaction( _to: address, _val: uint256, _data: bytes32, _fnSel: String[32]):
    assert self.isOwner[msg.sender] == True, "Not an Owner"
    txIndex: uint256 = len(self.transactions)
    self.transactions.append(Transaction({to:_to, val:_val, data: _data, fnSel: _fnSel, executed: False, numConfirmations: 0}))    

@external 
def confirmTransaction(txIndex: uint256):
    assert self.isOwner[msg.sender]==True, "Not an Owner"
    assert txIndex < len(self.transactions), "Does Not Exist"
    assert self.transactions[txIndex].executed == False, "Transaction Already Executed"
    assert self.isConfirmed[txIndex][msg.sender], "Transactions Already Confirmed"
    self.transactions[txIndex].numConfirmations += 1
    self.isConfirmed[txIndex][msg.sender] = True
    log ConfirmTransaction(msg.sender, txIndex)

@payable
@external
def executeTransaction(txIndex: uint256) -> Bytes[32]:
    assert self.isOwner[msg.sender]==True, "Not an Owner"
    assert txIndex < len(self.transactions), "Does Not Exist"
    assert self.transactions[txIndex].executed == False, "Transaction Already Executed"
    assert self.transactions[txIndex].numConfirmations >= numConfirmationsRequired, "Insufficent Confirmations"
    self.transactions[txIndex].executed = True
    success: bool = False
    response: Bytes[32] = b""
    functionSelector: Bytes[32] = convert(self.transactions[txIndex].fnSel, Bytes[32])
    success, response = raw_call(
        self.transactions[txIndex].to, 
        #add field for function selector, seperate from data
        _abi_encode(self.transactions[txIndex].data, (convert(keccak256(self.transactions[txIndex].fnSel), bytes4))),
        max_outsize=32, 
        value = self.transactions[txIndex].val,
        revert_on_failure = False
    )
    assert success
    return response

@nonpayable
@external
def revokeConfirmation(_txIndex: uint256):
    assert _txIndex < len(self.transactions), "Does Not Exist"
    assert self.transactions[_txIndex].executed == False, "Transaction Already Executed"
    self.transactions[_txIndex].numConfirmations -= 1
    self.isConfirmed[_txIndex][msg.sender] = False
    log RevokeConfirmation(msg.sender, _txIndex)

@view
@external
def getOwners() -> DynArray[address, 10]:
    return self.owners

@view
@external 
def getTransactionCount() -> uint256:
    return len(self.transactions)

@view
@external 
def getTransaction(_txIndex: uint256) -> (address, uint256, bytes32, String[32], bool, uint256):
    return(
        self.transactions[_txIndex].to,
        self.transactions[_txIndex].val,
        self.transactions[_txIndex].data,
        self.transactions[_txIndex].fnSel,
        self.transactions[_txIndex].executed,
        self.transactions[_txIndex].numConfirmations
    )
