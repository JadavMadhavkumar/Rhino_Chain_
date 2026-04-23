# Chapter 24: Literature Survey

## 24.1 Introduction

This chapter presents a comprehensive survey of the literature relevant to Phase 2 of our project: Blockchain-Enabled Multi-Biometric Authentication System using Zero-Knowledge Proofs and Federated Learning. The survey covers nine key research areas that form the theoretical foundation of our implementation.

The objective of this literature survey is to understand the current state of research in privacy-preserving biometric authentication, zero-knowledge cryptography, federated learning for biometrics, and decentralized identity management.

---

## 24.2 Zero-Knowledge Proofs and Groth16 Protocol

### 24.2.1 Background on Zero-Knowledge Proofs

Zero-knowledge proofs (ZKPs) are cryptographic protocols that allow one party (the prover) to convince another party (the verifier) that a statement is true without revealing any information beyond the validity of the statement itself. The concept was first introduced by Goldwasser, Micali, and Rackoff in 1989.

In biometric authentication, ZKPs enable a user to prove possession of biometric credentials without revealing the biometric data itself. This property is valuable because biometric traits are immutable and cannot be changed if compromised.

### 24.2.2 Groth16 Protocol

The Groth16 protocol, introduced by Jens Groth in 2016 at EUROCRYPT, represents a significant advancement in zero-knowledge proof systems. The protocol provides:

- **Proof Size**: Only 3 group elements, providing extremely compact proofs
- **Verification Time**: Constant (O(1)), regardless of circuit complexity
- **Setup Requirement**: Per-circuit trusted setup

The mathematical foundation of Groth16 is based on bilinear pairings on elliptic curves. The protocol transforms a computation into a Quadratic Arithmetic Program (QAP), which can then be proven using the ZKP system.

**Importance for Our Project**: We selected Groth16 because its small proof size makes it suitable for blockchain transactions, and its constant verification time is critical for real-time authentication on edge devices.

---

## 24.3 Decentralized Identifiers (DIDs) and W3C Standards

### 24.3.1 Introduction to DIDs

Decentralized Identifiers (DIDs) represent a new paradigm for digital identity management, as defined by the W3C DID Core specification (2022). Unlike traditional centralized identifiers, DIDs are:

- **Decentralized**: No central authority controls the identifier
- **Persistent**: The identifier persists without requiring central management
- **Cryptographically Verifiable**: Ownership can be proven cryptographically

### 24.3.2 W3C DID Core Specification

The W3C DID Core specification (2022), developed by Sporny, Longley, and Sabadello et al., provides:

- **DID Syntax**: `did:method:specific-identifier`
- **DID Document**: JSON-LD document containing verification methods
- **DID Methods**: Interoperable methods for creating and managing DIDs

### 24.3.3 Verifiable Credentials

The W3C Verifiable Credentials Data Model (2022) provides a standard format for credential issuance and verification, allowing issuers to make claims about subjects verifiable by third parties.

---

## 24.4 Federated Learning for Biometric Model Training

### 24.4.1 Introduction to Federated Learning

Federated Learning, introduced by McMahan et al. at Google in 2017, enables models to be trained across decentralized data sources. Instead of centralizing data, federated learning allows each client to train locally and share only model updates.

This approach addresses privacy concerns in biometric systems where centralized data collection creates significant risks.

### 24.4.2 FedAvg Algorithm

The Federated Averaging (FedAvg) algorithm is the most widely used federated learning algorithm. The aggregation formula is:

$$\theta_{global} = \sum_{k=1}^{K} \frac{n_k}{n} \cdot theta_k$$

Where $n_k$ is the number of samples at client $k$, and $\theta_k$ is the locally trained model.

**Importance for Our Project**: We implement federated learning to improve our biometric fusion model without centralizing biometric data.

### 24.4.3 Flower Framework

The Flower Framework (Beutel et al., 2020) provides an extensible toolkit for federated learning research. We utilize Flower for our federated learning simulation.

---

## 24.5 Differential Privacy in Machine Learning

### 24.5.1 Concept of Differential Privacy

Differential privacy, introduced by Dwork et al. in 2006, provides a mathematical framework for quantifying privacy guarantees. It ensures that algorithm output does not reveal information about any individual in the dataset.

### 24.5.2 DP-SGD Algorithm

The Differentially Private Stochastic Gradient Descent (DP-SGD) algorithm, introduced by Abadi et al. (2016), applies differential privacy to deep learning through:

1. **Gradient Clipping**: Bound the norm of gradients to limit individual influence
2. **Noise Addition**: Add Gaussian noise to clipped gradients
3. **Privacy Accounting**: Track cumulative privacy loss using privacy budget ($\epsilon$)

**Importance for Our Project**: We apply differential privacy with $\sigma = 0.01$ noise to prevent model inversion attacks.

---

## 24.6 Cryptographic Primitives and Pedersen Commitments

### 24.6.1 Pedersen Commitment Scheme

The Pedersen commitment scheme, introduced by Pedersen in 1991, allows committing to a value while keeping it hidden. The scheme is binding (cannot change value after committing) and hiding (value is hidden until revealed).

For message $m$ and randomness $r$:

$$Commit(m, r) = g^m \cdot h^r \mod p$$

**Importance for Our Project**: We use Pedersen commitments in our HDMID scheme. Biometric feature vectors are committed using Pedersen commitment during enrollment.

### 24.6.2 Symmetric-Key Primitives

