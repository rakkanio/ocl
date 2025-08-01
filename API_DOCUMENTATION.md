# Zklear API Documentation

## Overview

Zklear is a zero-knowledge payment system with Merkle tree state management. The API provides endpoints for account management, transaction processing, and batch operations with zero-knowledge proofs.

## Quick Start

1. **Start the API server:**
   ```bash
   cargo run -- serve --port 8081
   ```

2. **View the documentation:**
   - Open `index.html` in your browser for the landing page
   - Open `api-docs.html` in your browser for detailed API documentation

## API Documentation

The API documentation is provided through a clean, minimalistic HTML interface instead of Swagger. This approach offers:

- **Clean Design**: Modern, responsive design with clear typography
- **Comprehensive Coverage**: All endpoints with detailed descriptions
- **Example Requests/Responses**: Real JSON examples for each endpoint
- **No Dependencies**: Pure HTML/CSS - no external dependencies
- **Easy to Customize**: Simple to modify and extend

## Available Documentation

### Landing Page (`index.html`)
- Clean introduction to the Zklear system
- Quick overview of features
- Link to detailed API documentation

### API Documentation (`api-docs.html`)
- Complete endpoint reference
- Request/response examples
- Parameter descriptions
- Status codes
- System features overview

## API Endpoints

### System Information
- `GET /api/info` - Get system statistics and current state

### Account Management
- `GET /api/accounts` - List all accounts
- `POST /api/accounts` - Create account with specific address
- `POST /api/accounts/create` - Create account with random address

### Transaction Operations
- `GET /api/transactions` - List all transactions
- `POST /api/transactions` - Create new transaction

### Batch Processing
- `POST /api/batch/process` - Process transactions with ZK proofs
- `POST /api/receipt/verify` - Verify ZK proof receipt

## Features

- **Zero-Knowledge Proofs**: All transactions processed with cryptographic proofs
- **Merkle Tree State**: Efficient state management with verifiable roots
- **Batch Processing**: Process multiple transactions efficiently
- **Real-time Statistics**: Comprehensive system metrics
- **RESTful API**: Clean, standard REST interface

## Why Not Swagger?

We chose a custom HTML documentation approach over Swagger for several reasons:

1. **Simplicity**: No complex dependencies or build requirements
2. **Customization**: Full control over design and content
3. **Performance**: Lightweight and fast loading
4. **Maintainability**: Easy to update and modify
5. **Reliability**: No external service dependencies

## Development

To modify the documentation:

1. Edit `api-docs.html` for API endpoint documentation
2. Edit `index.html` for the landing page
3. The documentation uses pure HTML/CSS for maximum compatibility

## Testing

Test the API endpoints using curl or any HTTP client:

```bash
# Get system info
curl http://localhost:8081/api/info

# Create an account
curl -X POST http://localhost:8081/api/accounts/create \
  -H "Content-Type: application/json" \
  -d '{"balance": 10000}'

# Create a transaction
curl -X POST http://localhost:8081/api/transactions \
  -H "Content-Type: application/json" \
  -d '{"from": "0x1234...", "to": "0x5678...", "amount": 1000}'
```

The API server runs on `http://localhost:8081` by default. 