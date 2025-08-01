# ZK Payment System API

This document describes the REST API for the ZK payment system.

## Base URL
```
http://localhost:8081
```

## Endpoints

### 1. Get System Information
**GET** `/api/info`

Returns current system state including Merkle root, total amount, and counts.

**Response:**
```json
{
  "current_root": "0x6c65469be316a7cfb462b2861dff14f08832d6b77260812b770160f93693ad13",
  "total_amount": 22500,
  "account_count": 3,
  "transaction_count": 0
}
```

### 2. Get Accounts
**GET** `/api/accounts`

Returns all accounts in the system.

**Response:**
```json
[
  {
    "address": [17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17],
    "balance": 9900,
    "nonce": 1
  }
]
```

### 3. Create Account
**POST** `/api/accounts`

Creates a new account.

**Request Body:**
```json
{
  "address": "0x1111111111111111111111111111111111111111",
  "balance": 10000
}
```

**Response:**
```json
{
  "success": true,
  "message": "Account created successfully",
  "account": {
    "address": [17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17],
    "balance": 10000,
    "nonce": 0
  }
}
```

### 4. Get Transactions
**GET** `/api/transactions`

Returns all pending transactions.

**Response:**
```json
[]
```

### 5. Create Transaction
**POST** `/api/transactions`

Creates a new transaction.

**Request Body:**
```json
{
  "from": "0x1111111111111111111111111111111111111111",
  "to": "0x2222222222222222222222222222222222222222",
  "amount": 100
}
```

**Response:**
```json
{
  "success": true,
  "message": "Transaction created successfully",
  "transaction": {
    "from": [17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17],
    "to": [34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34],
    "amount": 100,
    "nonce": 2,
    "signature": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  }
}
```

### 6. Process Batch
**POST** `/api/batch/process`

Processes all pending transactions and updates the Merkle root.

**Response:**
```json
{
  "success": true,
  "message": "Batch processed successfully",
  "processed_count": 1,
  "new_root": "0x6c65469be316a7cfb462b2861dff14f08832d6b77260812b770160f93693ad13",
  "receipt_saved": true
}
```

### 7. Verify Receipt
**POST** `/api/receipt/verify`

Verifies the latest receipt.

**Response:**
```json
{
  "success": true,
  "message": "Receipt verified successfully",
  "processed_count": 1,
  "new_root": "0x1234567890abcdef"
}
```

## Usage Examples

### Start the API Server
```bash
cargo run -- serve --port 8081
```

### Test the API
```bash
# Get system info
curl http://localhost:8081/api/info

# Get accounts
curl http://localhost:8081/api/accounts

# Create a transaction
curl -X POST http://localhost:8081/api/transactions \
  -H "Content-Type: application/json" \
  -d '{"from":"0x1111111111111111111111111111111111111111","to":"0x2222222222222222222222222222222222222222","amount":100}'

# Process batch
curl -X POST http://localhost:8081/api/batch/process
```

## Features

- **Merkle Root Tracking**: Each operation updates and returns the current Merkle root
- **Transaction Validation**: Ensures transactions are valid before processing
- **State Persistence**: All data is saved to JSON files
- **CORS Support**: API supports cross-origin requests for web UI integration
- **Error Handling**: Comprehensive error responses for invalid requests

## Next Steps

This API is ready to be integrated with a web UI. The endpoints provide all the functionality needed for:

1. Displaying current system state
2. Creating accounts and transactions
3. Processing batches
4. Verifying receipts

The API follows RESTful conventions and returns JSON responses suitable for frontend consumption. 