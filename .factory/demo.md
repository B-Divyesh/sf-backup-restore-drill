# Demo sandbox

Open `https://backup-restore-drill.sociobot.in/demo/` or `/?demo=1` for the browser demo. It starts in the completed sample state. The banner says **Demo — sample data, nothing is saved** and offers **Reset demo** and **Start for real**.

For the real command-line path, run:

```sh
cargo run -- demo
```

The bundled sample is `examples/Documents/quarterly-tax-notes.txt`. The command creates a new OS temporary workspace, copies that shipped sample through the actual restore, fingerprint, application-check, cleanup, and receipt code path, then removes the complete workspace before exit. It never reads project configuration, user files, or receipt storage.

The browser demo stores no data. Its `?demo=1` entry redirects to `/demo/?demo=1`; it does not read or write real product state.
