#!/usr/bin/env python3
"""
Reddit Real Conversations Scraper
Extrae conversaciones AUTÉNTICAS de marketing, ventas y negocios
"""

import praw
import json
import os
from datetime import datetime
from typing import List, Dict
import time

class RedditRealScraper:
    """Scraper de conversaciones reales de Reddit"""
    
    def __init__(self, client_id: str = None, client_secret: str = None, user_agent: str = None):
        # Usar env vars si no se pasan parámetros
        self.client_id = client_id or os.getenv('REDDIT_CLIENT_ID')
        self.client_secret = client_secret or os.getenv('REDDIT_CLIENT_SECRET')
        self.user_agent = user_agent or os.getenv('REDDIT_USER_AGENT', 'ChapelTrainingBot/1.0')
        
        if not self.client_id or not self.client_secret:
            print("⚠️  SETUP REQUERIDO:")
            print("   1. Ve a: https://www.reddit.com/prefs/apps")
            print("   2. Click 'create app' → script type")
            print("   3. Guarda CLIENT_ID y CLIENT_SECRET en .env")
            return
        
        self.reddit = praw.Reddit(
            client_id=self.client_id,
            client_secret=self.client_secret,
            user_agent=self.user_agent
        )
    
    def scrape_marketing_conversations(self, limit: int = 1000) -> List[Dict]:
        """Extrae conversaciones reales de marketing"""
        
        # Subreddits con conversaciones AUTÉNTICAS
        subreddits = [
            'sales',                    # Vendedores reales compartiendo técnicas
            'marketing',                # Marketers discutiendo estrategias
            'entrepreneur',             # Emprendedores con problemas reales
            'smallbusiness',            # Pequeños negocios
            'Copywriting',              # Copywriters profesionales
            'AskMarketing',             # Preguntas reales de marketing
            'SocialMediaMarketing',     # SMM real
            'EmailMarketing',           # Email marketing
            'ContentMarketing',         # Content marketing
            'digital_marketing',        # Digital marketing general
            'PPC',                      # Paid advertising
            'SEO',                      # SEO discussions
            'Branding',                 # Branding strategies
            'startups',                 # Startup conversations
            'B2B_Marketing',            # B2B specific
        ]
        
        all_conversations = []
        
        for sub_name in subreddits:
            print(f"🔍 Scraping r/{sub_name}...")
            
            try:
                subreddit = self.reddit.subreddit(sub_name)
                
                # Top posts del último año (conversaciones más valiosas)
                for post in subreddit.top(time_filter='year', limit=limit // len(subreddits)):
                    # Filtrar posts de calidad (score alto = conversación valiosa)
                    if post.score < 10:
                        continue
                    
                    post_data = {
                        'subreddit': sub_name,
                        'title': post.title,
                        'body': post.selftext,
                        'score': post.score,
                        'num_comments': post.num_comments,
                        'created_utc': datetime.fromtimestamp(post.created_utc).isoformat(),
                        'url': f"https://reddit.com{post.permalink}",
                        'comments': []
                    }
                    
                    # Extraer comentarios (conversaciones reales)
                    try:
                        post.comments.replace_more(limit=0)
                        for comment in post.comments.list()[:50]:
                            # Filtrar comentarios cortos o de baja calidad
                            if len(comment.body) > 30 and comment.score > 0:
                                post_data['comments'].append({
                                    'text': comment.body,
                                    'score': comment.score,
                                    'author': str(comment.author) if comment.author else '[deleted]'
                                })
                    except Exception as e:
                        print(f"   ⚠️  Error extrayendo comentarios: {e}")
                    
                    if post_data['comments']:  # Solo guardar si tiene comentarios
                        all_conversations.append(post_data)
                    
                    time.sleep(0.1)  # Rate limiting
                
                print(f"   ✅ {len([c for c in all_conversations if c['subreddit'] == sub_name])} posts de r/{sub_name}")
                
            except Exception as e:
                print(f"   ❌ Error en r/{sub_name}: {e}")
                continue
        
        return all_conversations
    
    def scrape_sales_calls_conversations(self) -> List[Dict]:
        """Extrae conversaciones específicas de ventas (llamadas, objeciones, cierres)"""
        
        sales_subreddits = ['sales', 'SaaSSales', 'B2B_Sales']
        keywords = ['cold call', 'objection', 'closing', 'discovery call', 'demo', 'pitch']
        
        conversations = []
        
        for sub_name in sales_subreddits:
            print(f"🔍 Buscando conversaciones de ventas en r/{sub_name}...")
            
            try:
                subreddit = self.reddit.subreddit(sub_name)
                
                for keyword in keywords:
                    for post in subreddit.search(keyword, time_filter='year', limit=50):
                        if post.score < 5:
                            continue
                        
                        post_data = {
                            'subreddit': sub_name,
                            'keyword': keyword,
                            'title': post.title,
                            'body': post.selftext,
                            'score': post.score,
                            'comments': []
                        }
                        
                        post.comments.replace_more(limit=0)
                        for comment in post.comments.list()[:30]:
                            if len(comment.body) > 40:
                                post_data['comments'].append({
                                    'text': comment.body,
                                    'score': comment.score
                                })
                        
                        if post_data['comments']:
                            conversations.append(post_data)
                        
                        time.sleep(0.1)
                
            except Exception as e:
                print(f"   ❌ Error: {e}")
        
        return conversations
    
    def save_to_file(self, conversations: List[Dict], filename: str = 'reddit_marketing_real.json'):
        """Guarda conversaciones en JSON"""
        output_dir = os.path.join(os.path.dirname(__file__), 'output')
        os.makedirs(output_dir, exist_ok=True)
        
        filepath = os.path.join(output_dir, filename)
        
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(conversations, f, indent=2, ensure_ascii=False)
        
        print(f"\n✅ Guardado: {filepath}")
        print(f"📊 Total conversaciones: {len(conversations)}")
        print(f"📝 Total comentarios: {sum(len(c['comments']) for c in conversations)}")
        
        return filepath
    
    def get_statistics(self, conversations: List[Dict]) -> Dict:
        """Estadísticas del dataset"""
        total_comments = sum(len(c['comments']) for c in conversations)
        avg_score = sum(c['score'] for c in conversations) / len(conversations) if conversations else 0
        
        subreddit_counts = {}
        for conv in conversations:
            sub = conv.get('subreddit', 'unknown')
            subreddit_counts[sub] = subreddit_counts.get(sub, 0) + 1
        
        return {
            'total_conversations': len(conversations),
            'total_comments': total_comments,
            'average_score': avg_score,
            'subreddit_distribution': subreddit_counts,
            'avg_comments_per_post': total_comments / len(conversations) if conversations else 0
        }


def main():
    """Ejecutar scraping completo"""
    print("=" * 70)
    print("  REDDIT REAL CONVERSATIONS SCRAPER")
    print("  Extrayendo conversaciones AUTÉNTICAS de marketing y ventas")
    print("=" * 70)
    print()
    
    scraper = RedditRealScraper()
    
    # 1. Scraping de marketing general
    print("📥 FASE 1: Marketing Conversations")
    marketing_convs = scraper.scrape_marketing_conversations(limit=500)
    
    # 2. Scraping de ventas específico
    print("\n📥 FASE 2: Sales-Specific Conversations")
    sales_convs = scraper.scrape_sales_calls_conversations()
    
    # 3. Combinar y guardar
    all_convs = marketing_convs + sales_convs
    
    # 4. Guardar
    filepath = scraper.save_to_file(all_convs)
    
    # 5. Estadísticas
    stats = scraper.get_statistics(all_convs)
    
    print("\n" + "=" * 70)
    print("📊 ESTADÍSTICAS DEL DATASET")
    print("=" * 70)
    print(f"Total conversaciones: {stats['total_conversations']}")
    print(f"Total comentarios: {stats['total_comments']}")
    print(f"Promedio score: {stats['average_score']:.1f}")
    print(f"Promedio comentarios/post: {stats['avg_comments_per_post']:.1f}")
    print("\nDistribución por subreddit:")
    for sub, count in sorted(stats['subreddit_distribution'].items(), key=lambda x: x[1], reverse=True):
        print(f"  r/{sub}: {count} posts")
    
    print("\n✅ LISTO! Dataset guardado en:")
    print(f"   {filepath}")
    print("\n💡 Next step: python twitter_scraper.py")


if __name__ == '__main__':
    main()
