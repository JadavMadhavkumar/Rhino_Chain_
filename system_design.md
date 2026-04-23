# System Design Document - Phase 2

## Project: Blockchain-Enabled Multi-Biometric Authentication System with ZKP and Federated Learning

---

## 1. Overview Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                    BLOCKCHAIN-ENABLED MULTI-BIOMETRIC AUTHENTICATION      │
│                           PHASE 2 SYSTEM                             │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐ │
│  │   USER (Edge)    │     │   BLOCKCHAIN     │     │   FEDERATED      │ │
│  │   Device        │     │   Network        │     │   Learning      │ │
│  │   ESP32-S3      │     │   (Rust-based)   │     │   Server        │ │
│  └────────┬────────┘     └────────┬────────┘     └────────┬────────┘ │
│           │                        │                        │           │
│           │  1. Biometric Scan    │  2. Store HDMID        │ 3. Update │ │
│           │  2. Feature Extract   │  3. Verify Proof      │    Model  │ │
│           │  3. Generate ZKP      │                        │           │ │
│           │                      │                        │           │ │
│           └──────────┬───────────┴────────┬───────────────┘           │ │
│                      │                    │                           │ │
│              ┌───────▼───────┐    ┌───────▼───────┐                 │ │
│              │  REST API    │    │  DID/VC      │                 │ │
│              │  Server     │    │  Registry   │                 │ │
│              │  (Axum)     │    │             │                 │ │
│              └─────────────┘    └─────────────┘                 │ │
│                                                               │ │
└───────────────────────────────────────────────────────────────┘ │
```

---

## 2. Component Diagram

```
┌────────────────────────────────────────────────────────────────────────────────────┐
│                         USER LAYER                                    │
│  ════════════════════════════════════════════════════════════════        │
│                                                                     │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐               │
│  │ Fingerprint│    │    Iris    │    │   Web      │               │
│  │  Sensor   │    │   Scanner  │    │  Browser  │               │
│  └─────┬─────┘    └─────┬─────┘    └─────┬─────┘               │
│        │                │                │                            │
│        └────────┬───────┴──────┬────────┘                            │
│                 │            │                                    │
│         ┌───────▼────────────▼───────┐                              │
│         │   ESP32-S3 Edge Device   │                              │
│         │ + Biometric Fusion    │                              │
│         │ + Liveness Detection │                              │
│         │ + ZKP Generation    │                              │
│         └──────────┬───────────┘                              │
└────────────────────┼──────────────────────────────────────────────┘
                     │
                     │ HTTPS/WiFi
                     │
                     ▼
```

---

## 3. Block Diagram - Phase 2 Components

```
                            ┌─────────────────────────────────────────┐
                            │         BLOCKCHAIN NETWORK            │
                            │   ┌─────────────────────────────┐   │
                            │   │  Permissioned (Rust)        │   │
                            │   │  ┌────┐ ┌────┐ ┌────┐     │   │
                            │   │  │Node│ │Node│ │Node│     │   │
                            │   │  └────┘ └────┘ └────┘     │   │
                            │   └─────────────────────────────┘   │
                            └─────────────────┬───────────────────┘
                                              │
                    ┌───────────────────────────┼───────────────────┐
                    │                          │                    │
                    ▼                          ▼                    ▼
        ┌───────────────────────┐  ┌───────────────────────┐  ┌───────────────────────┐
        │   REST API Server      │  │   DID Operations    │  │  FL Aggregator     │
        │   (Axum/Rust)     │  │   Handler         │  │  (Flower)        │
        │                   │  │                 │  │                 │
        │ ┌─────────────┐  │  │ ┌───────────┐ │  │ ┌───────────┐│
        │ │/commit    │  │  │ │Create DID│ │  │ │Local Model││
        │ │/auth     │  │  │ │Verify   │ │  │ │Aggregation││
        │ │/did      │  │  │ │Revoke   │ │  │ │Privacy   ││
        │ │/vc       │  │  │ └────────┘ │  │ └──────────┘│
        │ └───────────┘  │  └─────────────┘  └───────────────┘
        └───────────────┘  └────────────────┘  └─────────────────────┘
