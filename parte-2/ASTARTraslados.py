import shutil
import subprocess
import sys
from pathlib import Path

MANIFEST = Path(__file__).parent.joinpath("astar", "Cargo.toml")

if __name__ == "__main__":
    if shutil.which("cargo") is None:
        print(
            "\033[31mError:\033[0m Ejecutable de `cargo` no encontrado. Se necesita la toolchain de Rust (https://www.rust-lang.org/tools/install) para continuar"
        )
        exit(-1)
    subprocess.run(
        ["cargo", "run", "--manifest-path", MANIFEST, "--release", *sys.argv[1:]],
    )
