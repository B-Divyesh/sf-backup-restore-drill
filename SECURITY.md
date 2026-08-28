# Security model

Restore Drill executes only the argument arrays you put in its local configuration. It never invokes a shell, interpolates environment variables, reads passphrases, or transmits data. Use repository credentials that can read backups but cannot prune or rewrite them.

The tool creates its own random temporary directory, confines sample checks to regular files within it, refuses symlinks, and removes the directory after success or failure. Receipt files contain paths, sizes, hashes, timings, and result labels—not restored contents or subprocess output.

Local users who can replace the binary or modify the receipt directory can defeat local evidence. Copy receipts to external append-only storage when that threat matters. Report vulnerabilities privately through the repository's security advisory feature.
