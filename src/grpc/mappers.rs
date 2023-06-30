
impl Into<crate::client_audit_logs_grpc::ClientAuditLog> for crate::postgres::dto::ClientAuditLogDto {
    fn into(self) -> crate::client_audit_logs_grpc::ClientAuditLog {
        crate::client_audit_logs_grpc::ClientAuditLog {
            client_id: self.client_id,
            id: self.id,
            created_at: self.created_at.unix_microseconds,
            new_context: self.new_context,
            prev_context: self.prev_context,
            user_id: self.user_id,
            r#type: self.log_type as i32,
        }
    }
}