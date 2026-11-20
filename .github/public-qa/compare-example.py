"""Run the published EtuSchlopf example or the documented public Chopin pair."""
import collections
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
from fractions import Fraction
from xml.etree import ElementTree as ET
from zipfile import ZipFile
import zlib

ROOT = Path(os.environ["RUNNER_TEMP"]) / "score-comparison"
DOCS = Path(os.environ["GITHUB_WORKSPACE"]) / "docs"
BASE = "https://repo.mdw.ac.at/projects/IWK/the_vienna_4x22_piano_corpus/data/"
CHOPIN = "Chopin_op10_no3_p01"
MIDI_SHA = "80dcfd48414a859e325b0fef197a57808a1758dd366469b7ab9174522027ba72"
WAV_SHA = "660d8ebd492206340e5efb436722fc8cd721488b4d61ad608af063c0f759d029"

def sha(data):
    return hashlib.sha256(data).hexdigest()

def save_pair(name):
    target = ROOT / "inputs" / name
    target.mkdir(parents=True)
    if name == "Beispiel":
        with ZipFile(ROOT / "public-example.zip") as archive:
            midi, = [x for x in archive.namelist() if x.lower().endswith((".mid", ".midi"))]
            wav = str(Path(midi).with_suffix(".wav"))
            data = [archive.read(midi), archive.read(wav)]
    elif name == CHOPIN:
        # The original mdw ZIP is about 1.3 GB. Read only the target member.
        # Fixed offset and lengths refer to the ZIP central directory of the
        # original archive. Verify status, range, local header, CRC and SHA.
        midizip = target / "mdw-midi.zip"
        subprocess.run(["curl", "-fsSL", "--max-time", "90", "--max-filesize", "2000000",
                        BASE + "midi.zip", "-o", str(midizip)], check=True)
        with ZipFile(midizip) as archive:
            midi = archive.read("midi/" + CHOPIN + ".mid")
        offset, compressed, uncompressed = 601831913, 12470847, 15103876
        last = offset + compressed + 639
        rawzip = target / "member.bin"
        headers = target / "headers.txt"
        status = subprocess.check_output(["curl", "-fsSL", "--max-time", "150",
            "--max-filesize", str(compressed + 640), "--range", f"{offset}-{last}",
            "-D", str(headers), "-o", str(rawzip), "-w", "%{http_code}", BASE + "audio.zip"], text=True)
        if status != "206" or f"bytes {offset}-{last}/1295130820" not in headers.read_text().lower():
            raise ValueError("Original audio archive did not serve the expected range")
        raw = rawzip.read_bytes()
        if raw[:4] != b"PK\x03\x04" or struct.unpack_from("<H", raw, 8)[0] != 8:
            raise ValueError("Wrong ZIP local header")
        nlen, xlen = struct.unpack_from("<HH", raw, 26)
        if raw[30:30+nlen].decode() != "audio/Chopin_Etude/" + CHOPIN + ".wav":
            raise ValueError("Wrong audio member")
        start = 30+nlen+xlen
        wav = zlib.decompress(raw[start:start+compressed], -15)
        if (len(wav), zlib.crc32(wav), sha(midi), sha(wav)) != (uncompressed, 1880586119, MIDI_SHA, WAV_SHA):
            raise ValueError("Original Chopin input identity differs")
        data = [midi, wav]
    else:
        raise ValueError("Unsupported input")
    for ext, value in zip(("mid", "wav"), data):
        (target / f"{name}.{ext}").write_bytes(value)
    return [sha(value) for value in data]

