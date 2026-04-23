# API Code Examples - Phase 2 Implementation

This document provides sample API code for the REST endpoints implemented in Phase 2.

---

## 1. Main API Server (Axum/Rust)

```rust
// src/main.rs

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

mod handlers;
mod models;
mod zkp;

pub type SharedState = Arc<Mutex<AppState>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub blockchain: Blockchain,
    pub fl_aggregator: FLServer,
}

#[tokio::main]
pub async fn main() {
    // Initialize state
    let state = Arc::new(Mutex::new(AppState {
        db: Database::new(),
        blockchain: Blockchain::new(),
        fl_aggregator: FLServer::new(),
    }));

    // Build router
    let app = Router::new()
        // Authentication endpoints
        .route("/api/v1/auth", post(handlers::authenticate))
        
        // Biometric endpoints
        .route("/api/v1/biometric/commitment", post(handlers::create_commitment))
        .route("/api/v1/biometric/update", post(handlers::update_commitment))
        
        // DID endpoints
        .route("/api/v1/did/:id", get(handlers::get_did))
        .route("/api/v1/did", post(handlers::create_did))
        .route("/api/v1/did/revoke", post(handlers::revoke_did))
        
        // Verifiable Credentials
        .route("/api/v1/vc/issue", post(handlers::issue_vc))
        
        // Federated Learning
        .route("/api/v1/fl/get_model", get(handlers::get_global_model))
        .route("/api/v1/fl/update", post(handlers::submit_model_update))
        
        // Health check
        .route("/api/v1/health", get(handlers::health_check))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
```

---

## 2. Data Models

```rust
// src/models/mod.rs

use serde::{Deserialize, Serialize};

/// HDMID - Hashed Decentralized Multi-Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDMID {
    pub hash: String,           // 64-character hex string
    pub salt: String,          // Random salt for hashing
    pub created_at: u64,       // Unix timestamp
    pub commitment: String,    // Pedersen commitment
}

/// DID Document following W3C standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub context: Vec<String>,
    pub id: String,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub service: Option<Vec<ServiceEndpoint>>,
    pub biometric_commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    pub controller: String,
    pub public_key_multibase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub id: String,
    #[serde(rename = "type")]
    pub service_type: String,
    pub endpoint: String,
}

/// ZKP Proof structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKPProof {
    pub a: String,  // G1 element
    pub b: String,  // G2 element
    pub c: String,  // G1 element
}

/// Authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub hdmid: String,
    pub proof: ZKPProof,
    pub timestamp: u64,
    pub verifier_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub session_token: Option<String>,
    pub message: String,
}

/// Commitment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentRequest {
    pub fingerprint_features: Vec<f32>,  // 261 dimensions
    pub iris_features: Vec<f32>,       // 52 dimensions
    pub salt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentResponse {
    pub did: String,
    pub hdmid: String,
    pub commitment: String,
    pub tx_hash: String,
}
```

---

## 3. Authentication Handler

```rust
// src/handlers/authenticate.rs

use axum::{
    extract::State,
    Json,
};
use crate::{models::*, zkp};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn authenticate(
    state: State<SharedState>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    let state = state.lock().await;
    
    // 1. Check timestamp validity (max 5 minutes)
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if current_time > payload.timestamp + 300 {
        return Json(AuthResponse {
            success: false,
            session_token: None,
            message: "Proof expired".to_string(),
        });
    }
    
    // 2. Retrieve stored commitment from database
    let stored_commitment = match state.db.get_commitment(&payload.hdmid) {
        Some(c) => c,
        None => {
            return Json(AuthResponse {
                success: false,
                session_token: None,
                message: "HDMID not found".to_string(),
            });
        }
    };
    
    // 3. Verify ZKP proof
    let is_valid = zkp::verify_proof(
        &payload.proof,
        &stored_commitment,
        &payload.hdmid,
    );
    
    if !is_valid {
        return Json(AuthResponse {
            success: false,
            session_token: None,
            message: "Invalid proof".to_string(),
        });
    }
    
    // 4. Generate session token
    let session_token = generate_session_token(&payload.hdmid);
    
    Json(AuthResponse {
        success: true,
        session_token: Some(session_token),
        message: "Authentication successful".to_string(),
    })
}

fn generate_session_token(hdmid: &str) -> String {
    use std::time::SystemTime;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("session_{}_{}", hdmid[..8].to_string(), timestamp)
}
```

---

## 4. Biometric Commitment Handler

