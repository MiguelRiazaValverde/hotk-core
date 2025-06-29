pub enum Response {
  OkRegister {
    id: u32,
  },
  ErrorRegister {
    id: u32,
    error: global_hotkey::Error,
  },
  OkUnregister {
    id: u32,
  },
  ErrorUnregister {
    id: u32,
    error: global_hotkey::Error,
  },
}

impl Response {
  pub fn to_napi(&self) -> HotkReponse {
    match self {
      Response::OkRegister { id } => HotkReponse {
        code: ResponseCode::Ok,
        id: *id,
        error: None,
      },
      Response::ErrorRegister { id, error } => HotkReponse {
        code: ResponseCode::Error,
        id: *id,
        error: Some(error.to_string()),
      },
      Response::OkUnregister { id } => HotkReponse {
        code: ResponseCode::Ok,
        id: *id,
        error: None,
      },
      Response::ErrorUnregister { id, error } => HotkReponse {
        code: ResponseCode::Error,
        id: *id,
        error: Some(error.to_string()),
      },
    }
  }
}

/**
 * Represents the possible response codes for hotkey operations.
 *
 * - `Ok`: The operation was successful.
 * - `Error`: The operation failed.
 */
#[napi(string_enum)]
pub enum ResponseCode {
  Ok,
  Error,
}

/**
 * Represents a response from a hotkey operation.
 */
#[napi]
pub struct HotkReponse {
  pub code: ResponseCode,
  pub id: u32,
  pub error: Option<String>,
}

#[napi]
impl HotkReponse {
  /**
   * Checks whether the response indicates success.
   *
   * @returns `true` if the operation was successful, otherwise `false`.
   */
  #[napi]
  pub fn is_ok(&self) -> bool {
    matches!(self.code, ResponseCode::Ok)
  }
}
