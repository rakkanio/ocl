import axios from 'axios';

const API_BASE_URL = 'http://localhost:8081';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 10000,
});

// Types
export interface Account {
  address: string;
  balance: number;
  nonce: number;
}

export interface Transaction {
  from: string;
  to: string;
  amount: number;
  nonce: number;
  signature: string;
}

export interface SystemInfo {
  current_root: string;
  total_amount: number;
  account_count: number;
  transaction_count: number;
  system_stats: {
    total_accounts_created: number;
    total_transactions_processed: number;
    average_balance: number;
    highest_balance: number;
    lowest_balance: number;
  };
}

export interface CreateAccountRequest {
  address?: string;
  balance: number;
}

export interface CreateTransactionRequest {
  from: string;
  to: string;
  amount: number;
}

export interface CreateAccountResponse {
  success: boolean;
  message: string;
  account?: Account;
}

export interface CreateTransactionResponse {
  success: boolean;
  message: string;
  transaction?: Transaction;
}

export interface ProcessBatchResponse {
  success: boolean;
  message: string;
  processed_count?: number;
  new_root?: string;
  receipt_saved: boolean;
}

// API functions
export const apiService = {


  // System info
  getSystemInfo: async (): Promise<SystemInfo> => {
    const response = await api.get('/api/info');
    return response.data;
  },

  // Accounts
  getAccounts: async (): Promise<Account[]> => {
    const response = await api.get('/api/accounts');
    return response.data;
  },

  createAccount: async (data: CreateAccountRequest): Promise<CreateAccountResponse> => {
    const response = await api.post('/api/accounts', data);
    return response.data;
  },

  createAccountWithRandomAddress: async (balance: number): Promise<CreateAccountResponse> => {
    const response = await api.post('/api/accounts/create', { balance });
    return response.data;
  },

  // Transactions
  getTransactions: async (): Promise<Transaction[]> => {
    const response = await api.get('/api/transactions');
    return response.data;
  },

  createTransaction: async (data: CreateTransactionRequest): Promise<CreateTransactionResponse> => {
    const response = await api.post('/api/transactions', data);
    return response.data;
  },

  // Batch processing
  processBatch: async (): Promise<ProcessBatchResponse> => {
    const response = await api.post('/api/batch/process');
    return response.data;
  },

  verifyReceipt: async (): Promise<ProcessBatchResponse> => {
    const response = await api.post('/api/receipt/verify');
    return response.data;
  },
};

export default apiService; 