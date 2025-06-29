# @hotk/core

⚠ Internal package — not intended for external use.

This package provides a minimal utility for registering and unregistering global keyboard shortcuts. It is designed as a low-level component of the @hotk ecosystem and its API may change without notice.

The event polling mechanism is separated from the main HotKeys object via takePoll(). This ensures that when the object is dropped, the associated polling promise is automatically canceled, preventing resource leaks and simplifying lifecycle management.

Useful for internal integrations where explicit control over keyboard shortcuts is required.

| Platform | Supported | Tested |
| -------- | :-------: | :----: |
| Windows  |    ✅     |   ✅   |
| macOS    |    ✅     |   ❌   |
| Linux    |    ✅     |   ❌   |

```js
import { HotKeys, KeyCode, Mods } from "@hotk/core";

async function managePoll(poll) {
  let response;
  while ((response = await poll.poll())) {
    console.log(response.code);
  }
}

(async () => {
  const manager = HotKeys.create();
  await manager.register([Mods.Control], KeyCode.KeyA);

  const poll = manager.takePoll();
  managePoll(poll);

  setTimeout(() => manager.destroy(), 3000);
})();
```
