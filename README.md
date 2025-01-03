**In Progress**

	 	 	   ----------------------------------------------------------------
	     	    	     ** Simple Kubernetes (K8s) Controller using shell script **
	  	 	   ----------------------------------------------------------------


```text
Functionality/Mechanism
------------------------
The controller watches events from the api server targetting change in configmap. When the annotation in configmap is modified with words that match with the label of the pod, the controller would request the list of pods containing that label to the api server, and then delete those pods.
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
