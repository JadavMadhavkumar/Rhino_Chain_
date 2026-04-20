# Chapter 21 :  System Architecture and Security Analysis 


## 21.1 Overall  System Architecture 

Overall System Architeture are three sprate layers that proved security , tranparency and reliability to the system to make sure the syetm is scalable for long run on identity management and data management system. on the three layers also include hardware layer so Build WebUSB base harware system where user can access biometric via web browser and USB device to make more user friendly.

on the first phase i build whole ESP32-S3 base sytem to support chain for RUST blockchain node , now i move to the scond level where i use block structure and DID/VC endpoints. on each node there will be ZKP proof mechanism.

there will only singale way data flow for users biometric to make it more secure there will no way any other data flow for transfering biometric.

whole API system are built on REST API and GraphQL API to make whole architecture accurate and faster on both side on user and devloper side. now you have douts about there will transfering vai API so answer is yes but only DID Documents and ZKP proof not any other data will pass by API now you have douts about there will transfering vai API so answer is yes but only DID Documents and ZKP proof not any other data will pass by API.

## 21.2 Threat Model

<Table of Threats>


## 21.3 Privacy Compliance 

the phase 2 are base on whole compliance and GDPR, CCPA and Pravacy on design and development on such way it will not harm user on any side.  i made few point that make system by it own law and regulation.


1. Data Minimization :  The System only collect two type biometric that size will 192 byte on top of that it will  on ZKP proof and DID document that will not not have any personal data of user.

2. any tipe of data that store off chain will be encrypted and hashed to make sure that data is secure and only user can use that data.

3. storage limitation :  raw biometric data  never be accessible to user or third party and it will never be repicated other vise user cradntial are remove from the system.

:
