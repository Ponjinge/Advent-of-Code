# Advent-of-Code

Personal Advent of Code solutions. Each year/day is an independent solution — there is no shared build, no services, no database, and no networked components.

- `2019/`, `2022/`, `2023/`, `2024/` — Python solutions as Jupyter notebooks (`.ipynb`). Also one standalone script `2023/Day_24/d24_spyder.py`.
- `2020/rust/` — a single Rust (Cargo) CLI project, dependency-free.

## Cursor Cloud specific instructions

Runtime tooling (Python 3.12, Rust/cargo 1.83) is preinstalled; the update script only adds the Python libraries. There are no long-running services to start.

### Puzzle inputs are NOT committed
Notebooks read `./input.txt` (relative to the notebook's own directory) and the Rust CLI reads an input-file path argument. All input files are git-ignored (see `.gitignore`) and absent, so solutions cannot produce real answers until you supply your own AoC input. To smoke-test a notebook/day, create a suitable `input.txt` next to it (or pass a temp file to the Rust CLI). Note that some notebooks hardcode the input size (e.g. `2024/01/01.ipynb` loops over `range(0,1000)`, so it needs exactly 1000 lines).

### Python / Jupyter
- pip installs scripts to `~/.local/bin`, which is not on `PATH`. Invoke Jupyter tooling via the module form: `python3 -m jupyter ...`, `python3 -m nbconvert ...`. Use `python3` (there is no `python` symlink).
- Run a notebook headless (to "test" a day):
  `python3 -m nbconvert --to notebook --execute <path>.ipynb --output /tmp/out.ipynb`
- Interactive: `python3 -m jupyter lab` / `python3 -m jupyter notebook`.
- Run the standalone script: `python3 2023/Day_24/d24_spyder.py`.
- No lint config exists; there are no automated Python tests.

### Rust (`2020/rust/`)
- Build: `cargo build`  |  Test: `cargo test` (no tests defined)  |  Lint/format: `cargo clippy` / `cargo fmt`.
- Run: `cargo run -- <day> <part> <input_file>` (only days 1–2 are implemented). Example with the official AoC 2020 Day 1 sample: `cargo run -- 1 1 <file>` → `514579`.
