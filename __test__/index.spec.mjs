import test from 'ava'

import { hotk, KeyCode, Mod } from '../index.js'

test('_', async t => {
  const manager = hotk();
  const response = manager.register([Mod.Control], KeyCode.KeyA);

  manager.init(console.log);

  t.truthy(response.isOk());
});
