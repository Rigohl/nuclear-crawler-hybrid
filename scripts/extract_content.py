#!/usr/bin/env python3
"""
Extractor de contenido real de URLs encontradas en búsquedas
Junta todos los .json y extrae el contenido de las URLs relevantes
"""

import json
import os
import glob
import re
import asyncio
import aiohttp
from pathlib import Path
from datetime import datetime
from collections import defaultdict
from bs4 import BeautifulSoup
import ssl

# Configuración
RESULTADOS_DIR = Path(__file__).parent.parent / "resultados"
OUTPUT_FILE = RESULTADOS_DIR / "CONTENIDO_EXTRAIDO_COMPLETO.json"
MAX_CONCURRENT = 20
TIMEOUT = 30

# URLs a ignorar (páginas de búsqueda, no contenido real)
IGNORE_PATTERNS = [
    r'duckduckgo\.com/html',
    r'bing\.com/search',
    r'google\.com/search',
    r'google\.com/recaptcha',
    r'mojeek\.com/search',
    r'kaggle\.com/search',
    r'dev\.to/search',
    r'arxiv\.org/search',
    r'paperswithcode\.com/search',
    r'reddit\.com/search',
    r'medium\.com/search',
    r'googleapis\.com',
    r'gstatic\.com',
    r'redditstatic\.com',
    r'doubleclick\.net',
    r'google-analytics\.com',
    r'\.css$',
    r'\.png$',
    r'\.ico$',
    r'\.svg$',
    r'\.js$',
    r'manifest\.json',
    r'favicon',
    r'apple-touch-icon',
]

# URLs de alto valor
HIGH_VALUE_DOMAINS = [
    'github.com',
    'stackoverflow.com',
    'ziglang.org',
    'reddit.com/r/',
    'lagerdata.com',
    'zig.guide',
    'handmade.network',
    'learn.microsoft.com',
    'nmichaels.org',
]

def is_valid_url(url: str) -> bool:
    """Verifica si la URL es válida para extraer contenido"""
    if not url or not url.startswith('http'):
        return False
    for pattern in IGNORE_PATTERNS:
        if re.search(pattern, url, re.IGNORECASE):
            return False
    return True

def is_high_value(url: str) -> bool:
    """Verifica si es una URL de alto valor"""
    for domain in HIGH_VALUE_DOMAINS:
        if domain in url:
            return True
    return False

def extract_urls_from_json(json_path: Path) -> list:
    """Extrae URLs únicas de un archivo JSON"""
    urls = []
    try:
        with open(json_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        # Buscar URLs en la estructura
        def find_urls(obj, depth=0):
            if depth > 10:
                return
            if isinstance(obj, dict):
                if 'url' in obj and isinstance(obj['url'], str):
                    url = obj['url']
                    title = obj.get('title', '')
                    description = obj.get('description', '')
                    if is_valid_url(url):
                        urls.append({
                            'url': url,
                            'title': title,
                            'description': description,
                            'source_file': json_path.name,
                            'high_value': is_high_value(url)
                        })
                for v in obj.values():
                    find_urls(v, depth + 1)
            elif isinstance(obj, list):
                for item in obj:
                    find_urls(item, depth + 1)
        
        find_urls(data)
    except Exception as e:
        print(f"Error leyendo {json_path}: {e}")
    
    return urls

async def fetch_content(session: aiohttp.ClientSession, url_info: dict) -> dict:
    """Extrae contenido de una URL"""
    url = url_info['url']
    result = {
        'url': url,
        'title': url_info.get('title', ''),
        'original_description': url_info.get('description', ''),
        'high_value': url_info.get('high_value', False),
        'source_file': url_info.get('source_file', ''),
        'fetched': False,
        'content': '',
        'extracted_text': '',
        'error': None
    }
    
    try:
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
            'Accept-Language': 'en-US,en;q=0.9',
        }
        
        ssl_context = ssl.create_default_context()
        ssl_context.check_hostname = False
        ssl_context.verify_mode = ssl.CERT_NONE
        
        async with session.get(url, headers=headers, timeout=aiohttp.ClientTimeout(total=TIMEOUT), ssl=ssl_context) as response:
            if response.status == 200:
                html = await response.text()
                
                # Parsear HTML
                soup = BeautifulSoup(html, 'html.parser')
                
                # Remover scripts, styles, etc.
                for tag in soup(['script', 'style', 'nav', 'footer', 'header', 'aside', 'noscript']):
                    tag.decompose()
                
                # Extraer título
                title_tag = soup.find('title')
                if title_tag:
                    result['title'] = title_tag.get_text(strip=True)
                
                # Extraer contenido principal
                main_content = ''
                
                # Buscar contenido en orden de prioridad
                content_selectors = [
                    'article',
                    'main',
                    '.post-content',
                    '.article-content',
                    '.entry-content',
                    '.content',
                    '#content',
                    '.markdown-body',  # GitHub
                    '.js-issue-body',  # GitHub issues
                    '.comment-body',   # GitHub comments
                    '.answer',         # Stack Overflow
                    '.post-text',      # Stack Overflow
                    '.s-prose',        # Stack Overflow nuevo
                ]
                
                for selector in content_selectors:
                    elements = soup.select(selector)
                    if elements:
                        for elem in elements:
                            main_content += elem.get_text(separator='\n', strip=True) + '\n\n'
                        break
                
                # Si no encontramos contenedor específico, usar body
                if not main_content:
                    body = soup.find('body')
                    if body:
                        main_content = body.get_text(separator='\n', strip=True)
                
                # Limpiar texto
                lines = main_content.split('\n')
                cleaned_lines = []
                for line in lines:
                    line = line.strip()
                    if line and len(line) > 20:  # Ignorar líneas muy cortas
                        cleaned_lines.append(line)
                
                result['extracted_text'] = '\n'.join(cleaned_lines[:200])  # Limitar a 200 líneas
                result['fetched'] = True
                result['content_length'] = len(result['extracted_text'])
                
            else:
                result['error'] = f"HTTP {response.status}"
                
    except asyncio.TimeoutError:
        result['error'] = "Timeout"
    except Exception as e:
        result['error'] = str(e)[:100]
    
    return result

