# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM busybox:latest AS env
ARG TARGETPLATFORM
RUN --mount=type=bind,source=artifact,target=artifact \
    if [ "$TARGETPLATFORM" = 'linux/amd64' ]; then\
      cp artifact/x86_64-unknown-linux-musl/role-adder-for-new-member .\
    ;elif [ "$TARGETPLATFORM" = 'linux/arm/v6' ]; then\
      cp artifact/arm-unknown-linux-gnueabihf/role-adder-for-new-member .\
    ;elif [ "$TARGETPLATFORM" = 'linux/arm/v7' ]; then\
      cp artifact/armv7-unknown-linux-gnueabihf/role-adder-for-new-member .\
    ;elif [ "$TARGETPLATFORM" = 'linux/arm64' ]; then\
      cp artifact/aarch64-unknown-linux-musl/role-adder-for-new-member .\
    ;fi

FROM scratch
COPY --from=env --chmod=755 role-adder-for-new-member .
ENTRYPOINT ["./role-adder-for-new-member"]
