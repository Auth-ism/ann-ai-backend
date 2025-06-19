
pub fn convert_i32(uuid: uuid::Uuid) -> i32 {
    let id = uuid::Uuid::new_v4();
    let bytes: [u8; 4] = id.as_bytes()[0..4].try_into().unwrap();
    i32::from_le_bytes(bytes)
}