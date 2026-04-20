# abstract

The phase one is all about building the basic architecture of the system that authenticate user via connection of wired and PCB(personal computer board) there are many security and privacy vulnerability that we resolving in phase 2 that totally working on the server side at server side we working with rust-base proof-of-work blockchain that totally capable to handle security, storing, and cryptography that we build zkp (zero knowledge proof). this phase are totally working REST API that validate all hash-chained blocks, while phase 1 is successfully demonstrating the accessibility and usability of system 


## 17  Phase 2 overview -  ZKP Enhanced Privacy layer 

### 17.1  Introduction

the cental  architecture of the zkp (zreo  knowledge proof) ti improve the security and privacy because user alwase come first , and zkp it back by high level of cryptography that close to crack by any normal device and the zkp  is a powerful tool that can be used to enhance the privacy and security of a wide range of applications, including blockchain, identity management, and secure communication.
there are all block that content user datain JSON that store as vector at at privacy layer also this layer we securing with zkp  to make sure accessbility and security manage by same sever.
there are basicly two main component one is your fingerprint and other one is eye(iris) that any one scane and use as sefty layer to authenticate user , business and computer (not added but there is feature in future that authenticate computer run by whole ai and mpc server). now we understand about DMID (decentralized multi identity management) it use as manage all  identity and authenticate all of them with security and privacy by zkp and also we can use it as identity management for all of our life like health record , financial record and etc.
in authentication layer system verify by 261 float32 for fingerprint and 52 for iris(eye) both of them are in vector from easy to use as large scale data on server side 
this phase 2 we totaly focus on privacy and authentication user on backend and api layer on Phase 1 we build prototype that handling communication with user and PCB(personal computer board) there are on need of server , load banlancer, container, kubernetes(so call high end devOps thing that i'm adding on this project), in simple term there are no need of other thing but at large scale we can't rely on sperate PCB and also user don't know to build PCB or ciruit at there end so we need to move to build scaleble and secure backend and API layer to handle all of this thing and also we need to make sure that all of this thing is secure and private by using zkp as our main tool to achieve this goal.


## 17.2 HDMID identity Hash 

basicaly hash is function that converting normal text to chiper text that rely on human input. one way hash is one way that can't be reverse back to normal text(that given by user) there many hash on market but we choose SHA-256 because it's widely used and also it's secure enough for our use case. we will use this hash to convert user input like name, email, phone number and etc to chiper text that we can store on our server without worrying about data breach or leak because even if someone get access to our database they can't reverse back to normal text without knowing the original input. this is the basic concept of HDMID (hash decentralized multi identity management) that we will use to manage all of our user identity in a secure and private way.

HDMID = SHA-256(CONCATENATE(fingerprint, iris, other_indentity_info))
where: other_identity_info all other infomation that proved by user that depend on user and my building prosess.

## 17.3 W3C  DID Methos Specification : did:dmid 
DID (decentralized identifier) is a totaly new type of identifier that is designed to be decentralized and self-sovereign each block are making with system.  W3C Decentralized Identifiers (DIDs) is a specification that defines a standard path to any user indetifying there information way decentralized system 

DID Syntax :  did:dmid:<methos-specific-id>
example : did:dmid:3a7f9c2b1e4d8a6f0b5c

this method-specific ID is the first 20 hex charecter of the HDMID hash. sufficient uniqeness for(80-bit subject resistance) also keeping the identifier compact.  
the commitment hash is fully compatible with the W3C DID that allowing seamless integration with existing decentralized identity systems and standards. this method-specofic ID is back by HDMID hash. the subject public key (ECDSA P-256), the biometric data(fingerprint and iris) apart of this method-specific ID/block contained biometric commitment hash, enrollment timestamp, and the associated verification method that credential identifiers.

## 17.4 ZKP (Zero Kowledge proof) for privcy and security 

This section are toatly workign on  security layer that make more reliable on server side , also we look in ZKP and there method and clerify how combining ZKP with our system to enhance the security of whole project 



