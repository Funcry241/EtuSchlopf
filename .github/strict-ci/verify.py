"""Rebuild native source snapshots from Git blobs and verify the native take receipts."""
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import zipfile

HERE = Path(__file__).resolve().parent
SESSION = json.loads((HERE / 'run.json').read_text())
ROOT = Path(os.environ['ETU_NATIVE_ROOT'])
REV = SESSION['revision']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def parent_index():
    return dict(line.split('\t', 1)[::-1] for line in gzip.decompress((HERE / 'native-r1965.tsv.gz').read_bytes()).decode().splitlines())


def indexes():
    now = parent_index()
    previous = dict(now)
    for rev in range(1966, REV + 1):
        previous = dict(now)
        delta = json.loads((HERE / f'r{rev}.json').read_text())
        for path, oid in delta.items():
            if oid is None:
                del now[path]
            else:
                now[path] = oid
    return previous, now


def fetch_blobs(required):
    objects = {}
    for archive in HERE.glob('objects-*.zip'):
        with zipfile.ZipFile(archive) as z:
            for key in z.namelist():
                if key in required:
                    objects[key] = z.read(key)
    need = sorted(required - objects.keys())
    stream = io.BytesIO(subprocess.check_output(['git', 'cat-file', '--batch'], input=''.join(x+'\n' for x in need).encode()))
    for oid in need:
        key, kind, size = stream.readline().split()
        if key.decode() != oid or kind != b'blob':
            raise ValueError(f'Missing source object {oid}')
        objects[oid] = stream.read(int(size))
        assert stream.read(1) == b'\n'
    for key, raw in objects.items():
        assert hashlib.sha1(f'blob {len(raw)}\0'.encode()+raw).hexdigest() == key
    return objects


def prepare():
    parent, child = indexes()
    objects = fetch_blobs(set(parent.values()) | set(child.values()))
    if ROOT.exists():
        raise ValueError(f'Expected fresh runner directory {ROOT}')
    ROOT.mkdir()
    for path, oid in parent.items():
        out = ROOT / path
        assert out.resolve().is_relative_to(ROOT.resolve())
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(objects[oid])
    assert f'revision=r{REV-1}' in (ROOT / 'ZIP_KIND.txt').read_text().splitlines()
    package = ROOT / SESSION['archive']
    with zipfile.ZipFile(package, 'w') as z:
        for name, oid in sorted(child.items()):
            if name == 'Cargo.lock':
                continue
            item=zipfile.ZipInfo(name, (2026,11,20,12,0,0))
            item.create_system=3
            item.external_attr=0o100644 << 16
            item.compress_type=zipfile.ZIP_DEFLATED
            z.writestr(item, objects[oid], compresslevel=6)
    if sha(package.read_bytes()) != SESSION['sha256']:
        raise ValueError(f'ZIP identity differs: {sha(package.read_bytes())} expected {SESSION["sha256"]}')
    print(f'Native parent r{REV-1}: {len(parent)} source files. Exact r{REV} ZIP: {SESSION["sha256"]}.',flush=True)


def evidence():
    assert f'revision=r{REV}' in (ROOT / 'ZIP_KIND.txt').read_text().splitlines()
    records = [p for p in (ROOT/'target/etuschlopf_take_rejection_ledger_v1').rglob(f'accepted-r{REV}-{SESSION["sha256"]}.txt')]
    assert len(records)==1 and f'accepted_zip_sha256={SESSION["sha256"]}' in records[0].read_text().splitlines()
    for leaf in ('etuschlopf_strict_test_validation.receipt','etuschlopf_strict_clippy_validation.receipt'):
        p=ROOT/'target'/leaf
        assert p.is_file() and p.stat().st_size>0, f'Missing native receipt {leaf}'
        print(f'{leaf}: {sha(p.read_bytes())}')
    _, child=indexes()
    for name,key in child.items():
        raw=(ROOT/name).read_bytes()
        assert hashlib.sha1(f'blob {len(raw)}\0'.encode()+raw).hexdigest()==key, f'Installed source mismatch: {name}'
    reports = list(ROOT.rglob('Beispiel.result.txt'))
    pdfs = [p for p in ROOT.rglob('Beispiel.pdf') if 'target' not in p.relative_to(ROOT).parts]
    assert reports and pdfs and any(p.read_bytes().startswith(b'%PDF-') for p in pdfs), 'Fresh example report/PDF missing'
    print(f'Accepted {SESSION["archive"]} SHA256 {SESSION["sha256"]}. Native Windows Strict and public example passed.')


if __name__=='__main__':
    {'prepare':prepare,'evidence':evidence}[sys.argv[1]]()
