# Copy audit — repair 2

Checked 2026-09-05. Counts use whitespace-delimited words after Markdown punctuation is removed. Commands, file paths, TOML fields, URLs, and decorative labels are excluded. No visitor-facing sentence exceeds 22 words or contains a banned marketing word.

## First screen

Read aloud at 390×844 before scrolling:

| Copy | Words | Claim / result |
| --- | ---: | --- |
| Verify one file restores from your backup | 7 | selected-file verification |
| For people with scripted backups who need repeatable proof that important files still restore. | 13 | audience statement |
| Try it with sample data | 5 | demo isolation |
| Runs bundled data and shows the completed receipt. | 8 | demo isolation |
| Free and MIT licensed. | 4 | license |
| No usage tracking. | 3 | browser privacy |
| Works offline after the first visit. | 7 | offline reload |

The job is verifying one restored backup file. The audience is people who already have scripted backups. The first action is **Try it with sample data**. The 390px regression test confirms all three facts remain before the fold.

## Landing inventory

| Copy | Words | Claim mapping / result |
| --- | ---: | --- |
| Restore Drill — Check a backup file restores | 7 | selected-file verification |
| Check that an important backup file restores before you need it. | 11 | selected-file verification |
| Skip to content | 3 | control |
| Offline. This guide and sample demo remain available after the first visit. | 11 | offline reload |
| Restore Drill home | 3 | label |
| How it works / Setup / Demo / Privacy | 7 | navigation |
| Command-line tool | 2 | product category |
| A file moves from an archive box to a temporary tray, is checked, then becomes a receipt. | 17 | image description |
| Restore a sample. Compare its fingerprint. Save the receipt. | 9 | process summary |
| Keep your backup tool. | 4 | scope |
| Restore Drill runs the restore command you configure. | 8 | configured-command |
| Remove the temporary restore folder. | 5 | temporary-folder-lifecycle |
| It cleans the folder before it writes a receipt. | 9 | temporary-folder-lifecycle |
| Check one selected backup file | 5 | restore-verification |
| Use a small check on a schedule. | 7 | guidance |
| Keep full recovery practice for a separate exercise. | 8 | limitation |
| Create a temporary restore folder | 5 | temporary-folder-lifecycle |
| Restore Drill creates a new folder for each check. | 9 | temporary-folder-lifecycle |
| Restore selected files | 3 | section heading |
| Configure your backup command to restore only the files you want to check. | 12 | configuration guidance |
| Compare the restored file | 4 | section heading |
| Compare its SHA-256 fingerprint. | 4 | restore-verification |
| It can check whether an application opens the restored file. | 10 | application-check |
| Write a receipt | 3 | section heading |
| The receipt is hash-linked JSON. | 5 | receipt-integrity |
| A later change is detected when the chain is checked. | 10 | receipt-integrity |
| Start with one important file | 5 | guidance |
| Choose a file you would notice missing. | 8 | guidance |
| Save its SHA-256 fingerprint from a trusted copy. | 8 | guidance |
| Rejects links and files outside the temporary restore folder. | 9 | path-safety |
| Restore Drill checks your selected files. | 6 | scope limitation |
| Your backup command controls its own restore scope. | 8 | scope limitation |
| Copy configuration | 2 | control |
| See the bundled restore now | 5 | demo isolation |
| The demo uses shipped sample data and removes its temporary restore folder. | 11 | demo-isolation |
| Check selected files from your existing backup. | 7 | restore-verification |
| View source on GitHub (external) | 5 | external link label |
| Built by Param Factory · build repair-2 | 6 | attribution |

Runtime messages are also short, specific, and actionable: **Configuration copied to clipboard**, **Clipboard was unavailable**, **Sample restore running**, **Missing file detected**, and **Confirm the backup includes this path, then run the drill again.**

## README inventory

| Copy | Words | Claim mapping / result |
| --- | ---: | --- |
| It checks that selected files still restore. | 7 | restore-verification |
| It restores into a new temporary restore folder and compares SHA-256 fingerprints. | 12 | restore-verification, temporary-folder-lifecycle |
| It can check whether an application opens the restored file. | 10 | application-check |
| It removes the folder and writes a hash-linked receipt. | 9 | temporary-folder-lifecycle, receipt-integrity |
| Run the shipped sample through the same restore checks. | 9 | demo-isolation |
| The command creates and removes a temporary restore folder. | 9 | demo-isolation |
| It does not read your configuration, backups, or receipt directory. | 10 | demo-isolation |
| The source build creates a single restore-drill command. | 7 | build-artifacts |
| Commands run their arguments directly, without a shell. | 8 | configured-command |
| Restore commands contain {target} once. | 5 | configured-command |
| Application checks contain {file} once. | 5 | configured-command |
| Restore Drill rejects links and files that lead outside its temporary restore folder. | 12 | path-safety |
| It does not copy backup-command output into receipts. | 8 | receipt-redaction |
| Receipts are read-only JSON files linked by SHA-256. | 8 | receipt-integrity |
| run, status, and receipts support --json for scheduler and alert-tool input. | 11 | json-and-exit-codes |
| The documented result-code table | 5 | json-and-exit-codes |
| npm run build creates dist/bin/restore-drill and dist/site/. | 7 | build-artifacts |
| Deploy with the included security-header and cache settings. | 7 | response-policy |
| The website has no forms, analytics, cookies, tracking pixels, or third-party scripts. | 12 | browser-privacy |
| Restore Drill is free and MIT licensed. | 7 | license |

## Terminology

| Concept | Use everywhere |
| --- | --- |
| Product | Restore Drill |
| Runnable example | bundled sample data / demo |
| Safety location | temporary restore folder |
| File identity | SHA-256 fingerprint |
| Receipt protection | hash-linked JSON |
| Application validation | application check |
| Timing | schedule |
| Backup process output | backup-command output |
| Command-line product | command-line tool |
