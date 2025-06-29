use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::GlobalHotKeyManager;

use crate::events::Response;

pub struct Manager {
  manager: GlobalHotKeyManager,
}

impl Manager {
  pub fn new() -> Option<Self> {
    GlobalHotKeyManager::new()
      .map(|manager| Self { manager })
      .ok()
  }

  pub fn register(&self, mods: Vec<Modifiers>, key: Code) -> (HotKey, Response) {
    let mods = mods.into_iter().fold(Modifiers::empty(), |acc, m| acc | m);
    let hotkey = HotKey::new(Some(mods), key);

    let r = match self.manager.register(hotkey) {
      Ok(()) => Response::OkRegister { id: hotkey.id },
      Err(error) => Response::ErrorRegister {
        id: hotkey.id,
        error,
      },
    };

    (hotkey, r)
  }

  pub fn unregister(&self, mods: Vec<Modifiers>, key: Code) -> (HotKey, Response) {
    let mods = mods.into_iter().fold(Modifiers::empty(), |acc, m| acc | m);
    let hotkey = HotKey::new(Some(mods), key);

    let r = match self.manager.register(hotkey) {
      Ok(()) => Response::OkUnregister { id: hotkey.id },
      Err(error) => Response::ErrorUnregister {
        id: hotkey.id,
        error,
      },
    };

    (hotkey, r)
  }
}
