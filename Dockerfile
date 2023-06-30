FROM rust:slim
COPY ./target/release/client-audit-logs-flows-grpc ./target/release/client-audit-logs-flows-grpc
ENTRYPOINT ["./target/release/client-audit-logs-flows-grpc"]