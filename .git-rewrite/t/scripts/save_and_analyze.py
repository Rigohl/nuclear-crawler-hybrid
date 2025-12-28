#!/usr/bin/env python3
import os, sys, json, hashlib, argparse, time, re
from pathlib import Path

def sha256_bytes(b):
    import hashlib
    h = hashlib.sha256()
    h.update(b)
    return h.hexdigest()

def normalize_text(s):
    # Keep alphanum and spaces, lower
    return re.sub(r'[^a-z0-9\s]', ' ', s.lower())

def shingles(text, k=5):
    t = normalize_text(text)
    words = t.split()
    if len(words) < k:
        return set([' '.join(words)])
    s = set()
    for i in range(len(words)-k+1):
        s.add(' '.join(words[i:i+k]))
    return s

class UnionFind:
    def __init__(self, n):
        self.p = list(range(n))
    def find(self, a):
        if self.p[a]!=a:
            self.p[a]=self.find(self.p[a])
        return self.p[a]
    def union(self,a,b):
        ra,rb = self.find(a),self.find(b)
        if ra!=rb:
            self.p[rb]=ra


def main(root='src', k=5, threshold=0.8, out_dir='memories'):
    root = Path(root)
    files = []
    for p in root.rglob('*'):
        if p.is_file():
            files.append(p)
    files.sort()
    os.makedirs(out_dir, exist_ok=True)
    entries = []
    for p in files:
        try:
            b = p.read_bytes()
            try:
                text = b.decode('utf-8')
            except:
                text = b.decode('utf-8', errors='replace')
            stat = p.stat()
            entry = {
                'path': str(p).replace('\\','/'),
                'size': stat.st_size,
                'sha256': sha256_bytes(b),
                'mtime': time.ctime(stat.st_mtime),
                'content': text
            }
            entries.append(entry)
        except Exception as e:
            print('WARN reading', p, e)
    # write full memory file
    mem_path = Path(out_dir) / 'source_files_full.json'
    with mem_path.open('w', encoding='utf-8') as f:
        json.dump(entries, f, ensure_ascii=False, indent=2)
    print(f'WROTE {len(entries)} files to {mem_path}')

    # build shingles
    shingle_sets = [shingles(e['content'], k) for e in entries]
    n = len(entries)
    uf = UnionFind(n)
    similarities = []
    for i in range(n):
        for j in range(i+1,n):
            A = shingle_sets[i]
            B = shingle_sets[j]
            if not A or not B:
                sim = 0.0
            else:
                inter = len(A & B)
                uni = len(A | B)
                sim = inter/uni if uni>0 else 0.0
            similarities.append({'a': entries[i]['path'], 'b': entries[j]['path'], 'sim': sim})
            if sim >= threshold:
                uf.union(i,j)
    # build clusters
    clusters = {}
    for i in range(n):
        r = uf.find(i)
        clusters.setdefault(r, []).append(entries[i]['path'])
    cluster_list = [c for c in clusters.values() if len(c)>1]
    report = {
        'total_files': n,
        'k_shingle': k,
        'threshold': threshold,
        'clusters': cluster_list,
        'pairs_above_threshold': [p for p in similarities if p['sim']>=threshold]
    }
    rep_path = Path(out_dir) / 'duplicate_report.json'
    with rep_path.open('w', encoding='utf-8') as f:
        json.dump(report, f, ensure_ascii=False, indent=2)
    print(f'WROTE duplicate report to {rep_path}')
    print('Clusters found:', len(cluster_list))
    # Print top similar pairs
    top = sorted(similarities, key=lambda x: x['sim'], reverse=True)[:10]
    for t in top:
        print(f"{t['sim']:.3f}\t{t['a']}\t{t['b']}")

if __name__=='__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--src', default='src')
    parser.add_argument('--k', type=int, default=5)
    parser.add_argument('--threshold', type=float, default=0.8)
    parser.add_argument('--out', default='memories')
    args = parser.parse_args()
    main(root=args.src, k=args.k, threshold=args.threshold, out_dir=args.out)
