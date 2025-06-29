// #![deny(clippy::all)]

use global_hotkey::GlobalHotKeyEvent;
use napi::threadsafe_function::ErrorStrategy;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::code::Desc;
use crate::code::Event;
use crate::code::KeyCode;
use crate::code::Mod;
use crate::events::HotkReponse;
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

static HOTK: Lazy<Option<Arc<Mutex<InnerHotk>>>> =
  Lazy::new(|| InnerHotk::create().map(|hotk| Arc::new(Mutex::new(hotk))));

struct InnerHotk {
  pub manager: Manager,
  pub hotkeys: Arc<std::sync::Mutex<HashMap<u32, Desc>>>,
  pub tsfn: Option<ThreadsafeFunction<Event, ErrorStrategy::Fatal>>,
}

impl InnerHotk {
  pub fn create() -> Option<Self> {
    Manager::new().map(|manager| Self {
      manager,
      hotkeys: Default::default(),
      tsfn: None,
    })
  }
}

/**
 * Manages global hotkeys by handling registration, unregistration and listening for hotkey events.
 */
#[napi]
pub struct HotkManager {
  hotk: Arc<Mutex<InnerHotk>>,
}

#[napi]
impl HotkManager {
  /**
   * Initializes the hotkey event listener.
   *
   * This method sets up the callback that will be invoked for every global hotkey event.
   * It can only be called once per program. Subsequent calls will do nothing and return `false`.
   *
   * @param on_event - A function that will be called with each hotkey event.
   *
   * @example
   * ```js
   * import { hotk, Mod, KeyCode } from '@hotk/core';
   *
   * // Get the singleton instance
   * const manager = hotk();
   *
   * // Register Ctrl + A as a hotkey
   * const result = manager.register([Mod.Control], KeyCode.KeyA);
   *
   * // Listen for hotkey events
   * manager.init((event) => {
   *   console.log('Received event:', event);
   * });
   * ```
   */
  #[napi(ts_args_type = "on_event: (event: Event) => void")]
  pub fn init(&mut self, on_event: JsFunction) -> napi::Result<bool> {
    let mut lock = self.hotk.lock().unwrap();

    if lock.tsfn.is_some() {
      return Ok(false);
    }

    let hotkeys = lock.hotkeys.clone();

    if let Some(tsfn) = lock.tsfn.take() {
      tsfn.abort()?;
    }

    let tsfn: ThreadsafeFunction<Event, ErrorStrategy::Fatal> = on_event
      .create_threadsafe_function(0, |ctx| {
        let event: Event = ctx.value;
        ctx.env.create_object().and_then(|mut obj| {
          obj.set("id", event.id)?;
          obj.set("code", event.code)?;
          obj.set("mods", event.mods)?;
          obj.set("eventType", event.event_type)?;
          Ok(vec![obj])
        })
      })?;

    lock.tsfn = Some(tsfn.clone());

    GlobalHotKeyEvent::set_event_handler(Some(move |event: GlobalHotKeyEvent| {
      if let Some(desc) = hotkeys.lock().unwrap().get(&event.id).cloned() {
        let ev = Event {
          id: event.id,
          code: desc.code,
          mods: desc.mods,
          event_type: match event.state {
            global_hotkey::HotKeyState::Pressed => code::EventType::Pressed,
            global_hotkey::HotKeyState::Released => code::EventType::Released,
          },
        };
        tsfn.call(ev, ThreadsafeFunctionCallMode::NonBlocking);
      }
    }));

    Ok(true)
  }

  /**
   * Registers a global hotkey.
   *
   * @example
   * ```js
   * import { hotk, Mod, KeyCode } from '@hotk/core';
   *
   * // Get the singleton instance
   * const manager = hotk();
   *
   * // Register Ctrl + A as a hotkey
   * const result = manager.register([Mod.Control], KeyCode.KeyA);
   *
   * if (result.isOk()) {
   *   console.log('Hotkey successfully registered');
   * } else {
   *   console.error('Failed to register hotkey');
   * }
   *
   * // Listen for hotkey events
   * manager.init((event) => {
   *   console.log('Received event:', event);
   * });
   * ```
   */
  #[napi]
  pub fn register(&self, mods: Vec<Mod>, code: KeyCode) -> HotkReponse {
    let lock = self.hotk.lock().unwrap();

    let (hotkey, response) = lock.manager.register(
      mods.iter().map(|m| m.global_hotkeys()).collect(),
      code.global_hotkeys(),
    );

    if let Response::OkRegister { .. } = response {
      lock
        .hotkeys
        .lock()
        .unwrap()
        .insert(hotkey.id, Desc::new(code, mods));
    }

    response.to_napi()
  }

