#!/usr/bin/env python3
"""
OSINT ADVANCED ANALYSIS - Integration of All Techniques
Applies: Graph Theory, ML, NLP, Cryptography, Statistics
"""

import json
import numpy as np
from collections import defaultdict, Counter
from typing import Dict, List, Tuple
import hashlib

class OSINTAnalyzer:
    def __init__(self, dataset_path: str):
        with open(dataset_path, 'r') as f:
            self.data = json.load(f)
        
        self.users = {u['id']: u for u in self.data['users']}
        self.posts = {p['id']: p for p in self.data['posts']}
        self.edges = self.data['edges']
        
        self.build_adjacency()
    
    def build_adjacency(self):
        """Build adjacency structure for graph analysis"""
        self.adj = defaultdict(list)
        for edge in self.edges:
            self.adj[edge['user_a']].append(edge['user_b'])
            if edge.get('mutual', False):
                self.adj[edge['user_b']].append(edge['user_a'])
    
    # ========================================================================
    # PHASE 1: GRAPH THEORY ANALYSIS
    # ========================================================================
    
    def identify_key_coordinators(self, top_n: int = 20) -> List[Dict]:
        """
        Identify key coordinators using centrality + bot probability
        Returns: List of suspicious coordinators
        """
        print("\n[ANALYSIS] Phase 1: Identifying Key Coordinators...")
        
        coordinators = []
        
        for uid, user in self.users.items():
            if user.get('bot_probability', 0) > 0.6:
                centrality = user.get('centrality_score', 0)
                degree = len(self.adj[uid])
                
                # Coordination score = high centrality + bot-like behavior
                coordination_score = (
                    user['bot_probability'] * 0.5 +
                    centrality * 0.3 +
                    (degree / max(len(self.adj[u]) for u in self.adj)) * 0.2
                )
                
                coordinators.append({
                    'user_id': uid,
                    'username': user['username'],
                    'bot_probability': user['bot_probability'],
                    'centrality': centrality,
                    'degree': degree,
                    'coordination_score': coordination_score,
                    'followers': user['followers']
                })
        
        coordinators = sorted(coordinators, key=lambda x: -x['coordination_score'])[:top_n]
        
        print(f"✓ Identified {len(coordinators)} key coordinators")
        for i, coord in enumerate(coordinators[:5]):
            print(f"  {i+1}. @{coord['username']} (score: {coord['coordination_score']:.2f})")
        
        return coordinators
    
    def detect_bot_clusters(self) -> List[List[int]]:
        """
        Detect clusters of bots (communities with high bot density)
        Returns: List of bot clusters
        """
        print("\n[ANALYSIS] Phase 2: Detecting Bot Clusters...")
        
        # Group users by community
        communities = defaultdict(list)
        for uid, user in self.users.items():
            comm_id = user.get('community_id', -1)
            if comm_id >= 0:
                communities[comm_id].append(uid)
        
        bot_clusters = []
        
        for comm_id, members in communities.items():
            bot_count = sum(1 for uid in members if self.users[uid].get('bot_probability', 0) > 0.6)
            bot_density = bot_count / len(members) if members else 0
            
            if bot_density > 0.3:  # >30% bots = suspicious cluster
                bot_clusters.append({
                    'community_id': comm_id,
                    'size': len(members),
                    'bot_count': bot_count,
                    'bot_density': bot_density,
                    'members': members
                })
        
        bot_clusters = sorted(bot_clusters, key=lambda x: -x['bot_density'])
        
        print(f"✓ Detected {len(bot_clusters)} suspicious bot clusters")
        for i, cluster in enumerate(bot_clusters[:3]):
            print(f"  {i+1}. Community {cluster['community_id']}: "
                  f"{cluster['bot_count']}/{cluster['size']} bots "
                  f"({cluster['bot_density']:.0%})")
        
        return bot_clusters
    
    # ========================================================================
    # PHASE 2: NLP ANALYSIS - AUTHORSHIP ATTRIBUTION
    # ========================================================================
    
    def find_alternate_accounts(self, target_user_id: int, threshold: float = 0.75) -> List[Dict]:
        """
        Find alternate accounts of target user using linguistic fingerprints
        Returns: List of likely alts
        """
        print(f"\n[ANALYSIS] Phase 3: Finding Alternate Accounts for User {target_user_id}...")
        
        target = self.users.get(target_user_id)
        if not target or not target.get('linguistic_profile'):
            return []
        
        target_profile = np.array(list(target['linguistic_profile'].values()))
        alts = []
        
        for uid, user in self.users.items():
            if uid == target_user_id or not user.get('linguistic_profile'):
                continue
            
            user_profile = np.array(list(user['linguistic_profile'].values()))
            
            # Cosine similarity
            sim = np.dot(target_profile, user_profile) / (
                np.linalg.norm(target_profile) * np.linalg.norm(user_profile) + 1e-10
            )
            
            if sim > threshold:
                alts.append({
                    'user_id': uid,
                    'username': user['username'],
                    'similarity': float(sim),
                    'followers': user['followers'],
                    'posts_count': user['posts_count']
                })
        
        alts = sorted(alts, key=lambda x: -x['similarity'])
        
        if alts:
            print(f"✓ Found {len(alts)} likely alternate accounts")
            for i, alt in enumerate(alts[:3]):
                print(f"  {i+1}. @{alt['username']} (similarity: {alt['similarity']:.2%})")
        else:
            print("✓ No highly similar accounts found")
        
        return alts
    
    def analyze_content_themes(self, user_id: int) -> Dict[str, float]:
        """
        Analyze content themes for a user
        Returns: Theme distribution
        """
        user = self.users.get(user_id)
        if not user or not user.get('linguistic_profile'):
            return {}
        
        return user['linguistic_profile']
    
    # ========================================================================
    # PHASE 3: STATISTICAL ANALYSIS - ANOMALY DETECTION
    # ========================================================================
    
    def detect_temporal_anomalies(self) -> List[Dict]:
        """
        Detect users with anomalous temporal behavior
        Returns: List of anomalous users
        """
        print("\n[ANALYSIS] Phase 4: Detecting Temporal Anomalies...")
        
        # Calculate entropy for each user's posting times
        user_post_times = defaultdict(list)
        
        for post in self.posts.values():
            user_post_times[post['user_id']].append(post['hour_of_day'])
        
        anomalies = []
        
        for uid, hours in user_post_times.items():
            if len(hours) < 3:
                continue
            
            # Calculate entropy
            hour_dist = Counter(hours)
            total = len(hours)
            entropy = -sum((c/total) * np.log2(c/total + 1e-10) for c in hour_dist.values())
            
            # Low entropy = predictable = suspicious
            if entropy < 2.0:
                anomalies.append({
                    'user_id': uid,
                    'username': self.users[uid]['username'],
                    'entropy': float(entropy),
                    'posts': len(hours),
                    'peak_hour': max(hour_dist, key=hour_dist.get)
                })
        
        anomalies = sorted(anomalies, key=lambda x: x['entropy'])
        
        print(f"✓ Detected {len(anomalies)} users with anomalous temporal patterns")
        for i, anom in enumerate(anomalies[:5]):
            print(f"  {i+1}. @{anom['username']} (entropy: {anom['entropy']:.2f}, "
                  f"peak: {anom['peak_hour']}:00)")
        
        return anomalies
    
    def detect_sentiment_anomalies(self) -> List[Dict]:
        """
        Detect users with unusual sentiment patterns
        Returns: List of anomalous sentiment users
        """
        print("\n[ANALYSIS] Phase 5: Detecting Sentiment Anomalies...")
        
        user_sentiments = defaultdict(list)
        
        for post in self.posts.values():
            user_sentiments[post['user_id']].append(post.get('sentiment', 0))
        
        anomalies = []
        
        for uid, sentiments in user_sentiments.items():
            if len(sentiments) < 3:
                continue
            
            avg_sentiment = np.mean(sentiments)
            std_sentiment = np.std(sentiments)
            
            # Very negative or very positive = suspicious
            if abs(avg_sentiment) > 0.7 or std_sentiment < 0.1:
                anomalies.append({
                    'user_id': uid,
                    'username': self.users[uid]['username'],
                    'avg_sentiment': float(avg_sentiment),
                    'std_sentiment': float(std_sentiment),
                    'posts': len(sentiments)
                })
        
        anomalies = sorted(anomalies, key=lambda x: -abs(x['avg_sentiment']))
        
        print(f"✓ Detected {len(anomalies)} users with anomalous sentiment")
        for i, anom in enumerate(anomalies[:5]):
            print(f"  {i+1}. @{anom['username']} (sentiment: {anom['avg_sentiment']:.2f})")
        
        return anomalies
    
    # ========================================================================
    # PHASE 4: INFORMATION THEORY - ENTROPY ANALYSIS
    # ========================================================================
    
    def analyze_network_entropy(self) -> Dict[str, float]:
        """
        Calculate various entropy metrics for the network
        Returns: Dictionary of entropy metrics
        """
        print("\n[ANALYSIS] Phase 6: Network Entropy Analysis...")
        
        degrees = [len(self.adj[uid]) for uid in self.adj]
        degree_dist = Counter(degrees)
        
        # Degree entropy
        total = sum(degree_dist.values())
        degree_entropy = -sum((c/total) * np.log2(c/total + 1e-10) 
                             for c in degree_dist.values())
        
        # Bot distribution entropy
        bot_status = [self.users[uid].get('bot_probability', 0) > 0.5 
                     for uid in self.users]
        bot_ratio = sum(bot_status) / len(bot_status)
        bot_entropy = -(bot_ratio * np.log2(bot_ratio + 1e-10) + 
                       (1-bot_ratio) * np.log2(1-bot_ratio + 1e-10))
        
        metrics = {
            'degree_entropy': float(degree_entropy),
            'bot_entropy': float(bot_entropy),
            'avg_degree': float(np.mean(degrees)),
            'max_degree': float(np.max(degrees)),
            'bot_ratio': float(bot_ratio)
        }
        
        print(f"✓ Network entropy metrics:")
        print(f"  • Degree entropy: {metrics['degree_entropy']:.3f}")
        print(f"  • Bot entropy: {metrics['bot_entropy']:.3f}")
        print(f"  • Average degree: {metrics['avg_degree']:.1f}")
        print(f"  • Bot ratio: {metrics['bot_ratio']:.1%}")
        
        return metrics
    
    # ========================================================================
    # FINAL REPORT
    # ========================================================================
    
    def generate_report(self) -> Dict:
        """Generate comprehensive OSINT analysis report"""
        print("\n" + "="*70)
        print("🔍 OSINT COMPREHENSIVE ANALYSIS REPORT")
        print("="*70)
        
        report = {
            'timestamp': self.data['metadata']['created_at'],
            'dataset_size': {
                'users': len(self.users),
                'posts': len(self.posts),
                'edges': len(self.edges)
            },
            'key_coordinators': self.identify_key_coordinators(),
            'bot_clusters': self.detect_bot_clusters(),
            'temporal_anomalies': self.detect_temporal_anomalies(),
            'sentiment_anomalies': self.detect_sentiment_anomalies(),
            'network_entropy': self.analyze_network_entropy()
        }
        
        print("\n" + "="*70)
        print("✅ ANALYSIS COMPLETE")
        print("="*70)
        print(f"\nFinal Statistics:")
        print(f"  • Key coordinators identified: {len(report['key_coordinators'])}")
        print(f"  • Bot clusters detected: {len(report['bot_clusters'])}")
        print(f"  • Temporal anomalies: {len(report['temporal_anomalies'])}")
        print(f"  • Sentiment anomalies: {len(report['sentiment_anomalies'])}")
        
        return report

def main():
    analyzer = OSINTAnalyzer("OSINT_REALISTIC_DATASET.json")
    report = analyzer.generate_report()
    
    # Export report
    with open("OSINT_ANALYSIS_REPORT.json", 'w') as f:
        json.dump(report, f, indent=2, default=str)
    
    print(f"\n💾 Report saved to: OSINT_ANALYSIS_REPORT.json")

if __name__ == "__main__":
    main()
