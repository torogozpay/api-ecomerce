ARG RUST_VERSION=1.75
ARG APP_NAME=api-ecomerce

################################################################################
# step 1: build the rust application
FROM --platform=$BUILDPLATFORM rustlang/rust:nightly AS build
#FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /api-ecomerce

COPY . .

RUN apt-get update && apt-get install -y cmake
RUN cargo +nightly build --release


# step 2: create the runtime image
FROM alpine:latest AS final

RUN apk add --update curl cmake 

#RUN echo "nameserver 8.8.8.8" > /etc/resolv.conf && \    
#    command_depending_on_dns_resolution

WORKDIR /api-ecomerce

COPY --from=build /api-ecomerce/target/release . 

# Expose the port that the application listens on.
EXPOSE 8282

#CMD ["/"]