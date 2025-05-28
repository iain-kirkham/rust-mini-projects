# File Sync Tool 🗂️

A simple and minimal directory synchronisation tool written in Rust.

This program syncs files from a **source** directory to a **target** directory:

-  Copies new and updated files.
-  Deletes files and directories in the target that no longer exist in the source directory.
-  Supports dry-run mode to preview actions without modifying anything.

## Usage

```bash
cargo run -- <source_directory> <target_directory> [OPTIONS]
```

### Arguments:

| Argument         | Description                                     |
|------------------|-------------------------------------------------|
| `<source>`       | Path to the source directory.                   |
| `<target>`       | Path to the target directory.                   |

### Options:

| Flag             | Description                                     |
|------------------|-------------------------------------------------|
| `-d, --dry-run`  | Perform a dry run (no actual file operations).  |

---

### 💡 Example

```bash
cargo run -- ./my_source ./my_backup --dry-run
```

This will print the list of files that would be copied or deleted but won't complete the actions.

```bash
cargo run -- ./my_source ./my_backup
```

This will perform a real synchronisation:
- Copy new or updated files from `my_source` to `my_backup`.
- Delete files in `my_backup` that no longer exist in `my_source`.

---

## Features

- Minimal, no external dependencies beyond `clap` and `walkdir`.
- Safe to use with `--dry-run` to test before syncing.
- Clear output showing skipped, copied, updated, and deleted files.

---

## Build

```bash
cargo build --release
```

The binary will be located at `target/release/file-sync-tool.exe`.

---

## Planned Features

- [x] Dry-run mode.
- [x] File copy and update detection.
- [x] Target clean up of removed files.
- [ ] Optional progress display.
- [ ] More detailed sync summary at the end.
- [ ] Error handling improvements.

---

## Notes

This project is inspired by Unix `rsync` while being:
- simple to understand
- ideal for learning
- extendable for future features.