```

---

## 4. Data Flow Diagram - Authentication

```
┌──────────────────────────────────────────────────────────────────────────┐
│                 AUTHENTICATION FLOW                     │
│  User Enrollment and Authentication Process         │
├──────────────────────────────────────────────────────────────────────────┤
│                                                 │
│  ┌──────────────────────────────────────────┐    │
│  │         1. ENROLLMENT PHASE              │    │
│  └────────────────┬───────────────────────┘    │
│                   │                              │
│  ┌───────────────▼──────────────────────┐     │
│  │   User scans biometrics               │     │
│  │   - Fingerprint (261 dim)            │     │
│  │   - Iris (52 dim)                 │     │
│  └───────────────┬───────────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Feature Fusion                  │     │
│  │   F = [F_fingerprint || F_iris]  │     │
│  └───────────────┬───────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Generate HDMID                  │     │
│  │   HDMID = SHA256(F || salt)       │     │
│  └───────────────┬───────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Pedersen Commitment              │     │
│  │   C = Pedersen(F, r)             │     │
│  └───────────────┬───────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Generate ZKP Proof              │     │
│  │   π = ZKP.Prove(witness)         │     │
│  └───────────────┬───────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Create DID                   │     │
│  │   did:dmid:xxxxxxxx           │     │
│  └───────────────┬───────────────────┘     │
│                │                              │
│  ┌─────────────▼──────────────────────┐     │
│  │   Store on Blockchain          │     │
│  │   - DID Document              │     │
│  │   - HDMID Hash               │     │
│  │   - Commitment Proof         │     │
│  └─────────────────────────────┘     │
│                                        │
│  ┌─────────────────────────────────┐ │
│  │       2. AUTHENTICATION PHASE   │ │
│  └────────────────┬────────────────┘ │
│                   │                    │
│  ┌───────────────▼─────────────────┐  │
│  │   User scans biometrics        │  │
│  └───────────────┬─────────────────┘  │
│                │                       │
│  ┌─────────────▼──────────────────┐   │
│  │   Generate new proof π'        │   │
│  └───────────────┬────────────────┘   │
│                │                       │
│  ┌─────────────▼──────────────────┐   │
│  │   API: POST /auth              │   │
│  │   - HDMID                    │   │
│  │   - ZKP Proof π'             │   │
│  └───────────────┬────────────────┘   │
│                │                       │
│  ┌─────────────▼──────────────────┐   │
│  │   Verify:                   │   │
│  │   1. Check timestamp       │   │
│  │   2. Verify ZKP          │   │
│  │   3. Match HDMID          │   │
│  └───────────────┬────────────────┘   │
│                │                       │
│  ┌─────────────▼──────────────────┐   │
│  │   Return: Success/Failure     │   │
│  └─────────────────────────────┘   │
│                                        │
└────────────────────────────────────────┘
```

---

## 5. ZKP Circuit Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ZKP GROTH16 CIRCUIT                      │
│              Biometric Authentication Proof                │
├─────────────────────────────────────────────────────────────────────┤
│                                                      │
│  Input:                                              │
│  ┌──────────────────────────────────────────────┐     │
│  │ Public Inputs:                               │     │
│  │  - C_on_chain (Pedersen Commitment)       │     │
│  │  - HDMID_on_chain (SHA256 Hash)        │     │
│  │  - T (timestamp)                  │     │
│  │  - V (verifier_id)              │     │
│  ├──────────────────────────────────────────┤     │
│  │ Private Inputs (Witness):                │     │
│  │  - F (biometric features)              │     │
│  │  - r (randomness)                  │     │
│  │  - salt                          │     │
│  └──────────────────────────────────────────┘     │
│                         │                         │
│                         ▼                         │
│  ┌─────────────────────────────────────────┐     │
│  │        CONSTRAINT SYSTEM                 │     │
│  │                                   │     │
│  │  1. Commitment Constraint:          │     │
│  │     Pedersen(F, r) == C_on_chain   │     │
│  │                                   │     │
│  │  2. Hash Constraint:              │     │
│  │     SHA256(F || salt) == HDMID     │     │
│  │                                   │     │
│  │  3. Timestamp Constraint:         │     │
│  │     T_current - T_enrolled < T  │     │
│  │                                   │     │
│  └────────────────┬─────────────────┘     │
│                   │                         │
│                   ▼                         │
│  ┌─────────────────────────────────┐       │
│  │     Output: ZKP Proof π        │       │
│  │     [A]₁, [B]₂, [C]₁      │       │
│  └─────────────────────────────────┘       │
│                                                      │
└──────────────────────────────────────────────────────┘
```

