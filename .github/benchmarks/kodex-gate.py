"""Measure the r1992 build script without changing the source it validates."""

import argparse
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time


BUILD_SCRIPT = Path("src/kodex_build_orchestrator.rs")
SSC_CONTRACT = Path("src/build_gate/ssc_evidence_proof_shadow_contract_v1.txt")
SERIAL_SSC = """    let validated_ssc =
        take_change_control::ssc_evidence_proof_shadow_permit_kodex::validate_current_tree(&manifest_dir)?;
"""
PARALLEL_SSC = """    let (validated_ssc, music_identity) = std::thread::scope(|scope| {
        let music_identity = scope.spawn(|| {
            take_change_control::validate_productive_music_identity_provenance_tree(&manifest_dir)
        });
        let validated_ssc =
            take_change_control::ssc_evidence_proof_shadow_permit_kodex::validate_current_tree(&manifest_dir);
        (validated_ssc, music_identity.join())
    });
    let validated_ssc = validated_ssc?;
"""
SERIAL_MUSIC_IDENTITY = "    take_change_control::validate_productive_music_identity_provenance_tree(&manifest_dir)?;\n"
EXPECTED_REJECTION = "closed evidence/proof/shadow contract changed"


def compile_gate(source, binary, rustflags):
    subprocess.run(
        ["rustc", "--edition=2024", "--crate-name", "build_script_build",
         "-C", "opt-level=0", "-C", "codegen-units=256", "-C", "debug-assertions=off",
         "-C", "overflow-checks=on", *rustflags, str(source / BUILD_SCRIPT), "-o", str(binary)],
        check=True,
    )


def run_gate(binary, source, output, rustflags):
    output.mkdir()
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(source), OUT_DIR=str(output),
               CARGO_ENCODED_RUSTFLAGS="\x1f".join(rustflags))
    started = time.perf_counter()
    result = subprocess.run([str(binary)], cwd=source, env=env, capture_output=True, timeout=240)
    return time.perf_counter() - started, result


def generated_files(output):
    return {file.relative_to(output): file.read_bytes() for file in output.rglob("*") if file.is_file()}


def compare(source, work):
    rustflags = ["-C", "target-feature=+crt-static"] if os.name == "nt" else []
    parallel = work / "parallel"
    shutil.copytree(source, parallel)
    script = parallel / BUILD_SCRIPT
    text = script.read_text(encoding="utf-8")
    if text.count(SERIAL_SSC) != 1 or text.count(SERIAL_MUSIC_IDENTITY) != 1:
        raise RuntimeError("Unexpected r1992 build-script shape")
    text = text.replace(SERIAL_SSC, PARALLEL_SSC).replace(
        SERIAL_MUSIC_IDENTITY, '    music_identity.expect("Music identity inspection panicked")?;\n'
    )
    script.write_text(text, encoding="utf-8")

    binaries = {}
    elapsed = {}
    outputs = {}
    directives = {}
    for variant, input_source in [("serial", source), ("parallel", parallel)]:
        binaries[variant] = work / f"{variant}.exe"
        compile_gate(input_source, binaries[variant], rustflags)

    for variant, binary in binaries.items():
        output = work / f"{variant}-out"
        elapsed[variant], result = run_gate(binary, source, output, rustflags)
        if result.returncode != 0:
            raise RuntimeError(f"{variant} failed ({result.returncode}): {result.stderr.decode(errors='replace')}")
        directives[variant] = result.stdout.replace(str(output).encode(), b"<OUT_DIR>")
        outputs[variant] = generated_files(output)
        print(f"{variant}: {elapsed[variant]:.2f} s, exit {result.returncode}", flush=True)

    if directives["serial"] != directives["parallel"]:
        raise RuntimeError("Cargo directives differ")
    if len(outputs["serial"]) != 3 or outputs["serial"] != outputs["parallel"]:
        raise RuntimeError("Generated files differ")

    invalid_source = work / "invalid"
    shutil.copytree(source, invalid_source)
    contract = invalid_source / SSC_CONTRACT
    with contract.open("ab") as target:
        target.write(b"\nDeliberate corruption for the negative benchmark case.\n")
    failures = {}
    for variant, binary in binaries.items():
        _, result = run_gate(binary, invalid_source, work / f"{variant}-invalid-out", rustflags)
        stderr = result.stderr.decode(errors="replace")
        diagnostic = next(
            (line for line in stderr.splitlines() if "EtuSchlopf architecture build gate failed:" in line),
            "",
        )
        if result.returncode == 0 or EXPECTED_REJECTION not in diagnostic:
            raise RuntimeError(f"{variant} did not reject the invalid SSC contract: {stderr}")
        failures[variant] = (result.returncode, diagnostic)
    if failures["serial"] != failures["parallel"]:
        raise RuntimeError(f"Rejections differ: {failures}")

    summary = (
        f"Serial: {elapsed['serial']:.2f} s\n\n"
        f"Parallel: {elapsed['parallel']:.2f} s\n\n"
        "Cargo directives and all three generated files match.\n\n"
        f"Invalid SSC contract: both variants returned exit {failures['serial'][0]} "
        "with the same diagnostic.\n"
    )
    print(summary, flush=True)
    if summary_path := os.getenv("GITHUB_STEP_SUMMARY"):
        with Path(summary_path).open("a", encoding="utf-8") as report:
            report.write(summary)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("temp", type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="etu-kodex-", dir=args.temp) as folder:
        compare(args.source.resolve(), Path(folder))
