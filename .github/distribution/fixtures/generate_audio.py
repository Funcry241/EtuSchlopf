from pathlib import Path
import struct,math,bisect,wave,gzip,hashlib,json,base64
import numpy as np
out=Path(__file__).resolve().parent
def parse(path):
 b=Path(path).read_bytes();assert b[:4]==b'MThd';hn=struct.unpack('>I',b[4:8])[0];fmt,ntracks,ppq=struct.unpack('>HHH',b[8:14]);assert fmt in (0,1) and ppq<32768
 pos=8+hn;events=[];tempos=[(0,500000)]
 def vlq(t,i):
  val=0
  while True:
   v=t[i];i+=1;val=(val<<7)|(v&127)
   if v<128:return val,i
 for track in range(ntracks):
  assert b[pos:pos+4]==b'MTrk';size=struct.unpack('>I',b[pos+4:pos+8])[0];t=b[pos+8:pos+8+size];pos+=8+size;i=0;ticks=0;running=None
  while i<len(t):
   dt,i=vlq(t,i);ticks+=dt;status=t[i]
   if status>=128:i+=1;running=status if status<240 else None
   else:assert running is not None;status=running
   if status==255:
    kind=t[i];i+=1;size,i=vlq(t,i);data=t[i:i+size];i+=size
    if kind==81:tempos.append((ticks,int.from_bytes(data,'big')))
   elif status in (240,247):size,i=vlq(t,i);i+=size
   else:
    count=1 if status&240 in (192,208) else 2;data=t[i:i+count];i+=count
    if status&240 in (128,144):events.append((ticks,track,status&15,status&240,data[0],data[1]))
 tempos=sorted(dict(tempos).items());times=[0.0]
 for k in range(1,len(tempos)):times.append(times[-1]+(tempos[k][0]-tempos[k-1][0])*tempos[k-1][1]/(1e6*ppq))
 def secs(tick):
  k=bisect.bisect_right([x[0] for x in tempos],tick)-1
  return times[k]+(tick-tempos[k][0])*tempos[k][1]/(1e6*ppq)
 active={};notes=[]
 for tick,track,ch,kind,key,vel in sorted(events):
  slot=(track,ch,key)
  if kind==144 and vel:
   assert slot not in active;active[slot]=(tick,vel)
  elif slot in active:
   start,v=active.pop(slot);notes.append((secs(start),secs(tick),key,v))
 assert not active
 return notes
entries=[]
for name,path,source in [('synthetic',out/'synthetic.mid','project-authored synthetic MIDI'),('menuet',out/'menuet.mid','https://www.mutopiaproject.org/ftp/BachJS/BWVAnh114/anna-magdalena-04/anna-magdalena-04.mid')]:
 midi=path.read_bytes();(out/(name+'.mid')).write_bytes(midi);notes=parse(path);rate=8000;duration=max(x[1] for x in notes)+0.25;audio=np.zeros(math.ceil(duration*rate),dtype=np.float64)
 for start,end,key,vel in notes:
  lo=round(start*rate);hi=round(end*rate);tt=np.arange(hi-lo)/rate;freq=440*2**((key-69)/12);phase=2*np.pi*freq*tt
  envelope=np.minimum(1,tt/.005)*np.minimum(1,np.maximum(0,(end-start-tt)/.025))
  tone=(np.sin(phase)+.2*np.sin(2*phase)+.08*np.sin(3*phase))*envelope*(vel/127)
  audio[lo:hi]+=tone
 audio*=.8/max(abs(audio));pcm=np.rint(audio*32767).astype('<i2');wav=out/(name+'.wav')
 with wave.open(str(wav),'wb') as w:w.setnchannels(1);w.setsampwidth(2);w.setframerate(rate);w.writeframes(pcm.tobytes())
 zipped=gzip.compress(wav.read_bytes(),compresslevel=9,mtime=0);(out/(name+'.wav.gz')).write_bytes(zipped)
 entry={'name':name,'midi_source':source,'midi_sha256':hashlib.sha256(midi).hexdigest(),'wav_sha256':hashlib.sha256(wav.read_bytes()).hexdigest(),'wav_gzip_sha256':hashlib.sha256(zipped).hexdigest(),'notes':len(notes),'duration_seconds':duration,'sample_rate':rate,'bits':16,'rms':float(np.sqrt(np.mean(audio**2))),'nonzero_samples':int(np.count_nonzero(pcm)),'wav_origin':'own audible harmonic synthesis of the exact MIDI, not an independent acoustic recording','composition_and_midi_license':'Public Domain' if name=='menuet' else 'CC0-1.0','wav_license':'CC0-1.0'};entries.append(entry)
print(json.dumps(entries,indent=2));(out/'manifest.json').write_text(json.dumps(entries,indent=2))