---

## 6. Federated Learning Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│         FEDERATED LEARNING SYSTEM          │
│          5-Node Simulation            │
├────────────────────────────────────────┤
│                                       │
│    ┌─────────────────────┐            │
│    │   Central Server   │            │
│    │  (FL Aggregator)  │            │
│    └────────┬──────────┘            │
│             │                        │
│    ┌────────┼────────┐               │
│    │        │        │               │
│    ▼        ▼        ▼               │
│ ┌─────┐  ┌─────┐  ┌─────┐          │
│ │Node│  │Node│  │Node│          │
│ │ 1  │  │ 2  │  │ 3  │           │
│ └──┬─┘  └──┬─┘  └──┬─┘          │
│    │       │       │                │
│    └───────┴───────┘                │
│             │                      │
│             ▼                      │
│    ┌────────────────┐             │
│    │   4. FL Client│             │
│    │  5. FL Client│             │
│    └────────────────┘             │
│                                       │
│  Data Flow:                          │
│  1. Server sends global model θ      │
│  2. Each client trains locally       │
│    - Local gradients               │
│    - Add DP noise (σ=0.01)        │
│  3. Clients send model updates    │
│  4. Server aggregates (FedAvg)  │
│  5. Updated global model         │
│                                       │
└─────────────────────────────────────┘
```

---

## 7. Complete System Architecture

```
                                    ┌──────────────────────────────┐
                                    │      BLOCKCHAIN LAYER       │
                                    │  ┌─────────┐ ┌─────────┐   │
                                    │  │ Block N │ │Block N-1│   │
                                    │  │ HDMID  │ │ HDMID  │   │
                                    │  │  DID   │ │  DID   │   │
                                    │  │  VC    │ │  VC    │   │
                                    │  └───────┘ └───────┘       │
                                    └────────────┬───────────────┘
                                               │
                              ┌────────────────┬─┴────────────────┐
                              │                │                  │
                              ▼                ▼                  ▼
                    ┌─────────────────┐ ┌───────��─��───────┐ ┌─────────────────┐
                    │  REST API      │ │  DID Registry │ │ FL Aggregator  │
                    │  (Axum)     │ │              │ │  (Flower)    │
                    │             │ │              │ │              │
                    │ /commit    │ │ Create DID  │ │ Global Model │
                    │ /auth     │ │ Verify     │ │ Local Train │
                    │ /did/:id  │ │ Revoke     │ │ Aggregate   │
                    │ /vc       │ │ Update    │ │ Privacy    │
                    └─────┬─────┘ └─────┬──────┘ └──────┬──────┘
                         │            │                 │
                         └────────────┼─────────────────┘
                                    │
                                    ▼
                    ┌─────────────────────────────────────────┐
                    │           EDGE DEVICE LAYER              │
                    │           ESP32-S3                    │
                    │  ┌────────────────────────────────┐    │
                    │  │      APPLICATION LAYER         │    │
                    │  │  ┌─────┐ ┌─────┐ ┌─────┐    │    │
                    │  │  │ZKP │ │Fusion│ │TinyML│    │    │
                    │  │  │    │ │     │ │     │    │    │
                    │  │  └─────┘ └─────┘ └─────┘    │    │
                    │  ├──────────────────────────────┤    │
                    │  │      CRYPTO LAYER              │    │
                    │  │  SHA-256 │ Pedersen │ Groth16│    │
                    │  ├──────────────────────────────┤    │
                    │  │      HARDWARE LAYER           │    │
                    │  │ Sensor │ Crypto │ WiFi   │    │
                    │  └──────────────────────────────┘    │
                    └─────────────────────────────────────┘
                                    │
                                    ▼
                    ┌──────────────────────────────────┐
                    │         USER INTERFACE            │
                    │  Fingerprint │ Iris │ WebBrowser │
                    └──────────────────────────────────┘
