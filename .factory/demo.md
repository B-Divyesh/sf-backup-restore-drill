# Demo sandbox

Open `https://backup-restore-drill.sociobot.in/demo/` or `/?demo=1` for the browser demo. It starts in the completed sample state. The banner says **Demo — sample data, nothing is saved** and offers **Reset demo** and **Start for real**. The page includes a self-hosted terminal capture from `restore-drill demo`.

For the real command-line path, run:

```sh
cargo run -- demo
```

The bundled sample is `examples/Documents/quarterly-tax-notes.txt`. The command runs that shipped sample through the restore, fingerprint, application-check, cleanup, and receipt code. It removes the temporary restore folder, then removes every demo file before exit. It never reads project configuration, user files, or receipt storage.

The browser demo uses no storage namespace: localStorage, sessionStorage, IndexedDB, and cookies remain empty. Its `?demo=1` entry redirects to `/demo/?demo=1`; it does not read or write real product state.
