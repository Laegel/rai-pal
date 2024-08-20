#[macro_export]
macro_rules! serializable_enum {
  ($enum_name:ident { $($variant:ident),* $(,)? }) => {
      #[derive(serde::Serialize, serde::Deserialize, specta::Type, Clone, PartialEq, Eq, Hash, Debug, Copy)]
      pub enum $enum_name {
          $($variant,)*
      }

      impl core::fmt::Display for $enum_name {
          fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
              write!(f, "{:?}", self)
          }
      }

      impl std::str::FromStr for $enum_name {
          type Err = $crate::result::Error;

          fn from_str(s: &str) -> $crate::result::Result<Self> {
              match s {
                  $(stringify!($variant) => Ok(Self::$variant),)*
                  _ => Err($crate::result::Error::UnknownEnumVariant(s.to_string())),
              }
          }
      }
  };
}
