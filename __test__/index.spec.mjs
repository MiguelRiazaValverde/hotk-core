import test from 'ava'

import { hotk, KeyCode, Mods } from '../index.js'

test('_', async t => {
  const manager = hotk();
  const response = manager.register([Mods.Control], KeyCode.KeyA);

  manager.onEvent(console.log);

  t.truthy(response.isOk());
});
