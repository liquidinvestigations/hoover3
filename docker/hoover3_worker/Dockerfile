FROM rust:1-bullseye

RUN apt-get -y update && apt-get install -y \
    clang \
    libmagic1 \
    libmagic-dev protobuf-compiler

RUN apt-get install -y zip curl bash

RUN bash -ex \
 -c ' ( curl -s "https://get.sdkman.io"  | bash )  && source /root/.sdkman/bin/sdkman-init.sh && sdk install java 23.0.1-graalce'
ENV GRAALVM_HOME=/root/.sdkman/candidates/java/23.0.1-graalce
ENV PATH=$GRAALVM_HOME/bin:$PATH
RUN java -version
RUN chmod a+rwx /root/ && chmod -R a+rwx /root/.sdkman/