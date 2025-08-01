import React from 'react';

const ApiSpec: React.FC = () => {
  return (
    <div>
      <h1 style={{ marginBottom: '2rem', color: '#2d3748', fontSize: '2rem', fontWeight: '700' }}>API Documentation</h1>
      
      {/* Introduction */}
      <div className="card">
        <h2 className="card-title">Introduction</h2>
        <p style={{ marginBottom: '1.5rem', lineHeight: '1.6' }}>
          The Zklear API provides a RESTful interface for interacting with a zero-knowledge payment system built on RISC Zero. 
          This system enables secure, privacy-preserving transactions with cryptographic guarantees of correctness and integrity.
        </p>
        <p style={{ marginBottom: '1.5rem', lineHeight: '1.6' }}>
          <strong>Key Features:</strong>
        </p>
        <ul style={{ marginBottom: '1.5rem', paddingLeft: '2rem', lineHeight: '1.6' }}>
          <li><strong>Zero-Knowledge Proofs:</strong> All state transitions are verified cryptographically without revealing sensitive data</li>
          <li><strong>Merkle Tree State Management:</strong> Efficient, verifiable state commitments using cryptographic hashing</li>
          <li><strong>Batch Processing:</strong> Multiple transactions processed atomically with single proof generation</li>
          <li><strong>Privacy-Preserving:</strong> Transaction details remain confidential while maintaining verifiability</li>
        </ul>
        <p style={{ lineHeight: '1.6' }}>
          The API follows REST conventions and returns JSON responses. All endpoints support standard HTTP methods and 
          include comprehensive error handling with appropriate HTTP status codes.
        </p>
      </div>

      {/* Table of Contents */}
      <div className="card">
        <h2 className="card-title">Table of Contents</h2>
        <ol style={{ paddingLeft: '1.5rem', margin: 0, lineHeight: '1.8' }}>
          <li style={{ marginBottom: '1rem' }}>
            <strong style={{ color: '#2d3748' }}>System Operations</strong>
            <ol style={{ paddingLeft: '1.5rem', marginTop: '0.5rem' }}>
              <li>
                <a href="#system-info" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  System Information
                </a>
              </li>
            </ol>
          </li>
          <li style={{ marginBottom: '1rem' }}>
            <strong style={{ color: '#2d3748' }}>Account Management</strong>
            <ol style={{ paddingLeft: '1.5rem', marginTop: '0.5rem' }}>
              <li>
                <a href="#get-accounts" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Get All Accounts
                </a>
              </li>
              <li>
                <a href="#create-account" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Create Account (Specific Address)
                </a>
              </li>
              <li>
                <a href="#create-random-account" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Create Account (Random Address)
                </a>
              </li>
            </ol>
          </li>
          <li style={{ marginBottom: '1rem' }}>
            <strong style={{ color: '#2d3748' }}>Transaction Operations</strong>
            <ol style={{ paddingLeft: '1.5rem', marginTop: '0.5rem' }}>
              <li>
                <a href="#get-transactions" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Get All Transactions
                </a>
              </li>
              <li>
                <a href="#create-transaction" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Create Transaction
                </a>
              </li>
            </ol>
          </li>
          <li style={{ marginBottom: '1rem' }}>
            <strong style={{ color: '#2d3748' }}>Batch Processing</strong>
            <ol style={{ paddingLeft: '1.5rem', marginTop: '0.5rem' }}>
              <li>
                <a href="#process-batch" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Process Batch
                </a>
              </li>
              <li>
                <a href="#verify-receipt" style={{ color: '#4a5568', textDecoration: 'none' }}>
                  Verify Receipt
                </a>
              </li>
            </ol>
          </li>
        </ol>
      </div>
      
      <div className="card">
        <h2 className="card-title">Base URL</h2>
        <div style={{ 
          fontFamily: 'monospace', 
          background: '#2d3748', 
          color: '#e2e8f0', 
          padding: '1rem', 
          borderRadius: '8px',
          fontSize: '1.1rem'
        }}>
          http://localhost:8081
        </div>
      </div>

      {/* System Information */}
      <div className="card" id="system-info">
        <h2 className="card-title">System Information</h2>
        <div style={{ marginBottom: '1rem' }}>
          <span className="badge badge-success">GET</span>
          <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/info</span>
        </div>
        <p>
          This endpoint provides a comprehensive overview of the entire Zklear system state. It returns the current Merkle root 
          (a cryptographic commitment to all account states), total aggregate balance across all accounts, and detailed 
          system statistics. The Merkle root serves as a zero-knowledge proof that all account states are consistent and 
          can be verified without revealing individual account details. This is essential for maintaining the integrity of 
          the payment system while preserving privacy.
        </p>
        
        <h3 style={{ marginTop: '1rem', marginBottom: '0.5rem' }}>Response Example:</h3>
        <pre style={{ 
          background: '#2d3748', 
          color: '#e2e8f0', 
          padding: '1rem', 
          borderRadius: '4px',
          overflow: 'auto',
          fontSize: '0.9rem'
        }}>
{`{
  "current_root": "0x1aa9a81b18e4b6c74aa07f8c8907c537b343e206faa42a50beb9b632ce3049c0",
  "total_amount": 62500,
  "account_count": 7,
  "transaction_count": 1,
  "system_stats": {
    "total_accounts_created": 7,
    "total_transactions_processed": 0,
    "average_balance": 8928.57,
    "highest_balance": 15000,
    "lowest_balance": 5000
  }
}`}
        </pre>
      </div>

      {/* Account Management */}
      <div className="card" id="account-management">
        <h2 className="card-title">Account Management</h2>
        
        <div style={{ marginBottom: '2rem' }} id="get-accounts">
          <h3>Get All Accounts</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-success">GET</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/accounts</span>
          </div>
          <p>
            Retrieves the complete list of all accounts in the Zklear system. Each account contains a unique 20-byte address 
            (displayed in hexadecimal format), current balance, and nonce (transaction counter). The nonce ensures that 
            transactions are processed in the correct order and prevents replay attacks. This endpoint is useful for 
            displaying account information in user interfaces and for auditing purposes.
          </p>
        </div>

        <div style={{ marginBottom: '2rem' }} id="create-account">
          <h3>Create Account with Specific Address</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-warning">POST</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/accounts</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Creates a new account with a user-specified address and initial balance. The address must be a valid 20-byte 
            hexadecimal string (40 characters). This endpoint is useful for creating accounts with predetermined addresses, 
            such as when migrating from other systems or when specific addresses are required for compatibility reasons. 
            The account will be initialized with a nonce of 0, ready to process its first transaction.
          </p>
          
          <h4 style={{ marginBottom: '0.75rem' }}>Request Body:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`{
  "address": "0x1234567890abcdef1234567890abcdef12345678",
  "balance": 10000
}`}
          </pre>
        </div>

        <div style={{ marginBottom: '2rem' }} id="create-random-account">
          <h3>Create Account with Random Address</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-warning">POST</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/accounts/create</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Creates a new account with a cryptographically secure randomly generated address and specified initial balance. 
            This is the recommended method for creating new accounts as it ensures address uniqueness and security. 
            The generated address follows the same format as Ethereum addresses (20 bytes, hexadecimal). This endpoint 
            is ideal for user onboarding flows where specific addresses are not required.
          </p>
          
          <h4 style={{ marginBottom: '0.75rem' }}>Request Body:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`{
  "balance": 10000
}`}
          </pre>
        </div>
      </div>

      {/* Transaction Operations */}
      <div className="card" id="transaction-operations">
        <h2 className="card-title">Transaction Operations</h2>
        
        <div style={{ marginBottom: '2rem' }} id="get-transactions">
          <h3>Get All Transactions</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-success">GET</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/transactions</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Retrieves all pending transactions in the Zklear system. These are transactions that have been created but 
            not yet processed in a batch. Each transaction includes the sender and recipient addresses, transfer amount, 
            and nonce. This endpoint is useful for displaying transaction history, monitoring pending transfers, and 
            auditing system activity.
          </p>
        </div>

        <div style={{ marginBottom: '2rem' }} id="create-transaction">
          <h3>Create Transaction</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-warning">POST</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/transactions</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Creates a new transaction to transfer tokens from one account to another. The transaction includes sender and 
            recipient addresses, transfer amount, and automatically assigned nonce. The nonce is calculated as the sender's 
            current nonce plus 1, ensuring proper transaction ordering. This endpoint validates that the sender has 
            sufficient balance and that the addresses are valid before creating the transaction. The transaction remains 
            pending until processed in a batch with zero-knowledge proofs.
          </p>
          
          <h4 style={{ marginBottom: '0.75rem' }}>Request Body:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`{
  "from": "0x1234567890abcdef1234567890abcdef12345678",
  "to": "0xabcdef1234567890abcdef1234567890abcdef12",
  "amount": 1000
}`}
          </pre>
        </div>
      </div>

      {/* Batch Processing */}
      <div className="card" id="batch-processing">
        <h2 className="card-title">Batch Processing</h2>
        
        <div style={{ marginBottom: '2rem' }} id="process-batch">
          <h3>Process Batch</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-warning">POST</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/batch/process</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Processes all pending transactions in a single batch using RISC Zero zero-knowledge proofs. This is the core 
            cryptographic operation of the Zklear system. The batch processing validates all transactions (checking balances, 
            nonces, and signatures), updates account states, and generates a new Merkle root that cryptographically 
            commits to the new state. The zero-knowledge proof ensures that the state transition is valid without revealing 
            individual account details. This endpoint returns the number of processed transactions, the new Merkle root, 
            and saves a cryptographic receipt for verification.
          </p>
          
          <h4 style={{ marginBottom: '0.75rem' }}>Response Example:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`{
  "success": true,
  "message": "Batch processed successfully",
  "processed_count": 3,
  "new_root": "0x2bb9b82c29f5c7d85b08f9c8908d648c454e317fbb43b51beb9b632ce3049c1",
  "receipt_saved": true
}`}
          </pre>
        </div>

        <div style={{ marginBottom: '2rem' }} id="verify-receipt">
          <h3>Verify Receipt</h3>
          <div style={{ marginBottom: '1rem' }}>
            <span className="badge badge-warning">POST</span>
            <span style={{ fontFamily: 'monospace', marginLeft: '0.5rem' }}>/api/receipt/verify</span>
          </div>
          <p style={{ marginBottom: '1.5rem' }}>
            Verifies a previously generated zero-knowledge proof receipt to ensure that batch processing was executed 
            correctly and honestly. This endpoint performs cryptographic verification of the RISC Zero proof, checking 
            that the state transition was valid, all transactions were processed correctly, and the new Merkle root 
            was computed properly. This verification can be performed by anyone with access to the receipt, enabling 
            trustless verification of system integrity without requiring access to the full system state.
          </p>
          
          <h4 style={{ marginBottom: '0.75rem' }}>Response Example:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`{
  "success": true,
  "message": "Receipt verified successfully",
  "processed_count": 3,
  "new_root": "0x2bb9b82c29f5c7d85b08f9c8908d648c454e317fbb43b51beb9b632ce3049c1"
}`}
          </pre>
        </div>
      </div>



      {/* Testing */}
      <div className="card">
        <h2 className="card-title">Testing Examples</h2>
        <p style={{ marginBottom: '1.5rem' }}>You can test the API endpoints using curl or any HTTP client:</p>
        
        <div style={{ marginBottom: '2rem' }}>
          <h4 style={{ marginBottom: '0.75rem' }}>Get System Info:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`curl http://localhost:8081/api/info`}
          </pre>
        </div>

        <div style={{ marginBottom: '2rem' }}>
          <h4 style={{ marginBottom: '0.75rem' }}>Create Account:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`curl -X POST http://localhost:8081/api/accounts/create \\
  -H "Content-Type: application/json" \\
  -d '{"balance": 10000}'`}
          </pre>
        </div>

        <div style={{ marginBottom: '2rem' }}>
          <h4 style={{ marginBottom: '0.75rem' }}>Create Transaction:</h4>
          <pre style={{ 
            background: '#2d3748', 
            color: '#e2e8f0', 
            padding: '1rem', 
            borderRadius: '4px',
            overflow: 'auto',
            fontSize: '0.9rem'
          }}>
{`curl -X POST http://localhost:8081/api/transactions \\
  -H "Content-Type: application/json" \\
  -d '{"from": "0x1234...", "to": "0x5678...", "amount": 1000}'`}
          </pre>
        </div>
      </div>
    </div>
  );
};

export default ApiSpec; 