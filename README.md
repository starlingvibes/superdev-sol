# Solana HTTP Server

A Rust-based HTTP server built with Axum that provides Solana-related endpoints for keypair generation, SPL token operations, message signing/verification, and SOL/token transfers.

## Features

- **Keypair Generation**: Generate Ed25519 keypairs for Solana
- **SPL Token Operations**: Create tokens and mint operations
- **Message Signing/Verification**: Sign and verify messages using Ed25519
- **SOL Transfers**: Create transfer instructions for SOL
- **Token Transfers**: Create transfer instructions for SPL tokens
- **Mock Implementation**: All operations return mock data (no actual blockchain calls)
- **Deployment Ready**: Configured for cloud deployment (Render, Railway, etc.)

## API Endpoints

### 1. Generate Keypair
- **POST** `/keypair`
- **Description**: Generates a new Ed25519 keypair
- **Response**: Returns public key and private key in base58 format

### 2. Create Token
- **POST** `/token/create`
- **Description**: Creates a mock SPL token initialization instruction
- **Body**:
  ```json
  {
    "mint": "string (base58 public key)",
    "mint_authority": "string (base58 public key)",
    "decimals": "number (0-255)"
  }
  ```

### 3. Mint Token
- **POST** `/token/mint`
- **Description**: Creates a mock token minting instruction
- **Body**:
  ```json
  {
    "mint": "string (base58 public key)",
    "destination": "string (base58 public key)",
    "authority": "string (base58 public key)",
    "amount": "number"
  }
  ```

### 4. Sign Message
- **POST** `/message/sign`
- **Description**: Signs a message using Ed25519
- **Body**:
  ```json
  {
    "message": "string",
    "secret": "string (base58 private key)"
  }
  ```

### 5. Verify Message
- **POST** `/message/verify`
- **Description**: Verifies a message signature using Ed25519
- **Body**:
  ```json
  {
    "message": "string",
    "signature": "string (base64)",
    "pubkey": "string (base58 public key)"
  }
  ```

### 6. Send SOL
- **POST** `/send/sol`
- **Description**: Creates a mock SOL transfer instruction
- **Body**:
  ```json
  {
    "from": "string (base58 public key)",
    "to": "string (base58 public key)",
    "lamports": "number"
  }
  ```

### 7. Send Token
- **POST** `/send/token`
- **Description**: Creates a mock SPL token transfer instruction
- **Body**:
  ```json
  {
    "destination": "string (base58 public key)",
    "mint": "string (base58 public key)",
    "owner": "string (base58 public key)",
    "amount": "number"
  }
  ```

## Response Format

All endpoints return JSON responses in a consistent format:

### Success Response (Status 200)
```json
{
  "success": true,
  "data": { /* endpoint-specific result */ }
}
```

### Error Response (Status 400)
```json
{
  "success": false,
  "error": "Description of error"
}
```

## Technical Details

### Cryptography
- **Ed25519**: Used for all signing and verification operations
- **Base58**: Encoding format for public and private keys
- **Base64**: Encoding format for signatures

### Error Handling
- Comprehensive input validation
- Proper HTTP status codes (200 for success, 400 for errors)
- Detailed error messages
- Consistent response format

### Security
- No private keys stored on server
- Standard cryptographic libraries used
- Input validation for all endpoints
- Safe error handling to prevent information leakage

## Setup Instructions

### Prerequisites
- Rust 1.70.0 or later
- Cargo package manager

### Local Development

1. **Clone the repository**:
   ```bash
   git clone <your-repository-url>
   cd superdev-project
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Run the server locally**:
   ```bash
   cargo run
   ```
   
   The server will start on `http://localhost:8080` by default.

4. **Set custom port** (optional):
   ```bash
   PORT=3000 cargo run
   ```

### Production Deployment

#### Environment Variables
- `PORT`: Port number for the server (default: 8080)

#### Deploy to Render

1. **Create a new Web Service** on [Render](https://render.com)

2. **Connect your GitHub repository**

3. **Configure the service**:
   - **Environment**: Rust
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/superdev-project`
   - **Port**: Set `PORT` environment variable or use default

4. **Deploy**: Render will automatically build and deploy your application

#### Deploy to Railway

1. **Create a new project** on [Railway](https://railway.app)

2. **Connect your GitHub repository**

3. **Configure deployment**:
   - Railway will automatically detect it's a Rust project
   - Set `PORT` environment variable if needed

4. **Deploy**: Railway will handle the build and deployment

#### Deploy to other platforms

The server is designed to work with any platform that supports Rust applications:
- Heroku
- AWS
- Google Cloud Platform
- DigitalOcean App Platform

## Testing

### Manual Testing

Test the endpoints using curl:

```bash
# Generate a keypair
curl -X POST http://localhost:8080/keypair

# Sign a message (use the secret from keypair generation)
curl -X POST http://localhost:8080/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, Solana!", "secret": "YOUR_SECRET_KEY"}'

# Verify a message (use the signature and public key from signing)
curl -X POST http://localhost:8080/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, Solana!", "signature": "SIGNATURE", "pubkey": "PUBLIC_KEY"}'
```

### Test Error Handling

```bash
# Test invalid input
curl -X POST http://localhost:8080/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "test"}'  # Missing secret key

# Test invalid key format
curl -X POST http://localhost:8080/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "test", "secret": "invalid_key"}'
```

## Project Structure

```
superdev-project/
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── handlers.rs      # Endpoint implementations
│   └── types.rs         # Request/response type definitions
├── Cargo.toml           # Dependencies and project configuration
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

## Dependencies

- **axum**: Modern web framework for Rust
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **ed25519-dalek**: Ed25519 cryptographic operations
- **solana-sdk**: Solana types and utilities
- **bs58**: Base58 encoding/decoding
- **base64**: Base64 encoding/decoding
- **uuid**: UUID generation
- **rand**: Random number generation

## Additional Features

- **Health Check**: Basic health check endpoint at root (`/`)
- **CORS**: Configured for cross-origin requests
- **Logging**: Structured logging for debugging and monitoring
- **Error Recovery**: Graceful error handling throughout the application
- **Memory Safe**: Built with Rust for memory safety and performance

## Performance Optimizations

- **Async/Await**: Non-blocking I/O operations
- **Zero-Copy**: Efficient data handling where possible
- **Minimal Dependencies**: Only essential crates included
- **Release Builds**: Optimized for production deployment

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions:
1. Check the existing issues on GitHub
2. Create a new issue with detailed description
3. Include steps to reproduce any bugs

## Deployment Status

🚀 **Ready for Production Deployment**

This server is configured and tested for production deployment on various cloud platforms. All endpoints have been validated for correct response formats and error handling.
