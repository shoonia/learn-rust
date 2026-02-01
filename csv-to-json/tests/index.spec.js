import { ok } from 'node:assert/strict';
import data from './data.json' with { type: 'json' };

console.log('\x1b[90m\x1b[0m');
console.log('\x1b[1m\x1b[36m', 'Running tests', '\x1b[0m');

data.forEach((item, index) => {
  ok(item.length == 3, `Item at index ${index} does not have length 3. Actual length: ${item.join()}`);
});

console.log('\x1b[32m', `Length: ${data.length}`, '\x1b[0m');
console.log('\x1b[1m\x1b[32m', 'Done!', '\x1b[0m');
console.log('\x1b[90m\x1b[0m');
