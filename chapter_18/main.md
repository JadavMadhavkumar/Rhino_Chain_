// main code

# chapter 18 :  zero  Knowledge proofs system 

## 18.1 Groth  16 protocol  selection and  verification
The Groth 16 protocol are based on the concept we call zero Knowaledge proof system,on currect senario there are many progress on both theory and practical on constructing Highly efficient non-interactive arguments with small  bitsize and low authentication complexity, in normal term we call it zk-SNARK, here zk stands for zero knowledge, SNARK stands for Succinct Non-interactive Argument of Knowledge, the groth 16 was best choice for our project there sevral criteria we use in this project form direct form base paper of groth16 1. proof compactness 2. constant verification time 3. mature rust base implementation 4. small proof size  via the bellman crate which is the same language use prives semester phase-1 the bellaman are totale written in rust with AXUM framework that henadel the API flow and security of system, the bellman crate is a rust library for building zk-SNARKs, it provides a high-level interface for creating and verifying zk-SNARK proofs, it also includes a number of pre-built circuits and gadgets that can be used to construct more complex proofs, the bellman crate is designed to be efficient and easy to use, making it a popular choice for developers working on zk-SNARK projects.

## 18.2 Biometric authencation and circuit design 

The ZKP/zk-SNARK circuit design for DMID authentication proves the following statement in zeror knowledege "the Biometric data collect in feature vector in F and Randomness R both are PEDEREN_COMMITMENT(F,R) = C_ON_CHAIN AND SHA-256((F ||  salt)) = HDMID_ON_CHAIN, AND the current timestamp T and verifer_id V are bound to this proof"

The Curcuit is toataly built on the bellman crate's , ConstraintsSystem trait, the main constraints are : 

1. biometric for all feature vector in F and Randomness R are committed to the on-chain C_ON_CHAIN and the commitment is done using PEDEREN_COMMITMENT function, all fucation are implemented in the bellman crate, the commitment is a way to bind the all feature vector and redomness to the on-chain data, it ensure that the biometric data is not tampered with and can be verified on-chain.
2. the SHA-256 hash and salt are used to ensure all intigrity of system and the hash is also bind to the on-chain data HDMID_ON_CHAIN, this make system more secure and resistant to tampering and replay attack that happend on most biometric authentication system, the hash function is a one-way function that takes an input and produces a fixed-size output, it is designed to be irreversible, meaning that it is computationally infeasible to determine the original input from the output, this property makes it ideal for use in authentication systems, as it allows us to verify that the biometric data is correct without revealing the actual data.
3. the timestamp T and auth_verifier_id V are bound to the proof, this ensure that the proof is only valid for a specific time and verifier, it prevent replay attack and ensure that the proof is only used for its intended purpose.


## 18.3 Performance Analysis on ESP-32-S3
The Total performace analysis are base on IoT device ESP-32-S3 that is a low-power microcontroller with wireless connectivity, it is designed for use in IoT applications and it have number features that make whole project stand out, such as dual-core processor, built-in Wi-Fi and Bluetooth, and support for a wide range of peripherals, the performance analysis of the ZKP/zk-SNARK circuit design for DMID authentication on the ESP-32-S3 is based on several factors, including the computational complexity of the circuit, the size of the proof, and the time required to generate and verify the proof.

operation                   Time(ESP-32-S3)         Notes 
Biometric features 
extraction (fingerprint)    100ms - 500ms           Depends on the complexity of biometric data and the sensor used
Bioimetric features(iris)   200ms - 700ms           Gabor phase + TFLM 
Biometric template fusion 
(HDMID)                     15ms - 50ms             SHA-256 in hardware 
ZKP proof                   600ms - 1500ms          BLS12-381 curve, multiple-exponentiation, and pairing operations
total enrollment            720ms - 2250ms          includes biometric (that on-chain TX submission)
Totale auth                 720ms- 2250ms           proof gen + verification 



