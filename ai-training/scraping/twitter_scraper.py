#!/usr/bin/env python3
"""
Twitter/X Real Marketers Scraper
Extrae tweets AUTÉNTICOS de marketers exitosos
"""

import tweepy
import json
import os
from datetime import datetime
from typing import List, Dict
import time

class TwitterMarketersScra:
    """Scraper de tweets reales de marketers"""
    
    # Marketers TOP (conversaciones auténticas, no corporativas)
    MARKETERS = [
        'garyvee',          # Gary Vaynerchuk - muy auténtico
        'neilpatel',        # Neil Patel - SEO/marketing
        'randfish',         # Rand Fishkin - SEO transparente
        'SethGodin',        # Seth Godin - marketing filosófico
        'annhandley',       # Ann Handley - content marketing
        'briansolis',       # Brian Solis - digital transformation
        'jaybaer',          # Jay Baer - customer experience
        'joepulizzi',       # Joe Pulizzi - content marketing
        'MrPaulSalinger',   # Paul Salinger - SaaS growth
        'heathercone',      # Heather Cone - growth hacking
        'nicolascole77',    # Nicolas Cole - copywriting
        'AlexLlull',        # Alex Llull - marketing directo
        'arvidkahl',        # Arvid Kahl - bootstrapping
        'mijustin',         # Justin Jackson - SaaS marketing
        'robwalling',       # Rob Walling - startups
        'hnshah',           # Hiten Shah - SaaS growth
        'steli',            # Steli Efti - sales startup
        'paulg',            # Paul Graham - startup advice
        'patio11',          # Patrick McKenzie - SaaS wisdom
        'agazdecki',        # Andrew Gazdecki - acquisitions
    ]
    
    def __init__(self, bearer_token: str = None):
        self.bearer_token = bearer_token or os.getenv('TWITTER_BEARER_TOKEN')
        
        if not self.bearer_token:
            print("⚠️  SETUP REQUERIDO:")
            print("   1. Ve a: https://developer.twitter.com/en/portal/dashboard")
            print("   2. Create Free tier account")
            print("   3. Generate Bearer Token")
            print("   4. Guarda en .env: TWITTER_BEARER_TOKEN=...")
            return
        
        self.client = tweepy.Client(bearer_token=self.bearer_token)
    
    def scrape_marketer_tweets(self, username: str, max_results: int = 100) -> List[Dict]:
        """Extrae tweets de un marketer específico"""
        
        try:
            user = self.client.get_user(username=username)
            
            if not user.data:
                print(f"   ❌ Usuario @{username} no encontrado")
                return []
            
            user_id = user.data.id
            
            tweets = self.client.get_users_tweets(
                id=user_id,
                max_results=max_results,
                tweet_fields=['created_at', 'public_metrics', 'conversation_id'],
                exclude=['retweets']  # Solo tweets originales
            )
            
            if not tweets.data:
                return []
            
            tweets_data = []
            
            for tweet in tweets.data:
                metrics = tweet.public_metrics
                
                tweets_data.append({
                    'user': username,
                    'text': tweet.text,
                    'created_at': tweet.created_at.isoformat() if tweet.created_at else None,
                    'retweets': metrics.get('retweet_count', 0),
                    'likes': metrics.get('like_count', 0),
                    'replies': metrics.get('reply_count', 0),
                    'impressions': metrics.get('impression_count', 0),
                    'engagement_score': metrics.get('like_count', 0) + metrics.get('reply_count', 0) * 2
                })
            
            print(f"   ✅ @{username}: {len(tweets_data)} tweets")
            return tweets_data
            
        except Exception as e:
            print(f"   ❌ Error con @{username}: {e}")
            return []
    
    def scrape_all_marketers(self, tweets_per_marketer: int = 100) -> List[Dict]:
        """Extrae tweets de todos los marketers"""
        
        all_tweets = []
        
        print(f"🔍 Scraping {len(self.MARKETERS)} marketers...")
        print()
        
        for username in self.MARKETERS:
            tweets = self.scrape_marketer_tweets(username, tweets_per_marketer)
            all_tweets.extend(tweets)
            time.sleep(1)  # Rate limiting
        
        return all_tweets
    
    def search_marketing_topics(self, query: str, max_results: int = 100) -> List[Dict]:
        """Busca tweets por tema de marketing"""
        
        try:
            tweets = self.client.search_recent_tweets(
                query=query,
                max_results=max_results,
                tweet_fields=['created_at', 'public_metrics', 'author_id']
            )
            
            if not tweets.data:
                return []
            
            tweets_data = []
            
            for tweet in tweets.data:
                metrics = tweet.public_metrics
                
                tweets_data.append({
                    'query': query,
                    'text': tweet.text,
                    'created_at': tweet.created_at.isoformat() if tweet.created_at else None,
                    'retweets': metrics.get('retweet_count', 0),
                    'likes': metrics.get('like_count', 0),
                    'replies': metrics.get('reply_count', 0)
                })
            
            print(f"   ✅ Query '{query}': {len(tweets_data)} tweets")
            return tweets_data
            
        except Exception as e:
            print(f"   ❌ Error con query '{query}': {e}")
            return []
    
    def scrape_marketing_threads(self) -> List[Dict]:
        """Extrae threads populares de marketing"""
        
        queries = [
            '"marketing strategy" -filter:retweets',
            '"sales tips" -filter:retweets',
            '"copywriting" min_faves:100',
            '"email marketing" min_faves:50',
            '"content marketing" -filter:retweets',
            '"growth hacking" min_faves:50',
            '"customer acquisition" -filter:retweets',
            '"conversion optimization" min_faves:25'
        ]
        
        all_threads = []
        
        print("🔍 Buscando threads de marketing...")
        
        for query in queries:
            threads = self.search_marketing_topics(query, max_results=100)
            all_threads.extend(threads)
            time.sleep(2)  # Rate limiting
        
        return all_threads
    
    def save_to_file(self, tweets: List[Dict], filename: str = 'twitter_marketers_real.json'):
        """Guarda tweets en JSON"""
        output_dir = os.path.join(os.path.dirname(__file__), 'output')
        os.makedirs(output_dir, exist_ok=True)
        
        filepath = os.path.join(output_dir, filename)
        
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(tweets, f, indent=2, ensure_ascii=False)
        
        print(f"\n✅ Guardado: {filepath}")
        print(f"📊 Total tweets: {len(tweets)}")
        
        return filepath
    
    def get_statistics(self, tweets: List[Dict]) -> Dict:
        """Estadísticas del dataset"""
        if not tweets:
            return {}
        
        total_engagement = sum(t.get('engagement_score', 0) for t in tweets)
        total_likes = sum(t.get('likes', 0) for t in tweets)
        total_retweets = sum(t.get('retweets', 0) for t in tweets)
        
        user_counts = {}
        for tweet in tweets:
            user = tweet.get('user', 'unknown')
            user_counts[user] = user_counts.get(user, 0) + 1
        
        return {
            'total_tweets': len(tweets),
            'total_likes': total_likes,
            'total_retweets': total_retweets,
            'total_engagement': total_engagement,
            'avg_likes': total_likes / len(tweets),
            'avg_retweets': total_retweets / len(tweets),
            'top_marketers': sorted(user_counts.items(), key=lambda x: x[1], reverse=True)[:10]
        }


