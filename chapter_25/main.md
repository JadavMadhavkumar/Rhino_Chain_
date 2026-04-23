# Chapter 25: Problem Statement and Objectives

## 25.1 Introduction

This chapter clearly defines the problem addressed in Phase 2 of our project and establishes the objectives that guide our implementation.

## 25.2 Background and Problem Context

### 25.2.1 Phase 1 Achievements

Phase 1 of our project successfully implemented:

1. **Hardware System**: ESP32-S3 based edge device with multi-biometric sensors
2. **Biometric Fusion**: Combined fingerprint and iris recognition
3. **Blockchain Integration**: Basic blockchain for identity anchoring
4. **TinyML Models**: Deployed CNN models on edge device

### 25.2.2 Limitations Identified in Phase 1

Despite the achievements, Phase 1 revealed several limitations:

| Limitation | Description | Impact |
|-----------|-------------|--------|
| L1 | No privacy-preserving verification | Raw biometric data exposed |
| L2 | Centralized model training | Privacy risk to users |
| L3 | Limited scalability | Single-node operation |
| L4 | No formal ZKPs | Cannot prove knowledge of biometrics |
| L5 | Weak against presentation attacks | Spoofing vulnerability |

### 25.2.3 Problem Statement

**Main Problem**: 
> "How to provide privacy-preserving, scalable, and secure multi-biometric authentication using zero-knowledge proofs and federated learning on edge devices while maintaining blockchain integrity?"

**Sub-Problems**:
1. How to verify biometric identity without revealing biometric data?
2. How to improve biometric models without centralizing user data?
3. How to scale from single-node to multi-node deployment?
4. How to defend against presentation attacks (liveness detection)?

## 25.3 Research Questions

### 25.3.1 Primary Research Question

**RQ1**: Can a zero-knowledge proof system be implemented on resource-constrained edge devices (ESP32-S3) to enable privacy-preserving biometric authentication?

### 25.3.2 Secondary Research Questions

- **RQ2**: How can federated learning improve biometric model accuracy while preserving user privacy?
- **RQ3**: What is the performance trade-off between security (ZKP) and authentication latency?
- **RQ4**: How does multi-modal liveness detection compare to single-modal approaches?

## 25.4 Objectives

### 25.4.1 Primary Objectives

| ID | Objective | Metric |
|----|-----------|--------|
| O1 | Implement Groth16 ZKP on ESP32-S3 | Verification time < 2s |
| O2 | Create federated learning system | Model accuracy > 90% |
| O3 | Deploy liveness detection model | Detection accuracy > 95% |
| O4 | Build REST API with ZKP endpoints | API latency < 500ms |

### 25.4.2 Secondary Objectives

- **O5**: Integrate W3C DID/VC for identity management
- **O6**: Implement HDMID hash commitment scheme
- **O7**: Create privacy-compliant data architecture (GDPR/PDP Act)
- **O8**: Document system performance metrics

## 25.5 Scope of Work

### 25.5.1 In Scope

1. **ZKP Implementation**
   - Groth16 circuit design
   - Biometric commitment scheme
   - HDMID hash generation

2. **Federated Learning**
   - 5-node simulation
   - SVM-based classifier
   - Differential privacy (σ = 0.01)

3. **Liveness Detection**
   - Passive liveness (texture analysis)
   - SVM classifier
   - TinyML optimization

4. **REST API**
   - Commitment endpoint
   - Authentication endpoint
   - DID management endpoints

### 25.5.2 Out of Scope

1. Mobile application development
2. Production blockchain deployment
3. Hardware security module (HSM) integration
4. Real-world large-scale testing

## 25.6 Hypothesis

**H1**: Zero-knowledge proofs can provide privacy-preserving biometric verification on edge devices without significant performance degradation.

**H2**: Federated learning can improve model accuracy while maintaining differential privacy guarantees.

**H3**: Multi-modal liveness detection provides superior anti-spoofing compared to single-modal approaches.

## 25.7 Methodology Overview

### 25.7.1 Research Design

```
Literature Survey → Problem Definition → Design → Implementation → Testing → Analysis
```

### 25.7.2 Implementation Approach

| Phase | Activity | Tools |
|-------|----------|-------|
| 1 | ZKP Circuit Design | Bellman, Rust |
| 2 | FL Simulation | Flower Framework |
| 3 | Model Training | TensorFlow, Scikit-learn |
| 4 | API Development | Axum, Rust |
| 5 | Testing & Validation | PyTest, Manual |

## 25.8 Expected Outcomes

### 25.8.1 Technical Outcomes

1. Working Groth16 ZKP implementation on ESP32-S3
2. Federated learning framework withprivacy guarantees
3. TinyML liveness detection model
4. REST API with ZKP endpoints

### 25.8.2 Performance Targets

| Metric | Target | Phase 1 Baseline |
|--------|--------|----------------|
| Authentication Time | < 2.5s | 3.2s |
| FAR | < 0.1% | 0.5% |
| FRR | < 2% | 3% |
| Model Accuracy | > 95% | 90% |
| Privacy Score | 100% | 0% |

## 25.9 Summary

This chapter has established:
- The problem context and limitations from Phase 1
- Clear research questions and objectives
- Scope of work boundaries
- Methodology for implementation
- Performance targets

The subsequent chapters detail the implementation of each objective.

---

*Literature Survey continues in Chapter 24*