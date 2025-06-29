// #![deny(clippy::all)]

use global_hotkey::GlobalHotKeyEvent;
use napi::bindgen_prelude::ObjectFinalize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::code::Desc;
use crate::code::Event;
use crate::code::KeyCode;
use crate::code::Mods;
use crate::events::HotkeyReponse;
use crate::events::Response;

mod code;
mod events;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Manager;

#[cfg(not(target_os = "windows"))]
mod plain;
#[cfg(not(target_os = "windows"))]
pub use plain::Manager;

#[macro_use]
extern crate napi_derive;

#[napi(custom_finalize)]
pub struct HotKeys {
  cancel: CancellationToken,
  manager: Manager,
  events: Option<Arc<Mutex<Receiver<GlobalHotKeyEvent>>>>,
  hotkeys: Arc<std::sync::Mutex<HashMap<u32, Desc>>>,
}

#[napi]
impl HotKeys {
  #[napi]
  pub fn create() -> Option<HotKeys> {
    let (sx, tx) = channel(100);

    let rt = Runtime::new().unwrap();
    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
      let _ = rt.block_on(async { sx.send(event).await });
    }));

    Manager::new().map(|manager| Self {
      cancel: CancellationToken::new(),
      manager,
      events: Some(Arc::new(Mutex::new(tx))),
      hotkeys: Default::default(),
    })
  }

  #[napi]
  pub fn register(&mut self, mods: Vec<Mods>, code: KeyCode) -> HotkeyReponse {
    let (hotkey, response) = self.manager.register(
      mods.iter().map(|m| m.global_hotkeys()).collect(),
      code.global_hotkeys(),
    );

    if let Response::OkRegister { .. } = response {
      self
        .hotkeys
        .lock()
        .unwrap()
        .insert(hotkey.id, Desc::new(code, mods));
    }

    response.to_napi()
  }

  #[napi]
  pub fn unregister(&mut self, mods: Vec<Mods>, code: KeyCode) -> HotkeyReponse {
    let (hotkey, response) = self.manager.unregister(
      mods.iter().map(|m| m.global_hotkeys()).collect(),
      code.global_hotkeys(),
    );

    if let Response::OkUnregister { .. } = response {
      self.hotkeys.lock().unwrap().remove(&hotkey.id);
    }

    response.to_napi()
  }

  #[napi]
  pub fn take_poll(&mut self) -> Option<HotKeysPoll> {
    self.events.take().map(|events| HotKeysPoll {
      hotkeys: self.hotkeys.clone(),
      cancel: self.cancel.clone(),
      events,
    })
  }

  #[napi]
  pub fn destroy(&self) {
    self.cancel.cancel();
  }
}

impl ObjectFinalize for HotKeys {
  fn finalize(self, _env: napi::Env) -> napi::Result<()> {
    self.destroy();
    Ok(())
  }
}

#[napi]
pub struct HotKeysPoll {
  cancel: CancellationToken,
  events: Arc<Mutex<Receiver<GlobalHotKeyEvent>>>,
  hotkeys: Arc<std::sync::Mutex<HashMap<u32, Desc>>>,
}

#[napi]
impl HotKeysPoll {
  #[napi]
  pub async fn poll(&self) -> Option<Event> {
    let mut lock = self.events.lock().await;

    tokio::select! {
      biased;

      _ = self.cancel.cancelled() => {None}

      result = lock.recv() => {
        if let Some((event, Some(desc))) = result.map(|e| (e, self.hotkeys.lock().unwrap().get(&e.id).cloned())) {
          Some(Event {
            code: desc.code,
            mods: desc.mods,
            event_type: match event.state {
              global_hotkey::HotKeyState::Pressed => code::EventType::Pressed,
              global_hotkey::HotKeyState::Released => code::EventType::Released,
            },
          })
        } else {
          None
        }
      }
    }
  }
}