def semantics(data):
    score = ET.fromstring(data)
    totals = collections.Counter()
    events = []
    meters = []
    parts = score.findall("part")
    for ipart, part in enumerate(parts):
        base = Fraction()
        ties = {}
        divisions = None
        beatspan = None
        for ibar, bar in enumerate(part.findall("measure"), 1):
            totals["measures"] += 1
            cursor = high = head = base
            for child in bar:
                if child.tag == "attributes":
                    if child.find("divisions") is not None:
                        divisions = int(child.findtext("divisions"))
                    tm = child.find("time")
                    if tm is not None:
                        n, d = int(tm.findtext("beats")), int(tm.findtext("beat-type"))
                        beatspan = Fraction(n*4, d)
                        meters.append([ipart+1, ibar, n, d])
                    continue
                if child.tag not in ("note", "forward", "backup"):
                    continue
                length = Fraction(int(child.findtext("duration","0")), divisions)
                if child.tag in ("forward", "backup"):
                    cursor += length if child.tag=="forward" else -length
                    high = max(high, cursor)
                    continue
                if child.find("grace") is not None:
                    totals["graces"] += 1
                    continue
                if child.find("chord") is None:
                    head = cursor
                    cursor += length
                high = max(high, head+length)
                pitch = child.find("pitch")
                if pitch is None:
                    totals["rests"] += 1
                    continue
                midi = 12*(1+int(pitch.findtext("octave"))) + {"C":0,"D":2,"E":4,"F":5,"G":7,"A":9,"B":11}[pitch.findtext("step")] + int(pitch.findtext("alter","0"))
                voice = (child.findtext("staff"),child.findtext("voice"),midi)
                flags = {t.get("type") for t in child.findall("tie")}
                if "stop" in flags:
                    onset, end = ties.pop(voice)
                    if end != head: raise ValueError("Discontinuous tie")
                else:
                    onset = head
                if "start" in flags:
                    if voice in ties: raise ValueError("Ambiguous tie")
                    ties[voice] = (onset, head+length)
                else:
                    events.append((ipart, onset, midi, head+length-onset))
            if beatspan is None or high<=base: raise ValueError("Invalid metric span")
            base = high if bar.get("implicit")=="yes" else max(high,base+beatspan)
        if ties: raise ValueError("Unclosed ties")
    totals["regular_attacks"] = len(events)
    rows = [[p,str(start),key,str(duration)] for p,start,key,duration in sorted(events)]
    return dict(totals), meters, sha(json.dumps(rows,separators=(",",":")).encode())

def run(name):
    inputs = save_pair(name)
    result = {}
    for revision in (1965,1966,1967,1968):
        work = ROOT / f"run-{revision}" / name
        work.mkdir(parents=True)
        for ext in ("mid","wav"):
            source = ROOT / "inputs" / name / f"{name}.{ext}"
            shutil.copyfile(source,work/source.name)
        (work/"docs").symlink_to(DOCS, target_is_directory=True)
        exe = ROOT / f"r{revision}" / "etuschlopf"
        log = work / "run.log"
        with log.open("wb") as stream:
            call = subprocess.run([str(exe),name,"--jobs","2"],cwd=work,stdout=stream,stderr=subprocess.STDOUT,timeout=480)
        if call.returncode:
            print(log.read_text(errors="replace")[-3000:],flush=True)
            raise ValueError(f"{name} r{revision} exit {call.returncode}")
        if inputs != [sha((work/f"{name}.{ext}").read_bytes()) for ext in ("mid","wav")]:
            raise ValueError("Input changed")
        score = {}
        for typ,suffix in (("master",".musicxml"),("render",".render.musicxml")):
            data = (work/(name+suffix)).read_bytes()
            counts, meter, music_sha = semantics(data)
            score[typ] = {"xml_sha256":sha(data), "score_sha256":music_sha, "counts":counts,"meter":meter}
        score["page_rgb"] = []
        subprocess.run(["pdftoppm","-r","144",str(work/(name+".pdf")),str(work/"page")],check=True)
        imgs = sorted(work.glob("page-*.ppm"),key=lambda p:int(p.stem.split("-")[-1]))
        if not imgs: raise ValueError("Empty score PDF")
        for page in imgs:
            # The PPM file encodes the dimensions, RGB maximum and raw samples.
            score["page_rgb"].append(sha(page.read_bytes()))
        score["exit"] = call.returncode
        result[revision] = score
        print(json.dumps({"piece":name,"revision":revision,**score},separators=(",",":")),flush=True)
    changes=[]
    for previous in (1965,1966,1967):
        old,new=result[previous],result[previous+1]
        same={key:old[key]==new[key] for key in ("master","render","page_rgb")}
        changes.append({"from":previous,"to":previous+1,**same})
    print(json.dumps({"piece":name,"input_sha256":inputs,"transitions":changes},separators=(",",":")),flush=True)
    if not all(all(row[k] for k in ("master","render","page_rgb")) for row in changes):
        raise ValueError("Changed notation or rendered page")
if __name__=="__main__":
    run(sys.argv[1])