Aly et al. (2020) provide a comprehensive survey of symmetric-key primitives including SHA-256 for hashing and AES for encryption.

---

## 24.7 Biometric Recognition Systems

### 24.7.1 Introduction to Biometric Recognition

Jain et al. (2004) provide a comprehensive introduction to biometric recognition systems, covering:

- **Physiological Biometrics**: Fingerprint, iris, face, hand geometry
- **Behavioral Biometrics**: Voice, signature, keystroke dynamics

### 24.7.2 Multi-Biometric Systems

Our system combines fingerprint (261-dimensional feature vector) and iris (52-dimensional feature vector) for:

- Higher accuracy through complementary information
- Improved robustness against spoofing attacks
- Better handling of noisy captures

### 24.7.3 Spoofing Attacks and Presentation Attack Detection

Biggio et al. (2012) demonstrate that presentation attacks can defeat many biometric systems. This motivates our liveness detection implementation.

---

## 24.8 ESP32-S3 Hardware Platform

### 24.8.1 ESP32-S3 Technical Specifications

The ESP32-S3 microcontroller (Espressif Systems, 2023) provides:

- **Processor**: Dual-core RISC-V @ 240 MHz
- **Memory**: 512 KB SRAM, 8 MB Flash
- **Connectivity**: Wi-Fi + Bluetooth 5.0
- **Cryptographic Hardware**: AES, SHA, RSA accelerators

### 24.8.2 Suitability for Edge AI

The ESP32-S3 is suitable for edge AI due to:

- Hardware cryptographic accelerators for fast SHA-256 operations
- Sufficient RAM for small neural network inference
- Low power consumption
- Integrated Wi-Fi for connectivity

---

## 24.9 Privacy Regulations and Compliance

### 24.9.1 General Data Protection Regulation (GDPR)

The GDPR (2016) establishes requirements for personal data processing:

- **Data Minimization**: Only collect necessary data
- **Purpose Limitation**: Use data only for stated purposes
- **Storage Limitation**: Delete data when no longer needed
- **Integrity and Confidentiality**: Protect data through security measures

### 24.9.2 Digital Personal Data Protection Act (India)

India's Digital Personal Data Protection Act (2023) establishes similar requirements including:

- **Data Fidinary**: Entity determining purpose of processing
- **Data Principal**: Individual whose data is processed
- **Consent**: Explicit, informed agreement

---

## 24.10 Summary and Gap Analysis

### 24.10.1 Literature Summary

| Area | Key Paper | Contribution |
|------|-----------|--------------|
| Zero-Knowledge Proofs | Groth (2016) | Constant-size ZKPs |
| Decentralized Identity | W3C DID Core (2022) | Self-sovereign identifiers |
| Federated Learning | McMahan (2017) | Privacy-preserving training |
| Differential Privacy | Abadi (2016) | DP-SGD algorithm |
| Pedersen Commitment | Pedersen (1991) | Binding/hiding commitments |
| Biometrics | Jain (2004) | Foundation of recognition |
| Security | Biggio (2012) | Spoofing attack analysis |
| Hardware | ESP32-S3 (2023) | Edge computing platform |
| Regulations | GDPR/PDP Act | Privacy compliance |

### 24.10.2 How Phase 2 Addresses These Gaps

Phase 2 addresses existing research gaps by:

1. Implementing Groth16 on ESP32-S3 edge devices
2. Combining federated learning with differential privacy
3. Using blockchain for immutable authentication audit trail
4. Ensuring privacy by design for GDPR/PDP Act compliance

---

## References

[26] J. Groth, "On the Size of Pairing-Based Non-interactive Arguments," EUROCRYPT 2016, Lecture Notes in Computer Science, Vol. 9666, pp. 305–326.

[27] M. Sporny, D. Longley, M. Sabadello et al., "Decentralized Identifiers (DIDs) v1.0," W3C Recommendation, July 2022.

[28] M. Sporny, D. Longley, D. Chadwick, "Verifiable Credentials Data Model v1.1," W3C Recommendation, March 2022.

[29] H. B. McMahan et al., "Communication-Efficient Learning of Deep Networks from Decentralized Data," AISTATS 2017, pp. 1273–1282.

[30] M. Abadi et al., "Deep Learning with Differential Privacy," ACM CCS 2016, pp. 308–318.

[31] D. J. Beutel et al., "Flower: A Friendly Federated Learning Research Framework," arXiv:2007.14390, 2020.

[32] A. Aly et al., "Design of Symmetric-Key Primitives for Advanced Cryptographic Protocols," IACR Transactions on Symmetric Cryptology, 2020(3).

[33] T. Pedersen, "Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing," CRYPTO 1991, Lecture Notes in Computer Science, Vol. 576, pp. 129–140.

[36] A. K. Jain, A. Ross, S. Prabhakar, "An Introduction to Biometric Recognition," IEEE Transactions on Circuits and Systems for Video Technology, Vol. 14, No. 1, 2004, pp. 4–20.

[37] B. Biggio et al., "Security Evaluation of Biometric Authentication Systems Under Real Spoofing Attacks," IET Biometrics, Vol. 1, Issue 1, 2012, pp. 11–24.

[38] Espressif Systems, "ESP32-S3 Technical Reference Manual v1.4," 2023.

[39] Government of India, "The Digital Personal Data Protection Act, 2023."

[40] European Parliament and Council, "General Data Protection Regulation (GDPR)," Regulation (EU) 2016/679, 2016.