#!/usr/bin/env python3
"""
🤖 ULTRA TOKEN BOT v3.0 - BOT REAL DE AHORRO DE TOKENS
======================================================
Un único archivo que hace TODO:
- Intercepta prompts en tiempo real
- Optimiza y comprime tokens REALMENTE
- Muestra efectos visuales
- Guarda cache persistente
- Funciona como servidor HTTP para integración
- MODO INTERCEPTOR: Lee de stdin, optimiza, envía a stdout
"""

import os, sys, json, time, hashlib, re, http.server, socketserver, threading, signal, subprocess
from dataclasses import dataclass, field
from collections import deque
from typing import Dict, List, Any, Tuple
from urllib.parse import parse_qs

# ═══════════════════════════════════════════════════════════════
# 🎨 COLORES Y EFECTOS
# ═══════════════════════════════════════════════════════════════
class C:
    RST = '\033[0m'; B = '\033[1m'; DIM = '\033[2m'
    RED = '\033[91m'; GRN = '\033[92m'; YLW = '\033[93m'
    BLU = '\033[94m'; MAG = '\033[95m'; CYN = '\033[96m'; WHT = '\033[97m'
    
    @staticmethod
    def c(t, col): return f"{col}{t}{C.RST}"
    
    @staticmethod
    def rainbow(t):
        cols = [C.RED, C.YLW, C.GRN, C.CYN, C.BLU, C.MAG]
        return ''.join(f"{cols[i%6]}{ch}{C.RST}" for i, ch in enumerate(t))
    
    @staticmethod
    def bar(cur, tot, w=30):
        p = cur/tot if tot else 0
        f = int(w*p)
        col = C.GRN if p >= 0.7 else C.YLW if p >= 0.4 else C.RED
        return f"{col}{'█'*f}{'░'*(w-f)}{C.RST} {p*100:.0f}%"
    
    @staticmethod
    def box(txt, title=""):
        lines = txt.split('\n')
        w = max(len(l) for l in lines + [title]) + 2
        r = [f"╔{'═'*w}╗"]
        if title: r += [f"║ {C.c(title.center(w-2), C.CYN)} ║", f"╠{'═'*w}╣"]
        r += [f"║ {l.ljust(w-2)} ║" for l in lines]
        r += [f"╚{'═'*w}╝"]
        return '\n'.join(r)

# ═══════════════════════════════════════════════════════════════
# 🧠 MOTOR DE OPTIMIZACIÓN REAL
# ═══════════════════════════════════════════════════════════════
@dataclass
class OptResult:
    original: str
    optimized: str
    orig_tokens: int
    opt_tokens: int
    saved: int
    ratio: float
    techniques: List[str]
    time_ms: float

