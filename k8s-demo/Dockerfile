FROM openjdk:8

WORKDIR /myapp
COPY target/k8s-demo-1.0-SNAPSHOT.jar /myapp

EXPOSE 8000
ENV name Cloud Man


#RUN javac Main.java
CMD ["java", "-cp", "/myapp/k8s-demo-1.0-SNAPSHOT.jar","k8s.Demo"]

