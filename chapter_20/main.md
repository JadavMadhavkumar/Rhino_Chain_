# Chapter 20 :  Machine Learning  Additional Resources 


## 20.1 The Liveness Detection Challenge 

In the last semester, i Done the first and base phase that also use the Tiny ML model that hendale the Hardware level Fuctions and now second phase that also handing user input and high quality photocopy of fingerprint, this is  secure with presentation attack. 

Liveness detection is a most important part of biometric auth system that determine wether the biometric sample if second layer of securty. 

## 20.2 training a liveness detection model 

I trained a simple model using SVM-based calassifier that i get form keggal and incude , 45 person Both eye (iris), ten hand fingerprint, whole system base SVM model, and i get more then 90% accuracy.

    Postive samples : 45 person Both eye (iris), ten hand fingerprint
    Negative calss(spoof) : 45 person Both eye (iris), ten hand fingerprint
    features : 12 texture features (LBP, HOG, GLCM) and 10 color features (mean, std, skewness, kurtosis) for each semple.

<TABLE> 

## 20.3 Federated Learning Simulation 

Biometric auth sytem are mantain by any singule entity or Organization that help to maintain user base lering for this project, I use the federated learning simulation ,i get all dataset for Keggale and also aply GDPR and PDPA legal framework to ensure Policy around user and the data pravacy.

for this phase i creacted 5 node federated learning setup  using flower framework.

    1. Node pratition :  i user 45 person with all there simple datbset and i split in to 5 node with all 5 them in to 9 group to mantain the balance for model training. 

    2. Local Model :  each node are train local on their own data using SVM-base classifier.

    3. Aggregation :  for whole aggregation i use FeDAvg algorith to combine the local/sytem model to create a goobal model that can be used for all liveness detection.

    4. Privacy :  i applied noise Sigma= 0.01 to maintain the whole privacy on hardware plus user network nodes.

    5. Evaluation :  each group are evaluate the globle model and combine the result i get more then 88% accuracy for phase 2.


<TABALE> 

## 20.4 TyniML and Hardware Integration 

Hardware intigration is most critical part od the project that i added the TinyML model that implemented on small scale on phase 1 and now in the second phase this model is crusal for the liveness detection and also mantain user input and the photocopy of fingerprinHardware intigration is most critical part od the project that i added the TinyML model that implemented on small scale on phase 1 and now in the second phase this model is crusal for the liveness detection and also mantain user input and the photocopy of fingerprint.
1. Model Deployment :  deployment od the model on the edge device that i use Esp32 microcontroller that have limited resource but it most suitable for this projec1. Model Deployment :  deployment od the model on the edge device that i use Esp32 microcontroller that have limited resource but it most suitable for this project.
2. Optimization :  this stage is most critical for whole system and base of architecture that i use quantization and pruning techniques of TinyML to optimize the model for improve the inference time and reduce the memory footrpint/size for the edge device.
3. Integration :  all fucation are intigrate with core system on the hardware level and maintain all peramter for liveness detection.
4. Testing : after the whole setup and integration i test one live demo for the liveness detection.

## 20.5 Conclusion 

implementing the liveness detection system using ML and TinuML is little time consuming and also require a lot of try and error mothode but after intigration it improve whole security layer.



