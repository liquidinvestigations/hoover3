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

RUN cargo install cargo-binstall \
   && cargo binstall cargo-watch \
   && cargo binstall dioxus-cli@0.6.2 \
   && cargo binstall cargo-nextest

RUN  echo "/usr/local/lib64" > /etc/ld.so.conf.d/openssl.conf && ldconfig

RUN PATH=$PATH:/root/.cargo/bin

RUN dx --version