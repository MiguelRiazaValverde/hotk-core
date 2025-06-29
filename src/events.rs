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
  pub fn to_napi(&self) -> HotkeyReponse {
    match self {
      Response::OkRegister { id } => HotkeyReponse {
        code: ResponseCode::Ok,
        id: *id,
        error: String::new(),
      },
      Response::ErrorRegister { id, error } => HotkeyReponse {
        code: ResponseCode::Error,
        id: *id,
        error: error.to_string(),
      },
      Response::OkUnregister { id } => HotkeyReponse {
        code: ResponseCode::Ok,
        id: *id,
        error: String::new(),
      },
      Response::ErrorUnregister { id, error } => HotkeyReponse {
        code: ResponseCode::Error,
        id: *id,
        error: error.to_string(),
      },
    }
  }
}

#[napi(string_enum)]
pub enum ResponseCode {
  Ok,
  Error,
}

#[napi]
pub struct HotkeyReponse {
  pub code: ResponseCode,
  pub id: u32,
  pub error: String,
}
