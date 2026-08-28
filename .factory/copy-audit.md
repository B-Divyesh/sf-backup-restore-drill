# Copy audit — polish 1

The landing first screen was read aloud from a cold mobile view. It names the job, audience, and one next action.

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| H1 | Verify one file restores from your backup | 7 | pass |
| Lede | For people with scripted backups who need repeatable proof that important files still restore. | 13 | pass |
| Action note | Runs a bundled restore and shows its receipt. | 8 | pass |
| Fact | Free and MIT licensed. | 4 | listed claim |
| Fact | No usage tracking. | 3 | listed local-network claim |
| Fact | Works offline after the first visit. | 7 | listed claim |

## Landing inventory

All visitor-facing landing sentences and meaningful controls were checked below. No entry exceeds 22 words or uses a banned marketing term.

| Copy | Words | Claim mapping / result |
| --- | ---: | --- |
| Skip to content | 3 | control |
| How it works | 3 | heading |
| Setup | 1 | heading |
| Demo | 1 | route |
| Privacy | 1 | route |
| Runs on your computer | 4 | product context |
| No usage tracking | 3 | local-network |
| Verify one file restores from your backup | 7 | restore-verification |
| For people with scripted backups who need repeatable proof that important files still restore. | 13 | restore-verification |
| Try it with sample data | 5 | demo-isolation |
| Runs a bundled restore and shows its receipt. | 8 | demo-isolation, restore-verification |
| Free and MIT licensed. | 4 | license |
| No usage tracking. | 3 | local-network |
| Works offline after the first visit. | 7 | offline-reload |
| Restore a sample. Compare its fingerprint. Save the receipt. | 9 | restore-verification |
| Keep your backup tool. | 4 | context |
| Restore Drill runs the restore command you configure. | 8 | documented command behavior |
| Remove the temporary folder. | 4 | demo-isolation |
| It cleans the folder before it writes a receipt. | 9 | demo-isolation |
| Check one selected backup file | 5 | restore-verification |
| Use a small check on a schedule. | 7 | guidance |
| Keep full recovery practice for a separate exercise. | 8 | limitation |
| Restore Drill creates a new folder for each check. | 9 | demo-isolation |
| Your existing backup command restores only the paths you choose. | 10 | documented configuration behavior |
| Compare its SHA-256 fingerprint. | 4 | restore-verification |
| You can run an application check too. | 7 | documented configuration behavior |
| The receipt is hash-linked JSON. | 5 | receipt-integrity |
| A later change is detected when the chain is checked. | 10 | receipt-integrity |
| Start with one important file | 5 | guidance |
| Choose a file you would notice missing. | 8 | guidance |
| Save its SHA-256 fingerprint from a trusted copy. | 8 | guidance |
| Rejects links and paths outside the temporary folder. | 8 | documented configuration behavior |
| See the bundled restore now | 5 | demo-isolation |
| The demo uses shipped sample data and deletes its temporary workspace. | 10 | demo-isolation |
| Check selected files from your existing backup. | 7 | restore-verification |
| Built by Param Factory · build polish-1 | 6 | attribution |

Terminology: runnable example → **sample data**; temporary location → **temporary restore folder**; file identity → **SHA-256 fingerprint**; receipt protection → **hash-linked**; timing → **schedule**; command-line product → **command-line tool**.
