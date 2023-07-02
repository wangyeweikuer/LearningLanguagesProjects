#!/bin/bash
mvn clean package

docker build -f ./Dockerfile  -t registry.cn-beijing.aliyuncs.com/dla_integration/k8s_demo:1.0.0 .

docker push registry.cn-beijing.aliyuncs.com/dla_integration/k8s_demo:1.0.0

