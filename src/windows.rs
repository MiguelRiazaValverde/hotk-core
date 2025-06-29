use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::GlobalHotKeyManager;

use std::ptr::null_mut;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{self, DispatchMessageW, PostThreadMessageW, TranslateMessage, MSG};

use crate::events::Response;

enum Action {
  Register {
    hotkey: HotKey,
    channel: Sender<Response>,
  },
  Unregister {
    hotkey: HotKey,
    channel: Sender<Response>,
  },
  Exit,
}

unsafe fn event_loop(receiver_handle: Receiver<Action>, tx: Sender<u32>) {
  let manager = GlobalHotKeyManager::new().unwrap();
  let _ = manager.register(HotKey::new(Some(Modifiers::CONTROL), Code::KeyE));
  let mut msg: MSG = std::mem::zeroed();
  let id = GetCurrentThreadId();
  let _ = tx.send(id);

  while winuser::GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
    TranslateMessage(&msg);
    DispatchMessageW(&msg);
    if msg.message != winuser::WM_USER {
      continue;
    }
    if let Ok(action) = receiver_handle.recv() {
      match action {
        Action::Register { hotkey, channel } => {
          let response = if let Err(error) = manager.register(hotkey) {
            Response::ErrorRegister {
              id: hotkey.id,
              error,
            }
          } else {
            Response::OkRegister { id: hotkey.id }
          };
          let _ = channel.send(response);
        }
        Action::Unregister { hotkey, channel } => {
          let response = if let Err(error) = manager.unregister(hotkey) {
            Response::ErrorUnregister {
              id: hotkey.id,
              error,
            }
          } else {
            Response::OkUnregister { id: hotkey.id }
          };
          let _ = channel.send(response);
        }
        Action::Exit => return,
      }
    } else {
      return;
    }
  }
}

pub struct Manager {
  handler: Option<JoinHandle<()>>,
  sender: Sender<Action>,
  thread_id: u32,
}

impl Manager {
  pub fn new() -> Option<Self> {
    let (sender_handle, receiver_handle) = channel();
    let (tx, rx) = channel();

    let handler = std::thread::spawn(move || {
      unsafe { event_loop(receiver_handle, tx) };
    });

    let thread_id = rx.recv().unwrap();

    Some(Self {
      handler: Some(handler),
      sender: sender_handle,
      thread_id,
    })
  }

  fn notify_thread(&self) {
    unsafe { PostThreadMessageW(self.thread_id, winuser::WM_USER, 0, 0) };
  }

  pub fn register(&self, mods: Vec<Modifiers>, key: Code) -> (HotKey, Response) {
    let mods = mods.into_iter().fold(Modifiers::empty(), |acc, m| acc | m);
    let hotkey = HotKey::new(Some(mods), key);

    let (sender_handle, receiver_handle) = channel();
    let _ = self.sender.send(Action::Register {
      hotkey,
      channel: sender_handle,
    });

    self.notify_thread();
    let r = receiver_handle.recv().unwrap();

    (hotkey, r)
  }

  pub fn unregister(&self, mods: Vec<Modifiers>, key: Code) -> (HotKey, Response) {
    let mods = mods.into_iter().fold(Modifiers::empty(), |acc, m| acc | m);
    let hotkey = HotKey::new(Some(mods), key);

    let (sender_handle, receiver_handle) = channel();
    let _ = self.sender.send(Action::Unregister {
      hotkey,
      channel: sender_handle,
    });

    self.notify_thread();
    let r = receiver_handle.recv().unwrap();

    (hotkey, r)
  }
}

impl Drop for Manager {
  fn drop(&mut self) {
    let _ = self.sender.send(Action::Exit);
    self.notify_thread();
    if let Some(join) = self.handler.take() {
      let _ = join.join();
    }
  }
}