class TokenOptimizer:
    """Motor de optimización REAL de tokens"""
    
    # Patrones de compresión REALES
    COMPRESS_PATTERNS = [
        # Frases largas → cortas
        (r'\bes decir\b', 'i.e.'),
        (r'\bpor ejemplo\b', 'ej.'),
        (r'\bten en cuenta que\b', 'nota:'),
        (r'\bes importante mencionar que\b', '⚠️'),
        (r'\bde acuerdo con\b', 'según'),
        (r'\ben otras palabras\b', 'o sea'),
        (r'\bcon el fin de\b', 'para'),
        (r'\bcon el objetivo de\b', 'para'),
        (r'\bdebido a que\b', 'porque'),
        (r'\ba pesar de que\b', 'aunque'),
        (r'\ben el caso de que\b', 'si'),
        (r'\bsiempre y cuando\b', 'si'),
        (r'\bpor lo tanto\b', '∴'),
        (r'\bsin embargo\b', 'pero'),
        (r'\bademás de eso\b', 'además'),
        (r'\ben primer lugar\b', '1)'),
        (r'\ben segundo lugar\b', '2)'),
        (r'\bpor último\b', 'finalmente'),
        (r'\bcomo resultado\b', '→'),
        (r'\bfor example\b', 'e.g.'),
        (r'\bthat is to say\b', 'i.e.'),
        (r'\bin order to\b', 'to'),
        (r'\bdue to the fact that\b', 'because'),
        (r'\bin spite of\b', 'despite'),
        (r'\bwith regard to\b', 're:'),
        (r'\bin addition to\b', 'also'),
        (r'\bas a result\b', '→'),
        (r'\bnevertheless\b', 'but'),
        (r'\bfurthermore\b', 'also'),
        # Espacios y redundancias
        (r'\s+', ' '),
        (r'\s*,\s*,\s*', ', '),
        (r'\.+', '.'),
        (r'\s+\.', '.'),
    ]
    
    # Palabras de relleno a eliminar
    FILLER_WORDS = [
        r'\brealmente\b', r'\bbásicamente\b', r'\bliteralmente\b',
        r'\bactualmente\b', r'\bprácticamente\b', r'\bdefinitivamente\b',
        r'\bobviamente\b', r'\bciertamente\b', r'\bpersonalmente\b',
        r'\breally\b', r'\bbasically\b', r'\bliterally\b', r'\bactually\b',
        r'\bpractically\b', r'\bdefinitely\b', r'\bobviously\b', r'\bcertainly\b',
        r'\bjust\b', r'\bvery\b', r'\bquite\b', r'\bsimply\b',
    ]
    
    def __init__(self):
        self.cache = {}
        self.cache_file = os.path.join(os.path.dirname(__file__), 'token_cache.json')
        self._load_cache()
        self.stats = {'total': 0, 'saved': 0, 'calls': 0}
    
    def _load_cache(self):
        try:
            if os.path.exists(self.cache_file):
                with open(self.cache_file, 'r', encoding='utf-8') as f:
                    self.cache = json.load(f)
        except: pass
    
    def _save_cache(self):
        try:
            with open(self.cache_file, 'w', encoding='utf-8') as f:
                json.dump(self.cache, f, ensure_ascii=False)
        except: pass
    
    def count_tokens(self, text: str) -> int:
        """Conteo REAL de tokens (aproximación GPT)"""
        # ~4 chars por token en promedio, ajustado por palabras
        words = len(text.split())
        chars = len(text)
        return max(1, int((chars / 4 + words) / 2))
    
    def optimize(self, text: str) -> OptResult:
        """Optimización REAL del prompt"""
        start = time.time()
        techniques = []
        
        # Check cache
        h = hashlib.md5(text.encode()).hexdigest()[:16]
        if h in self.cache:
            cached = self.cache[h]
            return OptResult(**cached, time_ms=(time.time()-start)*1000)
        
        original = text
        orig_tokens = self.count_tokens(text)
        
        # 1. Eliminar espacios múltiples
        text = re.sub(r'\s+', ' ', text.strip())
        if text != original:
            techniques.append('whitespace_cleanup')
        
        # 2. Eliminar palabras de relleno
        for pattern in self.FILLER_WORDS:
            if re.search(pattern, text, re.I):
                text = re.sub(pattern, '', text, flags=re.I)
                techniques.append('filler_removal')
                break
        
        # 3. Aplicar compresiones
        for pattern, replacement in self.COMPRESS_PATTERNS:
            if re.search(pattern, text, re.I):
                text = re.sub(pattern, replacement, text, flags=re.I)
                if 'phrase_compression' not in techniques:
                    techniques.append('phrase_compression')
        
        # 4. Eliminar oraciones duplicadas
        sentences = [s.strip() for s in re.split(r'[.!?]+', text) if s.strip()]
        seen = set()
        unique = []
        for s in sentences:
            key = s.lower()
            if key not in seen:
                seen.add(key)
                unique.append(s)
        if len(unique) < len(sentences):
            text = '. '.join(unique) + ('.' if unique else '')
            techniques.append('deduplication')
        
        # 5. Limpiar espacios finales
        text = re.sub(r'\s+', ' ', text.strip())
        
        opt_tokens = self.count_tokens(text)
        saved = orig_tokens - opt_tokens
        ratio = opt_tokens / orig_tokens if orig_tokens > 0 else 1.0
        
        # Actualizar stats
        self.stats['total'] += orig_tokens
        self.stats['saved'] += saved
        self.stats['calls'] += 1
        
        result = OptResult(
            original=original,
            optimized=text,
            orig_tokens=orig_tokens,
            opt_tokens=opt_tokens,
            saved=saved,
            ratio=ratio,
            techniques=techniques or ['already_optimal'],
            time_ms=(time.time()-start)*1000
        )
        
        # Guardar en cache
        self.cache[h] = {
            'original': original, 'optimized': text,
            'orig_tokens': orig_tokens, 'opt_tokens': opt_tokens,
            'saved': saved, 'ratio': ratio, 'techniques': techniques or ['already_optimal']
        }
        self._save_cache()
        
        return result