```

---

## 8. API Endpoints

```
┌────────────────────────────────────────────────────────────────┐
│                 REST API ENDPOINTS                 │
├────────────────────────────────────────────────────────────────┤
│                                                        │
│  Base URL: http://localhost:8080/api/v1                │
│                                                        │
│  ┌────────────────────────────────────────────────┐   │
│  │ AUTHENTICATION ENDPOINTS                       │   │
│  ├──────────────────────────────────────────────┤   │
│  │ POST   /auth                          │ Authenticate       │
│  │         Body: { hdmid, proof, timestamp }        │   │
│  │         Response: { success, session_token }   │   │
│  └──────────────────────────────────────────────┘   │
│                                                        │
│  ┌────────────────────────────────────────────────┐   │
│  │ COMMITMENT ENDPOINTS                           │   │
│  ├──────────────────────────────────────────────┤   │
│  │ POST   /biometric/commitment       │ Enroll HDMID    │
│  │         Body: { commitment, hdmid, proofs }  │   │
│  │         Response: { did, tx_hash }            │   │
│  │                                             │   │
│  │ POST   /biometric/update           │ Update HDMID    │
│  │         Body: { new_commitment, proofs }       │   │
│  │         Response: { did, tx_hash }            │   │
│  └──────────────────────────────────────────────┘   │
│                                                        │
│  ┌────────────────────────────────────────────────┐   │
│  │ DID ENDPOINTS                                │   │
│  ├──────────────────────────────────────────────┤   │
│  │ GET    /did/:id                    │ Get DID Doc   │
│  │         Response: { did_document }             │   │
│  │                                             │   │
│  │ POST   /did                        │ Create DID  │
│  │         Body: { public_key, commitment }     │   │
│  │         Response: { did, document }          │   │
│  │                                             │   │
│  │ POST   /did/revoke                 │ Revoke DID  │
│  │         Body: { did, reason }                │   │
│  │         Response: { status, tx_hash }          │   │
│  └──────────────────────────────────────────────┘   │
│                                                        │
│  ┌────────────────────────────────────────────────┐   │
│  │ CREDENTIAL ENDPOINTS                         │   │
│  ├──────────────────────────────────────────────┤   │
│  │ POST   /vc/issue                 │ Issue VC     │
│  │         Body: { did, claims, issuer }       │   │
│  │         Response: { vc, tx_hash }           │   │
│  └──────────────────────────────────────────────┘   │
│                                                        │
└────────────────────────────────────────────────────────┘
```

---

## 9. Database Schema

```
┌────────────────────────────────────────────────────────────────┐
│                    DATABASE SCHEMA                  │
├────────────────────────────────────────────────────────────────┤
│                                                        │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ OFF-CHAIN DATABASE (Encrypted)                        │    │
│  ├──────────────────────────────────────────────────────┤    │
│  │                                                    │    │
│  │ users table                                        │    │
│  │ ┌──────────┬──────────┬──────────┬──────────────┐  │    │
│  │ │ id       │ did      │ hdmid    │ salt       │  │    │
│  │ │ UUID    │ VARCHAR  │ VARCHAR  │ VARCHAR    │  │    │
│  │ │ PK     │ UNIQUE   │ INDEX   │ NOT NULL   │  │    │
│  │ └──────────┴──────────┴──────────┴──────────────┘  │    │
│  │                                                    │    │
│  │ biometric_data table                               │    │
│  │ ┌──────────┬──────────────┬────────────────────┐ │    │
│  │ │ user_id  │ fingerprint   │ iris              │ │    │
│  │ │ UUID    │ BLOB (encrypted)│ BLOB (encrypted) │ │    │
│  │ │ FK      │               │                  │ │    │
│  │ └──────────┴──────────────┴────────────────────┘ │    │
│  │                                                    │    │
│  │ commitments table                                 │    │
│  │ ┌──────────┬──────────────┬─────────────┬────────┐ │    │
│  │ │ id       │ commitment  │ created_at │ status │ │    │
│  │ │ UUID    │ VARCHAR     │ TIMESTAMP  │ ENUM   │ │    │
│  │ │ PK     │ NOT NULL    │ NOT NULL   │ ACTIVE │ │    │
│  │ └──────────┴──────────────┴─────────────┴────────┘ │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                        │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ ON-CHAIN (Blockchain Blocks)                          │    │
│  ├──────────────────────────────────────────────────────┤    │
│  │                                                    │    │
│  │ Block Structure:                                  │    │
│  │ ┌────────────────────────────��─��───────────────┐   │    │
│  │ │ index: uint64                                │   │    │
│  │ │ timestamp: uint64                           │   │    │
│  │ │ data: {                                    │   │    │
│  │ │   did: String,                             │   │    │
│  │ │   hdmid_hash: String,                     │   │    │
│  │ │   commitment_proof: String,              │   │    │
│  │ │   vc_root: String                         │   │    │
│  │ │ }                                       │   │    │
│  │ │ previous_hash: String                    │   │    │
│  │ │ hash: String                            │   │    │
│  └──────────────────────────────────────────────┘   │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                        │
└──────────────────────────────────────────────────────────┘
```

---

## 10. Security Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                 SECURITY LAYERS                    │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Layer 1: Data Security                            │
│  ┌─────────────────────────────────────────────┐     │
│  │ - Biometric data encrypted (AES-256)       │     │
│  │ - Only HDMID hash stored                   │     │
│  │ - Pedersen commitment for verification  │     │
│  └─────────────────────────────────────────────┘     │
│                                                     │
│  Layer 2: Cryptographic Security                   │
│  ┌─────────────────────────────────────────────┐     │
│  │ - SHA-256 for hashing                   │     │
│  │ - Groth16 ZKP for proof               │     │
│  │ - Elliptic curve cryptography         │     │
│  │ - TLS for transport                 │     │
│  └─────────────────────────────────────────────┘     │
│                                                     │
│  Layer 3: Privacy Security                        │
│  ┌─────────────────────────────────────────────┐     │
│  │ - Zero-knowledge proofs                  │     │
│  │ - Differential privacy (σ=0.01)       │     │
│  │ - No raw biometric transmission        │     │
│  └─────────────────────────────────────────────┘     │
│                                                     │
│  Layer 4: Access Control                         │
│  ┌───────────────────────────────────────��─��───┐     │
│  │ - DID-based authentication               │     │
│  │ - Token-based session management       │     │
│  │ - Rate limiting                     │     │
│  └─────────────────────────────────────────────┘     │
│                                                     │
│  Layer 5: Liveness Detection                    │
│  ┌─────────────────────────────────────────────┐     │
│  │ - Texture analysis (fingerprint)         │     │
│  │ - Pupil detection (iris)                │     │
│  │ - TinyML model on edge                  │     │
│  └─────────────────────────────────────────────┘     │
│                                                     │
└─────────────────────────────────────────────────────┘
```