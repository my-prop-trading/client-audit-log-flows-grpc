service_sdk::macros::use_my_postgres!();
use encryption::aes::AesKey;

use super::{encrypt_field, decrypt_field};

#[derive(Debug, Clone, PartialEq, Eq, DbEnumAsI32)]
pub enum ClientAuditLogTypeDto {
    #[enum_case(id = 0)]
    ChangedPersonalData = 0,
    #[enum_case(id = 1)]
    Registered = 1,
    #[enum_case(id = 2)]
    LoggedIn = 2,
}

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema, Clone,)]
pub struct ClientAuditLogDto {
    #[primary_key(0)]
    pub id: String,
    #[db_index(id=2, index_name: "client_audit_logs_client_id_idx", is_unique: false, order: "DESC")]
    pub client_id: String,
    #[sql_type("timestamp")]
    #[order_by_desc]
    #[db_index(id=1, index_name: "client_audit_logs_created_at_idx", is_unique: false, order: "DESC")]
    pub created_at: DateTimeAsMicroseconds,
    pub user_id: String,
    pub new_context: String,
    pub prev_context: String,
    pub log_type: ClientAuditLogTypeDto,
}

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct OrderDetailsDto {
    #[primary_key(0)]
    pub order_id: String,
    //Serialized trading package
    pub trading_package: String,
}

impl ClientAuditLogDto {
    pub fn encrypt_fields(&mut self, encode_key: &AesKey) {
        self.new_context = encrypt_field(&mut self.new_context, encode_key);
        self.prev_context = encrypt_field(&mut self.prev_context, encode_key);        
    }

    pub fn decrypt_fields(&mut self, encode_key: &AesKey) -> bool {
        self.new_context = decrypt_field(&self.client_id, &self.new_context, encode_key);
        self.prev_context = decrypt_field(&self.client_id, &self.prev_context, encode_key);

        return false;
    }
}

#[derive(WhereDbModel)]
pub struct WhereByIdWithPaginationModel<'s> {
    pub client_id: &'s str,
    #[limit]
    pub limit: usize,
    #[offset]
    pub offset: usize,
}

#[derive(WhereDbModel)]
pub struct WhereByAllWithPaginationModel {
    #[limit]
    pub limit: usize,
    #[offset]
    pub offset: usize,
}