# ═══════════════════════════════════════════════════════════════
# 🌐 SERVIDOR HTTP (para integración externa)
# ═══════════════════════════════════════════════════════════════
class TokenAPIHandler(http.server.BaseHTTPRequestHandler):
    optimizer = TokenOptimizer()
    
    def log_message(self, format, *args): pass  # Silenciar logs
    
    def _send_json(self, data, code=200):
        self.send_response(code)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, ensure_ascii=False).encode())
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
    
    def do_GET(self):
        if self.path == '/stats':
            self._send_json(self.optimizer.stats)
        elif self.path == '/health':
            self._send_json({'status': 'ok', 'version': '3.0'})
        else:
            self._send_json({'error': 'Use POST /optimize with {"text": "..."}'}, 400)
    
    def do_POST(self):
        if self.path == '/optimize':
            length = int(self.headers.get('Content-Length', 0))
            body = self.rfile.read(length).decode()
            try:
                data = json.loads(body)
                text = data.get('text', '')
                if not text:
                    self._send_json({'error': 'text required'}, 400)
                    return
                result = self.optimizer.optimize(text)
                self._send_json({
                    'optimized': result.optimized,
                    'original_tokens': result.orig_tokens,
                    'optimized_tokens': result.opt_tokens,
                    'saved_tokens': result.saved,
                    'compression_ratio': round(result.ratio, 3),
                    'techniques': result.techniques,
                    'time_ms': round(result.time_ms, 2)
                })
            except Exception as e:
                self._send_json({'error': str(e)}, 500)
        else:
            self._send_json({'error': 'Not found'}, 404)

