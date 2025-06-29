import test from 'ava'

import { HotKeys, KeyCode, Mods } from '../index.js'

async function poll(poll) {
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
  console.log((await poll.poll()).code);
}

test('_', async t => {

  const n = HotKeys.create();
  await n.register([Mods.Control], KeyCode.KeyA);

  const p = n.takePoll();
  poll(p);

  t.truthy(true);
});
