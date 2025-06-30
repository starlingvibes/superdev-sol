use axum::{extract::Json, response::Json as ResponseJson};
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::types::*;

const SPL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

pub async fn generate_keypair() -> ResponseJson<ApiResponse<KeypairResponse>> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    
    let pubkey = bs58::encode(verifying_key.as_bytes()).into_string();
    let secret = bs58::encode(signing_key.as_bytes()).into_string();
    
    let response = KeypairResponse { pubkey, secret };
    ResponseJson(ApiResponse::success(response))
}

pub async fn create_token(
    payload: Result<Json<CreateTokenRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate inputs
    let _mint_authority = match Pubkey::from_str(&req.mint_authority) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid mint authority public key".to_string())),
    };
    
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid mint public key".to_string())),
    };

    // Create mock instruction for initialize mint
    let accounts = vec![
        AccountMeta {
            pubkey: mint.to_string(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: "SysvarRent111111111111111111111111111111111".to_string(),
            is_signer: false,
            is_writable: false,
        },
    ];

    // Mock instruction data for InitializeMint
    let instruction_data = BASE64.encode([
        0, // InitializeMint instruction index
        req.decimals,
    ]);

    let response = InstructionResponse {
        program_id: SPL_TOKEN_PROGRAM_ID.to_string(),
        accounts,
        instruction_data,
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn mint_token(
    payload: Result<Json<MintTokenRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate inputs
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid mint public key".to_string())),
    };
    
    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid destination public key".to_string())),
    };
    
    let authority = match Pubkey::from_str(&req.authority) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid authority public key".to_string())),
    };

    let accounts = vec![
        AccountMeta {
            pubkey: destination.to_string(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: mint.to_string(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: authority.to_string(),
            is_signer: true,
            is_writable: false,
        },
    ];

    // Mock instruction data for MintTo
    let mut instruction_data = vec![7]; // MintTo instruction index
    instruction_data.extend_from_slice(&req.amount.to_le_bytes());
    
    let response = InstructionResponse {
        program_id: SPL_TOKEN_PROGRAM_ID.to_string(),
        accounts,
        instruction_data: BASE64.encode(instruction_data),
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn sign_message(
    payload: Result<Json<SignMessageRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<SignMessageResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate and decode secret key
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid secret key format".to_string())),
    };

    if secret_bytes.len() != 32 {
        return ResponseJson(ApiResponse::error("Invalid secret key length".to_string()));
    }

    let mut secret_array = [0u8; 32];
    secret_array.copy_from_slice(&secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_array);
    let verifying_key = signing_key.verifying_key();

    // Sign the message
    let message_bytes = req.message.as_bytes();
    let signature = signing_key.sign(message_bytes);

    let response = SignMessageResponse {
        signature: BASE64.encode(signature.to_bytes()),
        public_key: bs58::encode(verifying_key.as_bytes()).into_string(),
        message: req.message,
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn verify_message(
    payload: Result<Json<VerifyMessageRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<VerifyMessageResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate and decode public key
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid public key format".to_string())),
    };

    if pubkey_bytes.len() != 32 {
        return ResponseJson(ApiResponse::error("Invalid public key length".to_string()));
    }

    let mut pubkey_array = [0u8; 32];
    pubkey_array.copy_from_slice(&pubkey_bytes);

    let verifying_key = match VerifyingKey::from_bytes(&pubkey_array) {
        Ok(vk) => vk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid public key".to_string())),
    };

    // Decode signature
    let signature_bytes = match BASE64.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid signature format".to_string())),
    };

    if signature_bytes.len() != 64 {
        return ResponseJson(ApiResponse::error("Invalid signature length".to_string()));
    }

    let mut signature_array = [0u8; 64];
    signature_array.copy_from_slice(&signature_bytes);

    let signature = Signature::from_bytes(&signature_array);

    // Verify the signature
    let message_bytes = req.message.as_bytes();
    let is_valid = verifying_key.verify(message_bytes, &signature).is_ok();

    let response = VerifyMessageResponse {
        valid: is_valid,
        message: req.message,
        pubkey: req.pubkey,
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn send_sol(
    payload: Result<Json<SendSolRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate inputs
    let from = match Pubkey::from_str(&req.from) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid sender public key".to_string())),
    };
    
    let to = match Pubkey::from_str(&req.to) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid recipient public key".to_string())),
    };

    if req.lamports == 0 {
        return ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string()));
    }

    // Create transfer instruction accounts
    let accounts = vec![
        AccountMeta {
            pubkey: from.to_string(),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: to.to_string(),
            is_signer: false,
            is_writable: true,
        },
    ];

    // Create instruction data for transfer (system program instruction type 2)
    let mut instruction_data = vec![2, 0, 0, 0]; // Transfer instruction discriminator
    instruction_data.extend_from_slice(&req.lamports.to_le_bytes());

    let response = InstructionResponse {
        program_id: SYSTEM_PROGRAM_ID.to_string(),
        accounts,
        instruction_data: BASE64.encode(instruction_data),
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn send_token(
    payload: Result<Json<SendTokenRequest>, axum::extract::rejection::JsonRejection>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    let req = match payload {
        Ok(Json(req)) => req,
        Err(_) => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };
    // Validate inputs
    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid destination public key".to_string())),
    };
    
    let _mint = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid mint public key".to_string())),
    };
    
    let owner = match Pubkey::from_str(&req.owner) {
        Ok(pk) => pk,
        Err(_) => return ResponseJson(ApiResponse::error("Invalid owner public key".to_string())),
    };

    if req.amount == 0 {
        return ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string()));
    }

    // For SPL token transfers, we need source token account
    // This is a simplified mock - in reality you'd derive the associated token account
    let source_token_account = format!("{}Source", owner);

    let accounts = vec![
        AccountMeta {
            pubkey: source_token_account,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: destination.to_string(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: owner.to_string(),
            is_signer: true,
            is_writable: false,
        },
    ];

    // Mock instruction data for Transfer
    let mut instruction_data = vec![3]; // Transfer instruction index
    instruction_data.extend_from_slice(&req.amount.to_le_bytes());

    let response = InstructionResponse {
        program_id: SPL_TOKEN_PROGRAM_ID.to_string(),
        accounts,
        instruction_data: BASE64.encode(instruction_data),
    };

    ResponseJson(ApiResponse::success(response))
}