  /**
   * Unregisters a global hotkey.
   *
   * @example
   * ```js
   * import { hotk, Mod, KeyCode } from '@hotk/core';
   *
   * // Get the singleton instance
   * const manager = hotk();
   *
   * // Register Ctrl + A as a hotkey
   * const result = manager.register([Mod.Control], KeyCode.KeyA);
   *
   * // Listen for hotkey events
   * manager.init((event) => {
   *   console.log('Received event:', event);
   * });
   *
   * // Unregister the hotkey after 3 seconds
   * setTimeout(() => {
   *   manager.unregister([Mod.Control], KeyCode.KeyA);
   * }, 3000);
   * ```
   */
  #[napi]
  pub fn unregister(&self, mods: Vec<Mod>, code: KeyCode) -> HotkReponse {
    let lock = self.hotk.lock().unwrap();

    let (hotkey, response) = lock.manager.unregister(
      mods.iter().map(|m| m.global_hotkeys()).collect(),
      code.global_hotkeys(),
    );

    if let Response::OkUnregister { .. } = response {
      lock.hotkeys.lock().unwrap().remove(&hotkey.id);
    }

    response.to_napi()
  }

  /**
   * Allows the Node.js process to exit naturally if no other tasks are pending.
   *
   * This detaches the internal hotkey event callback from the event loop,
   * meaning the process won't be held open just to listen for hotkey events.
   *
   * @returns `true` if the operation succeeded, `false` otherwise.
   *
   * @example
   * ```js
   * const manager = hotk();
   * manager.init(console.log);
   * manager.unref(); // Now the process can exit if nothing else is running.
   * ```
   */
  #[napi]
  pub fn unref(&self, env: Env) -> bool {
    let mut lock = self.hotk.lock().unwrap();

    if let Some(tsfn) = &mut lock.tsfn {
      tsfn.unref(&env).is_ok()
    } else {
      false
    }
  }

  /**
   * Keeps the Node.js process alive to continue listening for hotkey events.
   *
   * This ensures that the process will not exit until `destroy()` is called
   * or `unref()` is used to detach the event callback from the event loop.
   *
   * @returns `true` if the operation succeeded, `false` otherwise.
   *
   * @example
   * ```js
   * const manager = hotk();
   * manager.init(console.log);
   * manager.refer(); // Keeps the process alive
   * ```
   */
  #[napi]
  pub fn refer(&self, env: Env) -> bool {
    let mut lock = self.hotk.lock().unwrap();

    if let Some(tsfn) = &mut lock.tsfn {
      tsfn.refer(&env).is_ok()
    } else {
      false
    }
  }

  /**
   * Stops listening for hotkey events.
   *
   * This is required to allow the Node.js process to exit when `init` was called without `unref: true`.
   *
   * @example
   * ```js
   * import { hotk, Mod, KeyCode } from '@hotk/core';
   *
   * // Get the singleton instance
   * const manager = hotk();
   *
   * // Register Ctrl + A as a hotkey
   * const result = manager.register([Mod.Control], KeyCode.KeyA);
   *
   * if (result.isOk()) {
   *   console.log('Hotkey successfully registered');
   * } else {
   *   console.error('Failed to register hotkey');
   * }
   *
   * // Start listening for hotkey events without unref
   * manager.init((event) => {
   *   console.log('Received event:', event);
   * });
   *
   * // Stop listening after 3 seconds so the process can exit
   * setTimeout(() => {
   *   manager.destroy();
   * }, 3000);
   * ```
   */
  #[napi]
  pub fn destroy(&mut self, env: Env) -> napi::Result<()> {
    self.unref(env);
    Ok(())
  }
}

/**
 * Returns a singleton instance of `HotkManager`.
 *
 * All `HotkManager` instances share the same internal state, allowing global coordination of hotkeys across your application.
 *
 * @example
 * ```js
 * import { hotk } from '@hotk/core';
 *
 * const manager = hotk();
 * ```
 */
#[napi]
pub fn hotk() -> Option<HotkManager> {
  HOTK.clone().map(|hotk| HotkManager { hotk })
}