```rust
// src/handlers/commitment.rs

use axum::{
    extract::State,
    Json,
};
use crate::models::*;
use sha2::{Sha256, Digest};
use std::str::FromStr;

pub async fn create_commitment(
    state: State<SharedState>,
    Json(payload): Json<CommitmentRequest>,
) -> Json<CommitmentResponse> {
    let mut state = state.lock().await;
    
    // 1. Generate random salt
    let salt = generate_salt();
    
    // 2. Fuse biometric features
    let mut fused_features = payload.fingerprint_features.clone();
    fused_features.extend(payload.iris_features.clone());
    
    // 3. Generate HDMID hash
    let mut hasher = Sha256::new();
    hasher.update(&fused_features);
    hasher.update(&salt);
    let hdmid_hash = format!("{:x}", hasher.finalize());
    
    // 4. Generate Pedersen commitment
    let commitment = generate_pedersen_commitment(&fused_features, &salt);
    
    // 5. Generate DID
    let did = format!("did:dmid:{}", &hdmid_hash[..20]);
    
    // 6. Create block on blockchain
    let tx_hash = state.blockchain.add_block(BlockData {
        did: did.clone(),
        hdmid_hash: hdmid_hash.clone(),
        commitment: commitment.clone(),
    });
    
    // 7. Store in database
    state.db.store_hdmid(HDMID {
        hash: hdmid_hash.clone(),
        salt: salt.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        commitment: commitment.clone(),
    });
    
    Json(CommitmentResponse {
        did,
        hdmid: hdmid_hash,
        commitment,
        tx_hash,
    })
}

fn generate_salt() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let salt: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..16);
            "0123456789abcdef"[idx..=idx].to_string()
        })
        .collect();
    salt
}

fn generate_pedersen_commitment(features: &[f32], salt: &str) -> String {
    // Simplified - actual implementation uses bellman crate
    format!("pedersen_{}", &features.len())
}
```

---

## 5. ZKP Proof Generation

```rust
// src/zkp/proof.rs

use crate::models::*;

/// Generate Groth16 proof for biometric authentication
pub fn generate_proof(
    witness: &[f32],      // Biometric features
    public_inputs: &[f32], // Commitment & hash
) -> ZKPProof {
    // Note: This is simplified
    // Actual implementation uses bellman::Circuit trait
    
    // 1. Create constraint system
    let mut cs = ConstraintSystem::new();
    
    // 2. Enforce biometry commitment constraint
    // Pedersen(F, r) == C_on_chain
    
    // 3. Enforce hash constraint
    // SHA256(F || salt) == HDMID_on_chain
    
    // 4. Generate proof using trusted setup
    let proof = bellman::Proof::create(
        &get_circuit(),
        &get_params(),
        &[witness, public_inputs],
    );
    
    ZKPProof {
        a: format!("{:?}", proof.a),
        b: format!("{:?}", proof.b),
        c: format!("{:?}", proof.c),
    }
}

/// Verify Groth16 proof
pub fn verify_proof(
    proof: &ZKPProof,
    commitment: &str,
    expected_hash: &str,
) -> bool {
    // Note: This is simplified
    // Actual implementation uses bellman crate
    
    let vk = get_verification_key();
    let public_inputs = vec![
        parse_field_element(commitment),
        parse_field_element(expected_hash),
    ];
    
    bellman::verify_proof(
        &vk,
        proof,
        &public_inputs,
    )
}

// Dummy circuit for illustration
fn get_circuit() -> BiometricCircuit<'static> {
    BiometricCircuit::default()
}

fn get_params() -> Parameters {
    Parameters::new()
}

fn get_verification_key() -> VerificationKey {
    VerificationKey::new()
}

#[derive(Debug, Clone)]
struct Parameters {
    // Groth16 parameters would be here
}

#[derive(Debug, Clone)]
struct VerificationKey {
    // Verification key would be here
}

fn parse_field_element(s: &str) -> f32 {
    // Convert string to field element
    s.len() as f32
}

// Placeholder for BiometricCircuit
#[derive(Debug, Default)]
struct BiometricCircuit<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
```

---

## 6. Federated Learning Handler

```rust
// src/handlers/federated.rs

use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FLModelUpdate {
    pub client_id: String,
    pub weights: Vec<f32>,
    pub sample_count: usize,
    pub round: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalModel {
    pub weights: Vec<f32>,
    pub round: u32,
    pub accuracy: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FLResponse {
    pub success: bool,
    pub message: String,
}

/// Get global model for local training
pub async fn get_global_model(
    state: State<SharedState>,
) -> Json<GlobalModel> {
    let state = state.lock().await;
    let model = state.fl_aggregator.get_global_model();
    
    Json(model)
}

/// Submit local model update
pub async fn submit_model_update(
    state: State<SharedState>,
    Json(update): Json<FLModelUpdate>,
) -> Json<FLResponse> {
    let mut state = state.lock().await;
    
    // Apply differential privacy before aggregation
    let private_update = apply_differential_privacy(update);
    
    // Aggregate using FedAvg
    let result = state.fl_aggregator.aggregate(private_update);
    
    Json(FLResponse {
        success: result,
        message: "Model updated".to_string(),
    })
}

/// Apply differential privacy noise
fn apply_differential_privacy(update: FLModelUpdate) -> FLModelUpdate {
    const NOISE_SCALE: f32 = 0.01;
    
    let mut noisy_weights = update.weights.clone();
    
    for weight in &mut noisy_weights {
        // Add Gaussian noise
        let noise: f32 = rand::distributions::Normal::new(0.0, NOISE_SCALE)
            .unwrap()
            .sample(&mut rand::thread_rng());
        *weight += noise;
    }
    
    FLModelUpdate {
        client_id: update.client_id,
        weights: noisy_weights,
        sample_count: update.sample_count,
        round: update.round,
    }
}
```