def main():
    """Ejecutar scraping completo"""
    print("=" * 70)
    print("  TWITTER REAL MARKETERS SCRAPER")
    print("  Extrayendo tweets AUTÉNTICOS de marketers exitosos")
    print("=" * 70)
    print()
    
    scraper = TwitterMarketersScra()
    
    # 1. Scraping de marketers específicos
    print("📥 FASE 1: Top Marketers")
    marketer_tweets = scraper.scrape_all_marketers(tweets_per_marketer=50)
    
    # 2. Scraping de threads populares
    print("\n📥 FASE 2: Marketing Threads")
    thread_tweets = scraper.scrape_marketing_threads()
    
    # 3. Combinar
    all_tweets = marketer_tweets + thread_tweets
    
    # 4. Filtrar por engagement (calidad)
    quality_tweets = [t for t in all_tweets if t.get('likes', 0) >= 5]
    
    print(f"\n🔍 Filtrado por calidad:")
    print(f"   Total: {len(all_tweets)} → Calidad: {len(quality_tweets)}")
    
    # 5. Guardar
    filepath = scraper.save_to_file(quality_tweets)
    
    # 6. Estadísticas
    stats = scraper.get_statistics(quality_tweets)
    
    print("\n" + "=" * 70)
    print("📊 ESTADÍSTICAS DEL DATASET")
    print("=" * 70)
    print(f"Total tweets: {stats['total_tweets']}")
    print(f"Total likes: {stats['total_likes']:,}")
    print(f"Total retweets: {stats['total_retweets']:,}")
    print(f"Promedio likes/tweet: {stats['avg_likes']:.1f}")
    print(f"Promedio retweets/tweet: {stats['avg_retweets']:.1f}")
    print("\nTop 10 marketers:")
    for user, count in stats['top_marketers']:
        print(f"  @{user}: {count} tweets")
    
    print("\n✅ LISTO! Dataset guardado en:")
    print(f"   {filepath}")
    print("\n💡 Next step: python linkedin_scraper.py")


if __name__ == '__main__':
    main()
