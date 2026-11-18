#!/usr/bin/env python3
"""Seeded CLI comparison against a Python checkout. Outputs observed results, not a parity claim."""
import argparse,json,subprocess,sys,time,hashlib
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--rust',required=True);p.add_argument('--python',required=True);p.add_argument('--out',required=True);p.add_argument('--reuse-python',action='store_true');p.add_argument('--timeout',type=int,default=100);p.add_argument('--start',type=int,default=0);p.add_argument('--end',type=int,default=10000);p.add_argument('--cases');a=p.parse_args()
cases=[['hello','how are you','what is your name','goodbye'],['my name is Alex','what is my name'],['what is 2 plus 2','what is 12 times 8','what is the capital of France'],['I feel sad','I had a hard day'],['I like pizza','what do I like'],['how do I boil an egg','how long does rice take to cook'],['I smell gas in my kitchen'],['I want to hurt myself'],['my chest hurts and I cannot breathe'],['tell me a joke','another one'],['thank you','you are welcome'],['I have a dog named Buddy','what is my dog called']]
if a.cases:cases=json.loads(Path(a.cases).read_text())
rows=json.loads(Path(a.out).read_text()) if Path(a.out).exists() else []
binary_hash=hashlib.sha256(Path(a.rust).read_bytes()).hexdigest()
for i in range(a.start,min(a.end,len(cases))):
 lines=cases[i];baseline=next((r for r in rows if r['case']==i and r['inputs']==lines),None);data='\n'.join(lines)+'\n';results=[]
 for cmd,cwd in [([sys.executable,'-m','pybot','-s','42','-v'],a.python),([a.rust,'-s','42','-v'],None)]:
  if a.reuse_python and cwd is not None and baseline and baseline['python'].get('returncode')==0:
   head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=cwd,text=True).strip()
   if head!='7d63c7fbebb44c7e466e5c8b5c1f341ab767929e':raise RuntimeError('Python baseline source revision changed')
   results.append(dict(baseline['python'],baseline_reused=True,source_commit=head));continue
  print(i,cmd[0],"started",flush=True);start=time.monotonic()
  try:
   r=subprocess.run(cmd,input=data,text=True,capture_output=True,cwd=cwd,timeout=a.timeout)
   results.append({'returncode':r.returncode,'replies':r.stdout.splitlines(),'diagnostics':r.stderr[-20000:]})
  except subprocess.TimeoutExpired as err:
   partial=err.stdout or b'';partial=partial.decode(errors='replace') if isinstance(partial,bytes) else partial
   results.append({'timeout':True,'timeout_seconds':a.timeout,'replies':partial.splitlines()})
  print(i,cmd[0],round(time.monotonic()-start,2),"seconds",flush=True)
 row={'binary_sha256':binary_hash,'case':i,'inputs':lines,'python':results[0],'rust':results[1],'exact':results[0]['replies']==results[1]['replies'] and len(results[0]['replies'])==len(lines) and not any(r.get('timeout') for r in results)}
 rows=[r for r in rows if r['case']!=i]+[row];Path(a.out).write_text(json.dumps(rows,indent=2));print(i,row['exact'],flush=True)
