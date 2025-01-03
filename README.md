Progress**

	 	 	   ---------------------------------------------------------------------------------------------------------
	     	    	     ** Container Orchestrator Kubernetes (K8s) Internals and implementing basic Container orchestrator **
	  	 	   ---------------------------------------------------------------------------------------------------------


```text
Container Orchestrator such as Kubernetes, etc, are basically system that manages containers such as deploying, scaling , etc and also provide automation features. It usually consists of
```

```text
Implementation
---------------
1) Create service account
kubectl create sa <sa name> -n test-namespace
2) Create role (what actions are allowed)
apiVersion: 
kind: Role
metadata:
