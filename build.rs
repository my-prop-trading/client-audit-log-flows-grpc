fn main() {
    let url = "https://raw.githubusercontent.com/my-prop-trading/proto-files/main/proto/";
    ci_utils::sync_and_build_proto_file(url, "ClientAuditLogsGrpcService.proto");
}
