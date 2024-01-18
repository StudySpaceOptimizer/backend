use super::common::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  pub user_id: i64,
  pub password_hash: String,
  pub email: String,
  pub user_role: UserRole,
  pub verified: bool,
  pub verification_token: String,
  pub points: i64,
}

impl FromRow<'_, SqliteRow> for User {
  fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
    Ok(User {
      user_id: row.try_get("user_id")?,
      email: row.try_get("email")?,
      password_hash: row.try_get("password_hash")?,
      user_role: row.try_get("user_role")?,
      verified: row.try_get("verified")?,
      verification_token: row.try_get("verification_token")?,
      points: row.try_get("points")?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum UserRole {
  Student,
  RegularUser,
  Admin,
  Assistant,
}

impl<'r> Decode<'r, Sqlite> for UserRole {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
    let value = <&str as Decode<Sqlite>>::decode(value)?;

    match value {
      "Student" => Ok(UserRole::Student),
      "RegularUser" => Ok(UserRole::RegularUser),
      "Admin" => Ok(UserRole::Admin),
      "Assistant" => Ok(UserRole::Assistant),
      _ => Err("Invalid UserRole".into()),
    }
  }
}

impl<'q> Encode<'q, sqlx::Sqlite> for UserRole {
  fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
    // 这里编码你的类型为 SQLite 能理解的格式。
    // 例如，如果你的类型可以转换为字符串：
    buf.push(SqliteArgumentValue::Text(self.to_string().into()));

    // 如果你的类型总是产生非空值，返回 IsNull::No
    IsNull::No
  }
}

impl Type<Sqlite> for UserRole {
  fn type_info() -> SqliteTypeInfo {
    <&str as Type<Sqlite>>::type_info()
  }
}

impl ToString for UserRole {
  fn to_string(&self) -> String {
    match *self {
      UserRole::Student => "Student".to_owned(),
      UserRole::RegularUser => "RegularUser".to_owned(),
      UserRole::Admin => "Admin".to_owned(),
      UserRole::Assistant => "Assistant".to_owned(),
    }
  }
}

impl FromStr for UserRole {
  type Err = std::io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Student" => Ok(UserRole::Student),
      "RegularUser" => Ok(UserRole::RegularUser),
      "Admin" => Ok(UserRole::Admin),
      "Assistant" => Ok(UserRole::Assistant),
      _ => Err(std::io::Error::new(
        ErrorKind::InvalidInput,
        "Provided string does not match any UserRole variant",
      )),
    }
  }
}
