const buffered_messages = []

import('./tile.worker.js')
    .then(w => {
        self.onmessage = w.onmessage;
        buffered_messages.forEach(msg => {
            self.onmessage(msg);
        });

    })
  .catch(e => console.error("Error importing `tile.worker.js`:", e));

onmessage = function(e) {
    buffered_messages.push(e)
}