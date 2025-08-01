import React, { useState, useEffect } from 'react';
import apiService, { SystemInfo, Account, Transaction } from '../services/api';

const Dashboard: React.FC = () => {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [batchResult, setBatchResult] = useState<{
    processedCount?: number;
    newRoot?: string;
    receiptSaved: boolean;
  } | null>(null);

  // Pagination states
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage] = useState(10);

  // Form states
  const [newAccountBalance, setNewAccountBalance] = useState('');
  const [newTransaction, setNewTransaction] = useState({
    from: '',
    to: '',
    amount: ''
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      

      
      const [systemData, accountsData, transactionsData] = await Promise.all([
        apiService.getSystemInfo(),
        apiService.getAccounts(),
        apiService.getTransactions()
      ]);
      
      setSystemInfo(systemData);
      setAccounts(accountsData);
      setTransactions(transactionsData);
      setError(null);
    } catch (err) {
      console.error('Data loading error:', err);
      setError(`Failed to load data: ${err instanceof Error ? err.message : 'Unknown error'}. Make sure the API server is running.`);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateAccount = async () => {
    try {
      const balance = parseInt(newAccountBalance);
      if (isNaN(balance) || balance <= 0) {
        setError('Please enter a valid balance');
        return;
      }

      const response = await apiService.createAccountWithRandomAddress(balance);
      if (response.success) {
        setMessage('Account created successfully!');
        setNewAccountBalance('');
        loadData();
      } else {
        setError(response.message);
      }
    } catch (err) {
      console.error('Account creation error:', err);
      setError(`Failed to create account: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleCreateTransaction = async () => {
    try {
      const amount = parseInt(newTransaction.amount);
      if (isNaN(amount) || amount <= 0) {
        setError('Please enter a valid amount');
        return;
      }

      if (!newTransaction.from || !newTransaction.to) {
        setError('Please enter both sender and recipient addresses');
        return;
      }

      const response = await apiService.createTransaction({
        from: newTransaction.from,
        to: newTransaction.to,
        amount
      });

      if (response.success) {
        setMessage('Transaction created successfully!');
        setNewTransaction({ from: '', to: '', amount: '' });
        loadData();
      } else {
        setError(response.message);
      }
    } catch (err) {
      console.error('Transaction creation error:', err);
      setError(`Failed to create transaction: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleProcessBatch = async () => {
    try {
      const response = await apiService.processBatch();
      if (response.success) {
        setMessage('Batch processed successfully!');
        setBatchResult({
          processedCount: response.processed_count || 0,
          newRoot: response.new_root || '',
          receiptSaved: response.receipt_saved
        });
        loadData();
      } else {
        setError(response.message);
        setBatchResult(null);
      }
    } catch (err) {
      console.error('Batch processing error:', err);
      setError(`Failed to process batch: ${err instanceof Error ? err.message : 'Unknown error'}`);
      setBatchResult(null);
    }
  };

  // Pagination logic
  const indexOfLastItem = currentPage * itemsPerPage;
  const indexOfFirstItem = indexOfLastItem - itemsPerPage;
  const currentAccounts = accounts.slice(indexOfFirstItem, indexOfLastItem);
  const totalPages = Math.ceil(accounts.length / itemsPerPage);

  const handlePageChange = (pageNumber: number) => {
    setCurrentPage(pageNumber);
  };

  const handleDeleteAccount = async (address: string) => {
    if (window.confirm('Are you sure you want to delete this account?')) {
      try {
        // For now, we'll just remove it from the local state
        // In a real app, you'd call an API to delete the account
        setAccounts(accounts.filter(acc => acc.address !== address));
        setMessage('Account deleted successfully!');
      } catch (err) {
        setError('Failed to delete account');
      }
    }
  };

  if (loading) {
    return (
      <div className="loading">
        <h2>Loading Zklear Dashboard...</h2>
      </div>
    );
  }

  return (
    <div>
      <h1 style={{ marginBottom: '2rem', color: '#2d3748', fontSize: '2rem', fontWeight: '700' }}>Zklear Dashboard</h1>
      
      {error && <div className="error">{error}</div>}
      {message && <div className="success">{message}</div>}

      {/* System Stats */}
      {systemInfo && (
        <div className="card">
          <h2 className="card-title">System Overview</h2>
          <div className="stats-grid">
            <div className="stat-card">
              <div className="stat-value">{systemInfo.account_count}</div>
              <div className="stat-label">Total Accounts</div>
            </div>
            <div className="stat-card">
              <div className="stat-value">{systemInfo.transaction_count}</div>
              <div className="stat-label">Pending Transactions</div>
            </div>
            <div className="stat-card">
              <div className="stat-value">{systemInfo.total_amount.toLocaleString()}</div>
              <div className="stat-label">Total Balance</div>
            </div>

          </div>
          
          <div style={{ marginTop: '1rem' }}>
            <strong>Current Merkle Root:</strong>
            <div style={{ 
              fontFamily: 'monospace', 
              fontSize: '0.9rem', 
              background: '#f7fafc', 
              padding: '0.5rem', 
              borderRadius: '4px',
              marginTop: '0.5rem',
              wordBreak: 'break-all'
            }}>
              {systemInfo.current_root}
            </div>
          </div>
        </div>
      )}

      {/* Accounts Section */}
      <div className="grid grid-2">
        {/* Create Account */}
        <div className="card">
          <h2 className="card-title">Create Account</h2>
          <div className="form-group">
            <label className="form-label">Initial Balance</label>
            <input
              type="number"
              className="form-input"
              placeholder="Enter initial balance"
              value={newAccountBalance}
              onChange={(e) => setNewAccountBalance(e.target.value)}
            />
          </div>
          <button className="btn" onClick={handleCreateAccount}>
            Create Account
          </button>
        </div>

        {/* Accounts List */}
        <div className="card">
          <h2 className="card-title">Accounts</h2>
          {accounts.length === 0 ? (
            <p style={{ color: '#718096' }}>No accounts found</p>
          ) : (
            <>
              <ul className="list">
                {currentAccounts.map((account, index) => (
                  <li key={index} className="list-item">
                    <div>
                      <div className="list-item-header">
                        <span 
                          style={{ 
                            fontFamily: 'monospace', 
                            fontSize: '0.9rem',
                            cursor: 'pointer',
                            padding: '0.25rem 0.5rem',
                            backgroundColor: '#f7fafc',
                            borderRadius: '4px',
                            border: '1px solid #e2e8f0'
                          }}
                          onClick={() => navigator.clipboard.writeText(account.address)}
                          title="Click to copy address"
                        >
                          {account.address}
                        </span>
                      </div>
                      <div className="list-item-subtitle">
                        Balance: {account.balance.toLocaleString()} | Nonce: {account.nonce}
                      </div>
                    </div>
                    <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
                      <span className="badge badge-success">Active</span>
                      <button 
                        onClick={() => handleDeleteAccount(account.address)}
                        style={{
                          background: '#fed7d7',
                          color: '#c53030',
                          border: 'none',
                          padding: '0.25rem 0.5rem',
                          borderRadius: '4px',
                          cursor: 'pointer',
                          fontSize: '0.8rem'
                        }}
                        title="Delete account"
                      >
                        Delete
                      </button>
                    </div>
                  </li>
                ))}
              </ul>
              
              {/* Pagination */}
              {totalPages > 1 && (
                <div style={{ 
                  display: 'flex', 
                  justifyContent: 'center', 
                  gap: '0.5rem', 
                  marginTop: '1rem',
                  alignItems: 'center'
                }}>
                  <button 
                    onClick={() => handlePageChange(currentPage - 1)}
                    disabled={currentPage === 1}
                    className="btn btn-small"
                    style={{ 
                      opacity: currentPage === 1 ? 0.5 : 1,
                      cursor: currentPage === 1 ? 'not-allowed' : 'pointer'
                    }}
                  >
                    Previous
                  </button>
                  
                  <span style={{ fontSize: '0.9rem', color: '#718096' }}>
                    Page {currentPage} of {totalPages}
                  </span>
                  
                  <button 
                    onClick={() => handlePageChange(currentPage + 1)}
                    disabled={currentPage === totalPages}
                    className="btn btn-small"
                    style={{ 
                      opacity: currentPage === totalPages ? 0.5 : 1,
                      cursor: currentPage === totalPages ? 'not-allowed' : 'pointer'
                    }}
                  >
                    Next
                  </button>
                </div>
              )}
            </>
          )}
        </div>
      </div>

      {/* Transactions Section */}
      <div className="grid grid-2">
        {/* Create Transaction */}
        <div className="card">
          <h2 className="card-title">Create Transaction</h2>
          <div className="form-group">
            <label className="form-label">From Address</label>
            <input
              type="text"
              className="form-input"
              placeholder="0x..."
              value={newTransaction.from}
              onChange={(e) => setNewTransaction({...newTransaction, from: e.target.value})}
            />
          </div>
          <div className="form-group">
            <label className="form-label">To Address</label>
            <input
              type="text"
              className="form-input"
              placeholder="0x..."
              value={newTransaction.to}
              onChange={(e) => setNewTransaction({...newTransaction, to: e.target.value})}
            />
          </div>
          <div className="form-group">
            <label className="form-label">Amount</label>
            <input
              type="number"
              className="form-input"
              placeholder="Enter amount"
              value={newTransaction.amount}
              onChange={(e) => setNewTransaction({...newTransaction, amount: e.target.value})}
            />
          </div>
          <button className="btn" onClick={handleCreateTransaction}>
            Create Transaction
          </button>
        </div>

        {/* Transactions List */}
        <div className="card">
          <h2 className="card-title">Pending Transactions</h2>
          {transactions.length === 0 ? (
            <p style={{ color: '#718096' }}>No pending transactions</p>
          ) : (
            <ul className="list">
              {transactions.map((tx, index) => (
                <li key={index} className="list-item">
                  <div>
                    <div className="list-item-header">
                      {tx.amount.toLocaleString()} tokens
                    </div>
                    <div className="list-item-subtitle">
                      From: {tx.from.substring(0, 8)}... → To: {tx.to.substring(0, 8)}...
                    </div>
                  </div>
                  <span className="badge badge-warning">Pending</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>

      {/* Batch Processing Section */}
      <div className="card" style={{ marginTop: '2rem' }}>
        <h2 className="card-title">Batch Processing</h2>
        <p className="card-subtitle">
          Process all pending transactions with zero-knowledge proofs
        </p>
        <div style={{ marginBottom: '1rem' }}>
          <button className="btn" onClick={handleProcessBatch}>
            Process Batch
          </button>
        </div>
        
        {/* Receipt and proof visualization */}
        {batchResult ? (
          <div style={{ 
            border: '2px solid #48bb78', 
            borderRadius: '8px',
            padding: '1.5rem',
            backgroundColor: '#f0fff4',
            color: '#2f855a'
          }}>
            <h3 style={{ margin: '0 0 1rem 0', fontSize: '1.2rem', color: '#2f855a' }}>
              ✅ ZK Proof Generated Successfully
            </h3>
            
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem', marginBottom: '1rem' }}>
              <div>
                <strong>Transactions Processed:</strong>
                <div style={{ 
                  fontFamily: 'monospace', 
                  fontSize: '1.1rem',
                  marginTop: '0.25rem'
                }}>
                  {batchResult.processedCount}
                </div>
              </div>
              
              <div>
                <strong>Receipt Status:</strong>
                <div style={{ 
                  fontFamily: 'monospace', 
                  fontSize: '1.1rem',
                  marginTop: '0.25rem'
                }}>
                  {batchResult.receiptSaved ? '✅ Saved' : '❌ Not Saved'}
                </div>
              </div>
            </div>
            
            <div>
              <strong>New Merkle Root:</strong>
              <div style={{ 
                fontFamily: 'monospace', 
                fontSize: '0.9rem', 
                background: '#ffffff', 
                padding: '0.75rem', 
                borderRadius: '4px',
                marginTop: '0.5rem',
                wordBreak: 'break-all',
                border: '1px solid #c6f6d5'
              }}>
                {batchResult.newRoot}
              </div>
            </div>
            
            <div style={{ 
              marginTop: '1rem', 
              padding: '1rem', 
              background: '#ffffff', 
              borderRadius: '4px',
              border: '1px solid #c6f6d5'
            }}>
              <strong>ZK Proof Details:</strong>
              <ul style={{ margin: '0.5rem 0 0 0', paddingLeft: '1.5rem' }}>
                <li>Proof generated using RISC Zero zkVM</li>
                <li>Receipt saved to <code>receipt.bin</code></li>
                <li>Merkle root updated cryptographically</li>
                <li>All transactions verified and processed</li>
              </ul>
              
              <div style={{ marginTop: '1rem', textAlign: 'center' }}>
                <button 
                  onClick={() => window.open('/api/receipt/download', '_blank')}
                  style={{
                    background: '#2d3748',
                    color: 'white',
                    border: 'none',
                    padding: '0.75rem 1.5rem',
                    borderRadius: '6px',
                    cursor: 'pointer',
                    fontSize: '0.9rem',
                    fontWeight: '600'
                  }}
                >
                  📥 Download ZK Proof Receipt
                </button>
              </div>
            </div>
          </div>
        ) : (
          <div style={{ 
            minHeight: '200px', 
            border: '2px dashed #e2e8f0', 
            borderRadius: '8px',
            padding: '1rem',
            textAlign: 'center',
            color: '#718096',
            backgroundColor: '#f7fafc'
          }}>
            <p style={{ margin: '0', fontSize: '1.1rem' }}>
              Receipt and proof will be displayed here after batch processing
            </p>
            <p style={{ margin: '0.5rem 0 0 0', fontSize: '0.9rem' }}>
              This area will show the generated ZK proof and receipt for verification
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Dashboard; 