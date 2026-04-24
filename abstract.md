# Abstract

## Blockchain-Enabled Multi-Biometric Authentication System using Zero-Knowledge Proofs and Federated Learning

---

The rapid digitization of services has created an unprecedented need for scalable, secure, and user-friendly authentication methods that transcend the limitations of traditional password or single-factor authentication systems. Conventional authentication approaches, including passwords, PINs, and one-time codes, have proven to be vulnerable to numerous attacks such as phishing, credential stuffing, and identity theft. Furthermore, centralized identity management systems create single points of failure, making them attractive targets for attackers seeking to exploit massive repositories of personal data.

This project presents a **Blockchain-Enabled Multi-Biometric Authentication System** that addresses these security challenges through the integration of distributed ledger technology, multi-biometric recognition, zero-knowledge proofs, and federated learning. The proposed system enables users to authenticate themselves using combined biometric modalities—specifically fingerprint and iris scanning—while preserving complete privacy through the implementation of zero-knowledge cryptography.

### Phase 1 Implementation

The first phase of the project established the foundational hardware and software architecture using an ESP32-S3 microcontroller as the edge computing device. The system captures biometric features from fingerprint and iris sensors, which are then fused to create a unified 313-dimensional feature vector. A TinyML-based convolutional neural network processes these features locally on the edge device, enabling real-time authentication without requiring cloud connectivity.

### Phase 2 Enhancements

The second phase significantly extends the system's capabilities through three primary enhancements:

1. **Zero-Knowledge Proof Privacy Layer**: Implementation of the Groth16 zero-knowledge proof protocol enables privacy-preserving biometric verification. Users can prove possession of their biometric credentials without ever revealing the actual biometric data. The system generates a hashed decentralized multi-identity (HDMID) commitment that serves as the user's cryptographic identity anchor, while the original biometric templates remain securely stored only on the user's device.

2. **Federated Learning Integration**: A five-node federated learning simulation, built using the Flower framework, enables collaborative improvement of the biometric recognition model without centralizing sensitive user data. Each node trains local models using differential privacy (σ=0.01) to prevent model inversion attacks, and the FedAvg algorithm aggregates these improvements into a global model that benefits all participants.

3. **Decentralized Identity Management**: Following the W3C DID Core specification, the system implements a custom DID method (did:dmid) that provides users with self-sovereign identity control. Each user's DID document is anchored on the permissioned Rust-based blockchain, creating an immutable audit trail of authentication events.

### Technical Specifications

The system utilizes the following key technologies:

| Component | Technology | Purpose |
|-----------|------------|----------|
| Edge Device | ESP32-S3 | Biometric capture and processing |
| Cryptography | Groth16 (Bellman) | Zero-knowledge proofs |
| Hashing | SHA-256 | HDMID generation |
| Commitment | Pedersen | Biometric commitment |
| Identity | W3C DID/VC | Self-sovereign identifiers |
| Blockchain | Rust | Permissioned ledger |
| ML Framework | TensorFlow Lite | TinyML models |
| Federated Learning | Flower | Distributed training |
| Privacy | DP-SGD | Differential privacy |

### Performance Results

The implemented system achieves the following performance metrics:

- **Authentication Time**: < 2.5 seconds (including biometric capture, ZKP proof generation, and verification)
- **False Acceptance Rate (FAR)**: < 0.1%
- **False Rejection Rate (FRR)**: < 2%
- **Model Accuracy**: 94.1% (federated fusion model)
- **Liveness Detection Accuracy**: > 95%

### Privacy and Compliance

The system is designed with privacy-by-design principles, ensuring compliance with major data protection regulations:

- **Data Minimization**: Only HDMID commitments are stored; raw biometric data never leaves the user's device
- **Purpose Limitation**: Data is used solely for authentication
- **Storage Limitation**: Encrypted off-chain storage with blockchain proof anchoring
- **Right to Erasure**: Users can revoke their DID, removing all links to their biometric identity

The system complies with the European Union's General Data Protection Regulation (GDPR) and India's Digital Personal Data Protection Act, 2023.

### Conclusion

This project demonstrates that privacy-preserving biometric authentication is achievable on resource-constrained edge devices through the clever integration of zero-knowledge proofs, federated learning, and blockchain technology. The system provides a secure, scalable, and privacy-compliant alternative to traditional authentication methods, making it particularly suitable for applications in banking, healthcare, government services, and Web3 ecosystems where identity security is paramount.

**Keywords**: Biometric Authentication, Zero-Knowledge Proofs, Blockchain, Federated Learning, Privacy-Preserving, Edge Computing, Decentralized Identifiers, Groth16, ESP32-S3, Self-Sovereign Identity