async def main():
    print("🔍 Buscando archivos JSON en resultados/...")
    
    # Encontrar todos los JSON
    json_files = list(RESULTADOS_DIR.glob("websearch_*.json"))
    print(f"📁 Encontrados {len(json_files)} archivos websearch_*.json")
    
    # Extraer todas las URLs
    all_urls = []
    seen_urls = set()
    
    for json_file in json_files:
        urls = extract_urls_from_json(json_file)
        for url_info in urls:
            if url_info['url'] not in seen_urls:
                seen_urls.add(url_info['url'])
                all_urls.append(url_info)
    
    print(f"🔗 URLs únicas encontradas: {len(all_urls)}")
    
    # Ordenar por prioridad (high_value primero)
    all_urls.sort(key=lambda x: (not x['high_value'], x['url']))
    
    # Filtrar solo las de alto valor para fetch (limitar a 50)
    high_value_urls = [u for u in all_urls if u['high_value']][:50]
    print(f"⭐ URLs de alto valor a procesar: {len(high_value_urls)}")
    
    # Fetch contenido
    results = []
    
    connector = aiohttp.TCPConnector(limit=MAX_CONCURRENT, ssl=False)
    async with aiohttp.ClientSession(connector=connector) as session:
        tasks = [fetch_content(session, url_info) for url_info in high_value_urls]
        
        print(f"⏳ Extrayendo contenido de {len(tasks)} URLs...")
        fetched = await asyncio.gather(*tasks, return_exceptions=True)
        
        for item in fetched:
            if isinstance(item, dict):
                results.append(item)
            else:
                print(f"Error: {item}")
    
    # Estadísticas
    successful = sum(1 for r in results if r.get('fetched'))
    print(f"✅ Contenido extraído exitosamente: {successful}/{len(results)}")
    
    # Crear JSON consolidado
    output = {
        'metadata': {
            'generated_at': datetime.now().isoformat(),
            'source_files': len(json_files),
            'total_unique_urls': len(all_urls),
            'high_value_urls': len(high_value_urls),
            'successfully_fetched': successful,
        },
        'all_urls_summary': [
            {
                'url': u['url'],
                'title': u['title'],
                'high_value': u['high_value']
            } for u in all_urls[:500]  # Primeras 500
        ],
        'extracted_content': [r for r in results if r.get('fetched')],
        'failed_urls': [r for r in results if not r.get('fetched')]
    }
    
    # Guardar
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)
    
    print(f"💾 Guardado en: {OUTPUT_FILE}")
    print(f"📊 Tamaño: {OUTPUT_FILE.stat().st_size / 1024:.1f} KB")

if __name__ == '__main__':
    asyncio.run(main())
