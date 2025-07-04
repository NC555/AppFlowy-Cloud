#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollabDocStateParams {
  #[prost(string, tag = "1")]
  pub object_id: ::prost::alloc::string::String,
  #[prost(int32, tag = "2")]
  pub collab_type: i32,
  #[prost(enumeration = "PayloadCompressionType", tag = "3")]
  pub compression: i32,
  #[prost(bytes = "vec", tag = "4")]
  pub sv: ::prost::alloc::vec::Vec<u8>,
  #[prost(bytes = "vec", tag = "5")]
  pub doc_state: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadCompressionType {
  None = 0,
  Zstd = 1,
}
impl PayloadCompressionType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      PayloadCompressionType::None => "NONE",
      PayloadCompressionType::Zstd => "ZSTD",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "NONE" => Some(Self::None),
      "ZSTD" => Some(Self::Zstd),
      _ => None,
    }
  }
}