# ═══════════════════════════════════════════════════════════════
# 🤖 INTERFAZ PRINCIPAL
# ═══════════════════════════════════════════════════════════════
class UltraTokenBot:
    def __init__(self):
        self.optimizer = TokenOptimizer()
        self.server = None
        self.server_thread = None
        self.running = True
    
    def banner(self):
        os.system('cls' if os.name == 'nt' else 'clear')
        print(f"""
{C.c('╔════════════════════════════════════════════════════════════════╗', C.CYN)}
{C.c('║', C.CYN)}  {C.rainbow('🤖 ULTRA TOKEN BOT v3.0')} - Ahorro REAL de Tokens           {C.c('║', C.CYN)}
{C.c('╠════════════════════════════════════════════════════════════════╣', C.CYN)}
{C.c('║', C.CYN)} ⚡ Compresión REAL    🧠 Optimización Inteligente   💾 Cache  {C.c('║', C.CYN)}
{C.c('╚════════════════════════════════════════════════════════════════╝', C.CYN)}
""")
    
    def start_server(self, port=8765):
        """Iniciar servidor HTTP en background"""
        try:
            self.server = socketserver.TCPServer(('', port), TokenAPIHandler)
            self.server_thread = threading.Thread(target=self.server.serve_forever, daemon=True)
            self.server_thread.start()
            print(C.c(f"  🌐 API Server: http://localhost:{port}", C.GRN))
            print(C.c(f"     POST /optimize {'{'}\"text\": \"...\"{'}'}",  C.DIM))
            return True
        except Exception as e:
            print(C.c(f"  ⚠️ Server no disponible: {e}", C.YLW))
            return False
    
    def show_result(self, r: OptResult):
        """Mostrar resultado con efectos"""
        # Animación de progreso
        print()
        for i in range(21):
            cur = r.orig_tokens - int(r.saved * i / 20)
            print(f"\r  ⚡ Optimizando: {C.bar(20-i, 20)} {cur} tokens", end='', flush=True)
            time.sleep(0.02)
        print()
        
        # Resultado
        save_pct = (1 - r.ratio) * 100
        col = C.GRN if save_pct > 20 else C.YLW if save_pct > 10 else C.CYN
        emoji = '🏆' if save_pct > 30 else '⭐' if save_pct > 15 else '✅'
        
        print(C.box(
            f"Original:   {r.orig_tokens} tokens\n"
            f"Optimizado: {r.opt_tokens} tokens\n"
            f"Ahorro:     {C.c(f'{r.saved} tokens ({save_pct:.1f}%)', col)}\n"
            f"Tiempo:     {r.time_ms:.1f}ms\n"
            f"Técnicas:   {', '.join(r.techniques)}",
            f"{emoji} RESULTADO"
        ))
        
        # Mostrar prompt optimizado
        print(f"\n  {C.c('📝 Optimizado:', C.GRN)}")
        print(C.c(f"  {r.optimized[:300]}{'...' if len(r.optimized) > 300 else ''}", C.WHT))
        print()
    
    def copy_to_clipboard(self, text: str):
        """Copia texto al clipboard en Windows usando clip.exe"""
        try:
            # Usar clip.exe que lee de stdin
            with subprocess.Popen(['clip'], stdin=subprocess.PIPE, text=True) as proc:
                proc.communicate(text)
            return True
        except Exception as e:
            print(f"Debug: clip failed {e}", file=sys.stderr)
            return False
    
    def run(self, interceptor_mode=False):
        """Loop principal o modo interceptor"""
        if interceptor_mode:
            self.run_interceptor()
            return
        
        self.banner()
        
        # Iniciar servidor
        print(C.c("  🔄 Iniciando sistemas...", C.DIM))
        time.sleep(0.3)
        self.start_server()
        
        # Stats
        print(C.c(f"  📊 Cache: {len(self.optimizer.cache)} prompts guardados", C.DIM))
        print()
        print(C.c("  💬 Comandos: 'stats' | 'demo' | 'clear' | 'exit'", C.CYN))
        print(C.c("  ─" * 35, C.DIM))
        
        while self.running:
            try:
                inp = input(f"\n  {C.c('👤 Prompt', C.YLW)} {C.c('❯', C.CYN)} ").strip()
                
                if not inp: continue
                
                if inp.lower() == 'exit':
                    print(C.c("\n  👋 ¡Hasta luego!", C.CYN))
                    stats = self.optimizer.stats
                    if stats['calls'] > 0:
                        print(C.box(
                            f"Llamadas: {stats['calls']}\n"
                            f"Tokens procesados: {stats['total']}\n"
                            f"Tokens ahorrados: {stats['saved']}\n"
                            f"Eficiencia: {stats['saved']/max(1,stats['total'])*100:.1f}%",
                            "📊 RESUMEN FINAL"
                        ))
                    break
                
                if inp.lower() == 'stats':
                    s = self.optimizer.stats
                    print(C.box(
                        f"Llamadas: {s['calls']}\n"
                        f"Tokens: {s['total']}\n"
                        f"Ahorrados: {s['saved']}\n"
                        f"Cache: {len(self.optimizer.cache)}",
                        "📊 ESTADÍSTICAS"
                    ))
                    continue
                
                if inp.lower() == 'clear':
                    self.banner()
                    continue
                
                if inp.lower() == 'demo':
                    inp = "Necesito que me expliques de manera muy detallada y extensa cómo funciona realmente el sistema de optimización de prompts, es decir, quiero saber básicamente todos los detalles técnicos posibles, por ejemplo, las técnicas que se usan, ten en cuenta que es muy importante que seas exhaustivo, sin embargo también quiero que sea claro."
                    print(C.c(f"  🎭 Demo ({len(inp)} chars)", C.MAG))
                
                # OPTIMIZAR
                result = self.optimizer.optimize(inp)
                self.show_result(result)
                
            except KeyboardInterrupt:
                print(C.c("\n\n  ⚠️ Interrumpido", C.YLW))
                break
            except Exception as e:
                print(C.c(f"  ❌ Error: {e}", C.RED))
        
    def run_interceptor(self):
        """Modo interceptor: lee de stdin, optimiza, envía a stdout y clipboard"""
        print(C.c("  🚀 MODO INTERCEPTOR ACTIVADO", C.GRN))
        print(C.c("  Escribe prompts, presiona Enter, el bot optimiza y copia al clipboard", C.DIM))
        print(C.c("  Ctrl+C para salir", C.DIM))
        print()
        
        try:
            for line in sys.stdin:
                prompt = line.strip()
                if not prompt:
                    continue
                
                # Optimizar
                result = self.optimizer.optimize(prompt)
                
                # Mostrar ahorro rápido
                save_pct = (1 - result.ratio) * 100
                status = f"💾 {result.saved} tokens ({save_pct:.1f}%)"
                print(C.c(f"[{status}] ", C.GRN), end='', file=sys.stderr)
                
                # Copiar al clipboard
                if self.copy_to_clipboard(result.optimized):
                    print(C.c("📋 Copiado!", C.CYN), file=sys.stderr)
                else:
                    print(C.c("⚠️ No se pudo copiar", C.YLW), file=sys.stderr)
                
                # Enviar optimizado a stdout (para piping)
                print(result.optimized)
                
        except KeyboardInterrupt:
            print(C.c("\n  👋 Modo interceptor terminado", C.CYN), file=sys.stderr)
        except Exception as e:
            print(C.c(f"  ❌ Error: {e}", C.RED), file=sys.stderr)
if __name__ == "__main__":
    interceptor_mode = '--intercept' in sys.argv
    
    bot = UltraTokenBot()
    bot.run(interceptor_mode=interceptor_mode)
