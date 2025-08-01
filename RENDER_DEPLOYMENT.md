# Zklear Payment System - Render Deployment Guide

## Overview

This guide will help you deploy the Zklear zero-knowledge payment system to Render, a modern cloud platform that makes deployment simple and efficient.

## Prerequisites

- A Render account (free tier available)
- Git repository with the Zklear codebase
- Docker installed locally (for testing)

## Deployment Steps

### 1. Prepare Your Repository

Ensure your repository contains all the necessary files:

```
ocl/
├── src/                    # Rust backend
├── methods/               # RISC Zero guest code
├── zklear-frontend/       # React application
├── Dockerfile.render      # Render-optimized Dockerfile
├── build.sh              # Build script
├── start.sh              # Start script
├── render.yaml           # Render configuration
└── Cargo.toml           # Rust dependencies
```

### 2. Connect to Render

1. **Sign up/Login to Render**: Go to [render.com](https://render.com)
2. **Create a new Web Service**:
   - Click "New +" → "Web Service"
   - Connect your Git repository
   - Select the repository containing Zklear

### 3. Configure the Service

Use these settings in Render:

**Basic Settings:**
- **Name**: `zklear-payment-system`
- **Environment**: `Docker`
- **Region**: Choose closest to your users
- **Branch**: `main`

**Build & Deploy Settings:**
- **Build Command**: `./build.sh`
- **Start Command**: `./start.sh`
- **Dockerfile Path**: `./Dockerfile.render`

**Environment Variables:**
```
RUST_LOG=info
RUST_BACKTRACE=1
```

### 4. Advanced Configuration

**Health Check:**
- **Path**: `/api/system-info`
- **Interval**: 30 seconds

**Auto-Deploy:**
- Enable automatic deployments from your main branch

### 5. Deploy

1. Click "Create Web Service"
2. Render will automatically:
   - Build the Docker image
   - Install dependencies
   - Compile the Rust application
   - Build the React frontend
   - Start the service

## Architecture

### Single Container Design

Zklear uses a single container approach optimized for Render:

```
┌─────────────────────────────────────┐
│         Render Container            │
│  ┌─────────────┐  ┌─────────────┐  │
│  │ Rust API    │  │ React App   │  │
│  │ (Port 8080) │  │ (Static)    │  │
│  └─────────────┘  └─────────────┘  │
└─────────────────────────────────────┘
```

### Key Features

- **Zero-Knowledge Proofs**: RISC Zero zkVM integration
- **Merkle Tree State**: Cryptographic state management
- **React Frontend**: Modern web interface
- **RESTful API**: Comprehensive API endpoints
- **Docker Optimization**: Multi-stage builds for efficiency

## Environment Variables

### Required by Render
- `PORT`: Automatically set by Render

### Optional Configuration
- `RUST_LOG`: Logging level (default: info)
- `RUST_BACKTRACE`: Enable backtraces (default: 1)

## API Endpoints

Once deployed, your API will be available at:

```
https://your-app-name.onrender.com/api/
```

**Available Endpoints:**
- `GET /api/system-info` - System information
- `GET /api/accounts` - List all accounts
- `POST /api/accounts/create` - Create new account
- `GET /api/transactions` - List all transactions
- `POST /api/transactions/create` - Create new transaction
- `POST /api/batch/process` - Process transaction batch
- `POST /api/receipt/verify` - Verify ZK proof receipt

## Frontend Access

The React frontend will be served at:
```
https://your-app-name.onrender.com/
```

## Monitoring & Logs

### View Logs
1. Go to your service dashboard in Render
2. Click on "Logs" tab
3. Monitor real-time application logs

### Health Checks
- Render automatically checks `/api/system-info` every 30 seconds
- Service will restart if health checks fail

## Troubleshooting

### Common Issues

**1. Build Failures**
- Check that all dependencies are properly specified
- Ensure RISC Zero toolchain is installed in Dockerfile
- Verify Node.js and npm are available

**2. Runtime Errors**
- Check logs for specific error messages
- Verify environment variables are set correctly
- Ensure PORT environment variable is being used

**3. API Connection Issues**
- Verify the API endpoints are correct
- Check that the frontend is using the right API URL
- Ensure CORS is properly configured

### Debug Commands

```bash
# Check if service is running
curl https://your-app-name.onrender.com/api/system-info

# View build logs
# Available in Render dashboard under "Logs"

# Test local deployment
docker build -f Dockerfile.render -t zklear .
docker run -p 8080:8080 zklear
```

## Performance Optimization

### Render-Specific Optimizations

1. **Multi-stage Docker builds** reduce image size
2. **Static file serving** from Rust backend
3. **Environment variable configuration** for flexibility
4. **Health checks** ensure service availability

### Resource Usage

- **Memory**: ~512MB (starter plan)
- **CPU**: Shared (starter plan)
- **Storage**: ~1GB for application and dependencies

## Security Considerations

1. **HTTPS**: Automatically provided by Render
2. **Environment Variables**: Secure storage for sensitive data
3. **CORS**: Configured for web frontend
4. **Input Validation**: All API endpoints validate input

## Scaling

### Render Plans

- **Starter**: Free tier, suitable for development
- **Standard**: $7/month, better performance
- **Pro**: $25/month, dedicated resources

### Auto-scaling

Render automatically handles:
- Traffic spikes
- Load balancing
- Health monitoring
- Automatic restarts

## Support

For Render-specific issues:
- [Render Documentation](https://render.com/docs)
- [Render Community](https://community.render.com)

For Zklear-specific issues:
- Check the logs in Render dashboard
- Review the application code
- Test locally with Docker

## Next Steps

After successful deployment:

1. **Test all functionality** through the web interface
2. **Monitor performance** using Render's dashboard
3. **Set up custom domain** if needed
4. **Configure monitoring** and alerts
5. **Scale resources** as usage grows

---

**Happy deploying! 🚀**

Your Zklear zero-knowledge payment system is now ready for production use on Render. 