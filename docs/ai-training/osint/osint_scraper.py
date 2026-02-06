#!/usr/bin/env python3
"""
OSINT Nuclear Scraper
Scraping masivo para inteligencia de fuentes abiertas
Incluye: teléfonos, chats, redes sociales, foros, leaks
"""

import asyncio
import aiohttp
import json
import re
from typing import List, Dict, Set
from dataclasses import dataclass, asdict
from playwright.async_api import async_playwright
import os

@dataclass
class OSINTTarget:
    """Target de scraping OSINT"""
    name: str
    phone: str = None
    email: str = None
    social_profiles: Dict[str, str] = None
    locations: List[str] = None
    associated_data: Dict = None

class NuclearOSINTScraper:
    """Scraper OSINT de alto rendimiento"""
    
    def __init__(self, stealth: bool = True):
        self.stealth = stealth
        self.results: List[OSINTTarget] = []
        self.proxies = self._load_proxies()
        
    def _load_proxies(self) -> List[str]:
        """Carga proxies para rotación"""
        # TODO: Implementar rotación de proxies
        return []
    
    async def scrape_phone_databases(self, phone: str) -> Dict:
        """
        Busca número en bases de datos públicas
        - TrueCaller (API pública)
        - WhitePages
        - SpyDialer
        - Pipl
        """
        results = {
            'phone': phone,
            'found_in': [],
            'associated_names': [],
            'locations': [],
            'social_profiles': {}
        }
        
        # TrueCaller scraping (requiere API key o scraping)
        truecaller_data = await self._scrape_truecaller(phone)
        if truecaller_data:
            results['found_in'].append('TrueCaller')
            results['associated_names'].extend(truecaller_data.get('names', []))
        
        # WhitePages
        whitepages_data = await self._scrape_whitepages(phone)
        if whitepages_data:
            results['found_in'].append('WhitePages')
            results['locations'].extend(whitepages_data.get('locations', []))
        
        return results
    
    async def scrape_social_media_by_phone(self, phone: str) -> Dict:
        """
        Busca perfiles sociales asociados a un teléfono
        - Facebook (phone search)
        - Instagram
        - Telegram
        - WhatsApp Business
        """
        profiles = {}
        
        # Facebook phone search
        fb_profile = await self._scrape_facebook_by_phone(phone)
        if fb_profile:
            profiles['facebook'] = fb_profile
        
        # Telegram (t.me/+phone)
        tg_profile = await self._scrape_telegram_by_phone(phone)
        if tg_profile:
            profiles['telegram'] = tg_profile
        
        # WhatsApp (wa.me/phone)
        wa_profile = await self._check_whatsapp_exists(phone)
        if wa_profile:
            profiles['whatsapp'] = wa_profile
        
        return profiles
    
    async def scrape_email_osint(self, email: str) -> Dict:
        """
        OSINT de email
        - Have I Been Pwned
        - Hunter.io
        - EmailRep
        - Pipl
        """
        results = {
            'email': email,
            'breaches': [],
            'social_profiles': {},
            'domain_info': {},
            'reputation': {}
        }
        
        # Have I Been Pwned
        breaches = await self._check_hibp(email)
        results['breaches'] = breaches
        
        # EmailRep (reputación)
        rep = await self._check_emailrep(email)
        results['reputation'] = rep
        
        return results
    
    async def scrape_username_osint(self, username: str) -> Dict:
        """
        Busca username en TODAS las plataformas
        Usando: https://github.com/sherlock-project/sherlock
        """
        platforms = [
            'Instagram', 'Twitter', 'Facebook', 'LinkedIn',
            'GitHub', 'Reddit', 'TikTok', 'Snapchat',
            'Telegram', 'Discord', 'Twitch', 'YouTube',
            'Medium', 'Dev.to', 'Stack Overflow', 'HackerOne',
            'Behance', 'Dribbble', 'Pinterest', 'Flickr'
        ]
        
        results = {
            'username': username,
            'found_on': [],
            'profiles': {}
        }
        
        async with aiohttp.ClientSession() as session:
            tasks = [
                self._check_username_on_platform(session, username, platform)
                for platform in platforms
            ]
            
            platform_results = await asyncio.gather(*tasks)
            
            for platform, data in zip(platforms, platform_results):
                if data:
                    results['found_on'].append(platform)
                    results['profiles'][platform] = data
        
        return results
    
    async def scrape_leaked_databases(self, query: str) -> List[Dict]:
        """
        Busca en bases de datos filtradas (LEGALMENTE accesibles)
        - Vigilante.pw
        - LeakCheck
        - Dehashed
        """
        results = []
        
        # LeakCheck API
        leaks = await self._search_leakcheck(query)
        results.extend(leaks)
        
        return results
    
    async def scrape_forum_posts(self, keywords: List[str]) -> List[Dict]:
        """
        Scraping de foros con conversaciones reales
        - Reddit (ya implementado)
        - HackerNews
        - Quora
        - Stack Overflow
        - 4chan archives
        - Foros de nicho
        """
        all_posts = []
        
        # HackerNews
        hn_posts = await self._scrape_hackernews(keywords)
        all_posts.extend(hn_posts)
        
        # Quora
        quora_posts = await self._scrape_quora(keywords)
        all_posts.extend(quora_posts)
        
        return all_posts
    
    async def scrape_chat_logs(self, platform: str, target: str) -> List[Dict]:
        """
        Scraping de logs de chats públicos
        - Telegram public groups/channels
        - Discord public servers
        - Slack public channels
        """
        if platform == 'telegram':
            return await self._scrape_telegram_channel(target)
        elif platform == 'discord':
            return await self._scrape_discord_server(target)
        
        return []
    
    # === IMPLEMENTACIONES ESPECÍFICAS ===
    
    async def _scrape_truecaller(self, phone: str) -> Dict:
        """Scrape TrueCaller (requiere account o API)"""
        # TODO: Implementar scraping con Playwright
        return {}
    
    async def _scrape_whitepages(self, phone: str) -> Dict:
        """Scrape WhitePages"""
        async with async_playwright() as p:
            browser = await p.chromium.launch(headless=self.stealth)
            page = await browser.new_page()
            
            try:
                url = f"https://www.whitepages.com/phone/{phone}"
                await page.goto(url, wait_until='networkidle')
                
                # Extraer info
                name = await page.locator('.name').inner_text() if await page.locator('.name').count() > 0 else None
                location = await page.locator('.location').inner_text() if await page.locator('.location').count() > 0 else None
                
                await browser.close()
                
                return {
                    'name': name,
                    'locations': [location] if location else []
                }
            except:
                await browser.close()
                return {}
    
    async def _scrape_facebook_by_phone(self, phone: str) -> Dict:
        """Busca perfil de Facebook por teléfono"""
        # Requiere sesión de Facebook activa
        # TODO: Implementar con cookies guardadas
        return {}
    
    async def _scrape_telegram_by_phone(self, phone: str) -> Dict:
        """Busca Telegram por teléfono"""
        # Telegram no expone búsqueda pública por teléfono
        # Pero se puede verificar si existe: t.me/+phone
        async with aiohttp.ClientSession() as session:
            url = f"https://t.me/+{phone}"
            async with session.get(url) as resp:
                if resp.status == 200:
                    return {'url': url, 'exists': True}
        return {}
    
    async def _check_whatsapp_exists(self, phone: str) -> Dict:
        """Verifica si número existe en WhatsApp"""
        # wa.me/phone redirects si existe
        async with aiohttp.ClientSession() as session:
            url = f"https://wa.me/{phone}"
            async with session.get(url, allow_redirects=False) as resp:
                if resp.status in [200, 302]:
                    return {'url': url, 'exists': True}
        return {}
    
    async def _check_hibp(self, email: str) -> List[Dict]:
        """Have I Been Pwned API"""
        async with aiohttp.ClientSession() as session:
            url = f"https://haveibeenpwned.com/api/v3/breachedaccount/{email}"
            headers = {
                'User-Agent': 'NuclearOSINT-Research',
                'hibp-api-key': os.getenv('HIBP_API_KEY', '')
            }
            
            try:
                async with session.get(url, headers=headers) as resp:
                    if resp.status == 200:
                        return await resp.json()
            except:
                pass
        
        return []
    
    async def _check_emailrep(self, email: str) -> Dict:
        """EmailRep.io API"""
        async with aiohttp.ClientSession() as session:
            url = f"https://emailrep.io/{email}"
            
            try:
                async with session.get(url) as resp:
                    if resp.status == 200:
                        return await resp.json()
            except:
                pass
        
        return {}
    
    async def _check_username_on_platform(self, session: aiohttp.ClientSession, 
                                         username: str, platform: str) -> Dict:
        """Verifica username en plataforma específica"""
        urls = {
            'Instagram': f'https://www.instagram.com/{username}/',
            'Twitter': f'https://twitter.com/{username}',
            'GitHub': f'https://github.com/{username}',
            'Reddit': f'https://www.reddit.com/user/{username}',
            'LinkedIn': f'https://www.linkedin.com/in/{username}',
            # ... más plataformas
        }
        
        url = urls.get(platform)
        if not url:
            return {}
        
        try:
            async with session.get(url) as resp:
                if resp.status == 200:
                    return {
                        'url': url,
                        'exists': True,
                        'status_code': resp.status
                    }
        except:
            pass
        
        return {}
    
    async def _search_leakcheck(self, query: str) -> List[Dict]:
        """LeakCheck API"""
        # TODO: Implementar con API key
        return []
    
    async def _scrape_hackernews(self, keywords: List[str]) -> List[Dict]:
        """Scraping HackerNews Algolia API"""
        posts = []
        
        async with aiohttp.ClientSession() as session:
            for keyword in keywords:
                url = f"https://hn.algolia.com/api/v1/search?query={keyword}&tags=story"
                
                try:
                    async with session.get(url) as resp:
                        if resp.status == 200:
                            data = await resp.json()
                            for hit in data.get('hits', []):
                                posts.append({
                                    'title': hit.get('title'),
                                    'url': hit.get('url'),
                                    'author': hit.get('author'),
                                    'points': hit.get('points'),
                                    'num_comments': hit.get('num_comments')
                                })
                except:
                    continue
        
        return posts
    
    async def _scrape_quora(self, keywords: List[str]) -> List[Dict]:
        """Scraping Quora"""
        # TODO: Implementar con Playwright
        return []
    
    async def _scrape_telegram_channel(self, channel: str) -> List[Dict]:
        """Scraping de canal público de Telegram"""
        messages = []
        
        async with async_playwright() as p:
            browser = await p.chromium.launch(headless=self.stealth)
            page = await browser.new_page()
            
            try:
                await page.goto(f"https://t.me/s/{channel}", wait_until='networkidle')
                
                # Scroll para cargar mensajes
                for _ in range(10):
                    await page.mouse.wheel(0, 5000)
                    await asyncio.sleep(1)
                
                # Extraer mensajes
                message_elements = await page.locator('.tgme_widget_message').all()
                
                for elem in message_elements:
                    try:
                        text = await elem.locator('.tgme_widget_message_text').inner_text()
                        author = await elem.locator('.tgme_widget_message_author').inner_text()
                        date = await elem.locator('.tgme_widget_message_date time').get_attribute('datetime')
                        
                        messages.append({
                            'platform': 'Telegram',
                            'channel': channel,
                            'author': author,
                            'text': text,
                            'date': date
                        })
                    except:
                        continue
                
                await browser.close()
                
            except Exception as e:
                await browser.close()
                print(f"Error scraping Telegram channel: {e}")
        
        return messages
    
    async def _scrape_discord_server(self, server_id: str) -> List[Dict]:
        """Scraping Discord (requiere bot token)"""
        # TODO: Implementar con discord.py
        return []
    
    def save_results(self, filename: str = 'osint_results.json'):
        """Guardar resultados"""
        output_dir = 'ai-training/osint/output'
        os.makedirs(output_dir, exist_ok=True)
        
        filepath = os.path.join(output_dir, filename)
        
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump([asdict(r) for r in self.results], f, indent=2, ensure_ascii=False)
        
        print(f"✅ Resultados guardados: {filepath}")


async def main():
    """Demo de uso"""
    print("=" * 70)
    print("  NUCLEAR OSINT SCRAPER")
    print("  Scraping masivo para inteligencia de fuentes abiertas")
    print("=" * 70)
    print()
    
    scraper = NuclearOSINTScraper(stealth=True)
    
    # Ejemplo: Buscar username en todas las plataformas
    print("🔍 Buscando username...")
    results = await scraper.scrape_username_osint("example_user")
    print(json.dumps(results, indent=2))
    
    # Ejemplo: Scraping de canal de Telegram
    print("\n📥 Scraping Telegram channel...")
    messages = await scraper.scrape_chat_logs('telegram', 'example_channel')
    print(f"   Mensajes extraídos: {len(messages)}")


if __name__ == '__main__':
    asyncio.run(main())