---

## 7. DID Handler

```rust
// src/handlers/did.rs

use axum::{
    extract::{Path, State},
    Json,
};
use crate::models::*;

pub async fn get_did(
    state: State<SharedState>,
    Path(did): Path<String>,
) -> Json<Option<DIDDocument>> {
    let state = state.lock().await;
    let doc = state.db.get_did(&did);
    Json(doc)
}

pub async fn create_did(
    state: State<SharedState>,
    Json(payload): Json<CreateDIDRequest>,
) -> Json<DIDResponse> {
    let mut state = state.lock().await;
    
    let did = format!("did:dmid:{}", generate_id());
    let doc = DIDDocument {
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
        ],
        id: did.clone(),
        verification_method: vec![VerificationMethod {
            id: format!("{}#key-1", did),
            method_type: "Ed25519VerificationKey2020".to_string(),
            controller: did.clone(),
            public_key_multibase: payload.public_key,
        }],
        authentication: vec![format!("{}#key-1", did)],
        service: None,
        biometric_commitment: payload.biometric_commitment,
    };
    
    // Store on blockchain
    let tx_hash = state.blockchain.add_did(&doc);
    
    state.db.store_did(&did, doc.clone());
    
    Json(DIDResponse {
        did,
        document: doc,
        tx_hash,
    })
}

#[derive(Debug, Deserialize)]
pub struct CreateDIDRequest {
    pub public_key: String,
    pub biometric_commitment: String,
}

#[derive(Debug, Serialize)]
pub struct DIDResponse {
    pub did: String,
    pub document: DIDDocument,
    pub tx_hash: String,
}

fn generate_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let id: String = (0..20)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    id
}
```

---

## 8. Database Structure

```rust
// src/database/mod.rs

use std::collections::HashMap;
use crate::models::*;

pub struct Database {
    hdmid_store: HashMap<String, HDMID>,
    did_store: HashMap<String, DIDDocument>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            hdmid_store: HashMap::new(),
            did_store: HashMap::new(),
        }
    }
    
    pub fn get_commitment(&self, hdmid: &str) -> Option<String> {
        self.hdmid_store.get(hdmid).map(|h| h.commitment.clone())
    }
    
    pub fn store_hdmid(&mut self, hdmid: HDMID) {
        self.hdmid_store.insert(hdmid.hash.clone(), hdmid);
    }
    
    pub fn get_did(&self, did: &str) -> Option<DIDDocument> {
        self.did_store.get(did).cloned()
    }
    
    pub fn store_did(&mut self, did: &str, doc: DIDDocument) {
        self.did_store.insert(did.to_string(), doc);
    }
}
```

---

## 9. Usage Examples

### Example: Create Commitment

```bash
# Request
curl -X POST http://localhost:8080/api/v1/biometric/commitment \
  -H "Content-Type: application/json" \
  -d '{
    "fingerprint_features": [0.1, 0.2, ...],
    "iris_features": [0.5, 0.6, ...]
  }'

# Response
{
  "did": "did:dmid:3a7f9c2b1e4d8a6f0b5c",
  "hdmid": "a1b2c3d4...",
  "commitment": "pedersen_abc123...",
  "tx_hash": "0x123abc..."
}
```

### Example: Authenticate

```bash
# Request
curl -X POST http://localhost:8080/api/v1/auth \
  -H "Content-Type: application/json" \
  -d '{
    "hdmid": "a1b2c3d4...",
    "proof": {
      "a": "0x...",
      "b": "0x...",
      "c": "0x..."
    },
    "timestamp": 1699999999,
    "verifier_id": "service1"
  }'

# Response
{
  "success": true,
  "session_token": "session_a1b2c3d4_1699999999",
  "message": "Authentication successful"
}
```

### Example: Get DID

```bash
# Request
curl http://localhost:8080/api/v1/did/did:dmid:3a7f9c2b1e4d8a6f0b5c

# Response
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:dmid:3a7f9c2b1e4d8a6f0b5c",
  "verificationMethod": [...],
  "authentication": [...],
  "biometricCommitment": "a1b2c3d4..."
}
```