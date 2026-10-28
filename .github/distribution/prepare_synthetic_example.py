"""Create a corpus-independent 4/4 MIDI/WAV pair for the packaged Windows test."""
from pathlib import Path
import hashlib
import json
import math
import struct
import sys
import wave


def vlq(value):
    data = [value & 127]
    value >>= 7
    while value:
        data.insert(0, 128 | (value & 127))
        value >>= 7
    return bytes(data)


def make(destination):
    destination.mkdir(parents=True, exist_ok=False)
    ppq, bpm, bars, rate = 96, 120, 8, 8000
    notes = []
    for bar in range(bars):
        for beat, pitch in enumerate((60, 62, 64, 67)):
            notes.append(((bar * 4 + beat) * ppq, ppq, pitch, 0))
        notes.append((bar * 4 * ppq, 4 * ppq, (48, 43, 41, 43)[bar % 4], 1))
    events = [(0, 0, b'\xff\x51\x03\x07\xa1\x20'), (0, 0, b'\xff\x58\x04\x04\x02\x18\x08')]
    for pos, length, pitch, channel in notes:
        events += [(pos, 2, bytes([0x90 | channel, pitch, 72])), (pos + length, 1, bytes([0x80 | channel, pitch, 0]))]
    track, previous = bytearray(), 0
    for tick, order, message in sorted(events):
        track += vlq(tick - previous) + message
        previous = tick
    track += b'\x00\xff\x2f\x00'
    midi = b'MThd' + struct.pack('>IHHH', 6, 0, 1, ppq) + b'MTrk' + struct.pack('>I', len(track)) + track
    (destination / 'Beispiel.mid').write_bytes(midi)
    samples = bytearray()
    for frame in range(int(bars * 4 * 60 / bpm * rate)):
        time = frame / rate
        beat = time * bpm / 60
        bar = int(beat // 4)
        upper = (60, 62, 64, 67)[int(beat) % 4]
        lower = (48, 43, 41, 43)[bar % 4]
        qtime, btime = (beat % 1) * 60 / bpm, (beat % 4) * 60 / bpm
        upper_env = max(0, min(1, qtime / .01, (.5 - qtime) / .03))
        lower_env = max(0, min(1, btime / .01, (2 - btime) / .03))
        value = .10 * upper_env * math.sin(2 * math.pi * 440 * 2 ** ((upper - 69) / 12) * qtime)
        value += .12 * lower_env * math.sin(2 * math.pi * 440 * 2 ** ((lower - 69) / 12) * btime)
        samples += struct.pack('<h', round(32767 * value))
    with wave.open(str(destination / 'Beispiel.wav'), 'wb') as w:
        w.setparams((1, 2, rate, 0, 'NONE', 'not compressed'))
        w.writeframes(samples)
    identity = {suffix + '_sha256': hashlib.sha256((destination / ('Beispiel.' + suffix)).read_bytes()).hexdigest() for suffix in ('mid', 'wav')}
    identity['midi_sha256'] = identity.pop('mid_sha256')
    identity['origin'] = 'synthetic, corpus-independent; not a user composition'
    (destination / 'input-manifest.json').write_text(json.dumps(identity, indent=2) + '\n')
    print('Created synthetic test pair:', len(notes), 'notes,', bars, 'bars, 4/4.')


if __name__ == '__main__':
    make(Path(sys.argv[1]))
