# @hotk/core

This package provides a minimal utility for registering and unregistering global keyboard shortcuts. It is designed as a low-level component of the @hotk ecosystem and its API may change without notice.

The `init` method accepts a callback to receive hotkey events and an optional boolean parameter (`unref`). If `unref` is set to `true` (the default), the Node.js event loop will not wait for these events to finish before exiting, allowing the process to close normally when there are no other tasks. If `unref` is `false`, the event listener will keep the Node.js process alive, preventing it from exiting until the listener is explicitly destroyed.

| Platform | Supported | Tested |
| -------- | :-------: | :----: |
| Windows  |    ✅     |   ✅   |
| macOS    |    ❓     |   ❌   |
| Linux    |    ❓     |   ❌   |

```js
import { hotk, KeyCode, Mod } from "@hotk/core";

const manager = hotk();

manager.register([Mod.Control], KeyCode.KeyA);

manager.init(console.log);

setTimeout(() => manager.destroy(), 3000);
```
