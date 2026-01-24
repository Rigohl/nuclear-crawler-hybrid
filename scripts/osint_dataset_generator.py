#!/usr/bin/env python3
"""
OSINT REALISTIC DATASET GENERATOR - PYTHON
Generates realistic social network data and applies:
- Graph Theory (community detection, centrality)
- Machine Learning (classification, embeddings)
- NLP (authorship attribution, linguistic analysis)
- Cryptography (hashing, anonymization)
- Statistical Analysis (entropy, anomaly detection)
"""

import json
import hashlib
import numpy as np
from datetime import datetime, timedelta
from collections import defaultdict, Counter
import random
from dataclasses import dataclass, asdict
from typing import List, Dict, Tuple
import math

# ============================================================================
# CONSTANTS
# ============================================================================

NUM_USERS = 10000
NUM_POSTS = 50000
BOT_RATIO = 0.05
TIME_RANGE_DAYS = 365
SEED = 42

# Linguistic markers for authorship fingerprinting
LINGUISTIC_TEMPLATES = {
    "formal": ["The situation regarding", "Furthermore,", "In conclusion,", "It is evident that"],
    "casual": ["lol", "btw", "imo", "ngl", "fr fr"],
    "technical": ["algorithm", "implementation", "optimization", "vulnerability", "exploit"],
    "marketing": ["exclusive", "limited", "amazing", "revolutionary", "must-see"],
    "conspiratorial": ["they", "coverup", "truth", "wake up", "research it"]
}

# ============================================================================
# DATA CLASSES
# ============================================================================

@dataclass
class User:
    id: int
    username: str
    created_at: int  # Days since epoch
    followers: int
    following: int
    posts_count: int
    location: str
    bio: str
    is_bot: bool
    account_age_days: int
    avg_post_length: int
    post_frequency: float
    activity_entropy: float
    language_entropy: float
    timezone_offset: int
    
    # Advanced features
    linguistic_profile: Dict[str, float] = None
    centrality_score: float = 0.0
    community_id: int = -1
    bot_probability: float = 0.0
    embedding: List[float] = None

@dataclass
class Post:
    id: int
    user_id: int
    timestamp: int
    hour_of_day: int
    text: str
    text_length: int
    sentiment: float
    engagement_score: int
    language_markers: List[float]
    hashtags: int
    urls: int
    mentions: int
    
    # Advanced features
    linguistic_fingerprint: Dict[str, float] = None
    is_suspicious: bool = False
    coordinated_with: List[int] = None

@dataclass
class Edge:
    user_a: int
    user_b: int
    interaction_count: int
    mutual: bool
    edge_age_days: int
    
    # Advanced features
    suspicious_pattern: bool = False
    coordination_score: float = 0.0

# ============================================================================
# GENERATORS
# ============================================================================

def hash_id(uid: int) -> str:
    """Hash user ID using SHA-256 (for anonymization)"""
    return hashlib.sha256(str(uid).encode()).hexdigest()[:16]

def generate_username(uid: int) -> str:
    """Generate realistic username based on user ID"""
    random.seed(uid * 1337)
    prefixes = ["tech", "cyber", "info", "data", "net", "dark", "ghost", 
                "shadow", "rapid", "swift"]
    suffixes = ["master", "ninja", "king", "lord", "hunter", "seeker", 
                "runner", "walker", "rider"]
    
    prefix = prefixes[uid % len(prefixes)]
    suffix = suffixes[(uid // 10) % len(suffixes)]
    number = uid % 9999
    
    return f"{prefix}_{suffix}{number}"

def generate_location() -> str:
    """Generate random location"""
    locations = ["New York", "San Francisco", "London", "Berlin", "Tokyo",
                 "Moscow", "Beijing", "Dubai", "Singapore", "Toronto",
                 "Sydney", "São Paulo", "Mumbai", "Bangkok", "Istanbul"]
    return random.choice(locations)

def generate_bio() -> str:
    """Generate user bio"""
    templates = [
        "Software engineer & privacy advocate",
        "Cybersecurity researcher | Bug bounty hunter",
        "Data scientist | Machine learning enthusiast",
        "Network security specialist",
        "Open source contributor",
        "Penetration tester | Red team",
        "Blockchain researcher | DeFi trader",
        "Information security professional"
    ]
    return random.choice(templates)

def generate_post_text(user_id: int, is_bot: bool, template_type: str = None) -> str:
    """Generate realistic post text"""
    random.seed(user_id * 42)
    
    templates = [
        "Just deployed new feature to production! #coding #devops",
        "Check out this amazing security research: https://example.com/research",
        "Thread: 10 things I learned about {topic} #{topic}",
        "The current state of cybersecurity is {sentiment}",
        "Question for @everyone: what do you think about {topic}?",
        "New blog post: Deep dive into {topic} - https://example.com",
        "Feeling {sentiment} about the latest security vulnerability",
        "@someone @someone_else need your input on this critical issue"
    ]
    
    template = random.choice(templates)
    topics = ["cybersecurity", "privacy", "encryption", "vulnerabilities", "machine learning"]
    sentiments = ["amazing", "concerning", "interesting", "disappointing", "revolutionary"]
    
    post = template.replace("{topic}", random.choice(topics))
    post = post.replace("{sentiment}", random.choice(sentiments))
    
    return post

# ============================================================================
# MATHEMATICAL FEATURES
# ============================================================================

def calculate_entropy(values: List[float]) -> float:
    """Calculate Shannon entropy"""
    if not values or len(values) == 0:
        return 0.0
    
    # Normalize to probabilities
    values = np.array(values)
    values = np.maximum(values, 1e-10)  # Avoid log(0)
    p = values / np.sum(values)
    
    # H = -Σ p_i * log2(p_i)
    entropy = -np.sum(p * np.log2(p + 1e-10))
    return entropy

def calculate_linguistic_fingerprint(text: str) -> Dict[str, float]:
    """
    Extract linguistic fingerprint from text
    Returns: probability distribution over linguistic markers
    """
    fingerprint = {}
    text_lower = text.lower()
    
    for category, markers in LINGUISTIC_TEMPLATES.items():
        count = sum(1 for marker in markers if marker in text_lower)
        fingerprint[category] = count / (len(text) + 1)
    
    return fingerprint

def cosine_similarity(v1: List[float], v2: List[float]) -> float:
    """Calculate cosine similarity between two vectors"""
    v1 = np.array(v1)
    v2 = np.array(v2)
    
    norm1 = np.linalg.norm(v1)
    norm2 = np.linalg.norm(v2)
    
    if norm1 == 0 or norm2 == 0:
        return 0.0
    
    return np.dot(v1, v2) / (norm1 * norm2)

def euclidean_distance(v1: List[float], v2: List[float]) -> float:
    """Calculate Euclidean distance"""
    return np.linalg.norm(np.array(v1) - np.array(v2))

# ============================================================================
# GRAPH ANALYSIS - GRAPH THEORY
# ============================================================================

def build_adjacency_list(edges: List[Edge]) -> Dict[int, List[int]]:
    """Build adjacency list from edges"""
    adj = defaultdict(list)
    for edge in edges:
        adj[edge.user_a].append(edge.user_b)
        if edge.mutual:
            adj[edge.user_b].append(edge.user_a)
    return adj

def calculate_betweenness_centrality(edges: List[Edge], num_users: int) -> Dict[int, float]:
    """
    Calculate betweenness centrality (simplified via degree)
    In full implementation, use Floyd-Warshall
    """
    degree = defaultdict(int)
    
    for edge in edges:
        degree[edge.user_a] += 1
        degree[edge.user_b] += 1
    
    max_degree = max(degree.values()) if degree else 1
    
    centrality = {}
    for uid in range(1, num_users + 1):
        centrality[uid] = degree[uid] / max_degree
    
    return centrality

def louvain_community_detection(edges: List[Edge], num_users: int, iterations: int = 10) -> Dict[int, int]:
    """
    Simplified Louvain algorithm for community detection
    Returns: mapping of user_id -> community_id
    """
    # Initialize: each user is own community
    community = {uid: uid for uid in range(1, num_users + 1)}
    
    # Build edge map
    edge_map = defaultdict(lambda: defaultdict(int))
    for edge in edges:
        edge_map[edge.user_a][edge.user_b] += edge.interaction_count
        if edge.mutual:
            edge_map[edge.user_b][edge.user_a] += edge.interaction_count
    
    # Iterative refinement
    for iteration in range(iterations):
        improvement = False
        
        for uid in range(1, min(num_users + 1, 1000)):  # Sample for speed
            old_community = community[uid]
            best_community = old_community
            best_modularity = 0
            
            # Try moving to neighbor communities
            neighbor_communities = set()
            for neighbor in edge_map[uid].keys():
                neighbor_communities.add(community[neighbor])
            
            for new_community in neighbor_communities:
                # Simplified modularity calculation
                internal_edges = 0
                external_edges = 0
                
                for neighbor, weight in edge_map[uid].items():
                    if community[neighbor] == new_community:
                        internal_edges += weight
                    else:
                        external_edges += weight
                
                modularity = internal_edges - external_edges
                if modularity > best_modularity:
                    best_modularity = modularity
                    best_community = new_community
                    improvement = True
            
            community[uid] = best_community
        
        if not improvement:
            break
    
    return community

def detect_coordinated_behavior(users: List[User], posts: List[Post]) -> Dict[int, float]:
    """
    Detect coordinated behavior / bot networks
    Returns: user_id -> coordination_score (0-1)
    """
    coordination_scores = {}
    
    # Group posts by hour
    posts_by_hour = defaultdict(list)
    for post in posts:
        hour_key = post.timestamp * 24 + post.hour_of_day
        posts_by_hour[hour_key].append(post)
    
    # Calculate synchronization patterns
    for uid in range(1, len(users) + 1):
        user_posts = [p for p in posts if p.user_id == uid]
        
        if not user_posts:
            coordination_scores[uid] = 0.0
            continue
        
        # Entropy of posting times (low = coordinated)
        hours = Counter([p.hour_of_day for p in user_posts])
        entropy = calculate_entropy(list(hours.values()))
        
        # Normalize entropy (0 = perfectly coordinated, 1 = random)
        max_entropy = math.log2(24)  # 24 hours
        normalized_entropy = entropy / max_entropy if max_entropy > 0 else 0
        
        coordination_scores[uid] = 1.0 - normalized_entropy
    
    return coordination_scores

# ============================================================================
# MACHINE LEARNING - CLASSIFICATION
# ============================================================================

def extract_user_features(user: User, posts_list: List[Post], edges: List[Edge]) -> np.ndarray:
    """
    Extract feature vector for user classification
    Returns: 20D feature vector
    """
    user_posts = [p for p in posts_list if p.user_id == user.id]
    user_edges = [e for e in edges if e.user_a == user.id or e.user_b == user.id]
    
    features = [
        user.followers,                               # 0: followers
        user.following,                               # 1: following
        user.post_frequency,                          # 2: posts/day
        user.activity_entropy,                        # 3: activity entropy
        user.language_entropy,                        # 4: language entropy
        user.account_age_days,                        # 5: account age
        len(user_posts),                              # 6: posts count
        np.mean([p.engagement_score for p in user_posts]) if user_posts else 0,  # 7: avg engagement
        np.mean([p.text_length for p in user_posts]) if user_posts else 0,       # 8: avg text length
        len(user_edges),                              # 9: degree (connections)
        user.timezone_offset,                         # 10: timezone
        np.std([p.sentiment for p in user_posts]) if user_posts else 0,          # 11: sentiment variance
        np.mean([p.mentions for p in user_posts]) if user_posts else 0,          # 12: avg mentions
        np.mean([p.hashtags for p in user_posts]) if user_posts else 0,          # 13: avg hashtags
        np.mean([p.urls for p in user_posts]) if user_posts else 0,              # 14: avg URLs
        user.followers / (user.following + 1),                                   # 15: follower ratio
        user.posts_count / (user.account_age_days + 1),                          # 16: post rate
        1.0 if len([p for p in user_posts if p.text_length > 200]) / (len(user_posts) + 1) > 0.5 else 0.0,  # 17: long posts
        len(user_edges) / (user.followers + 1),                                  # 18: network density
        (1.0 if user.is_bot else 0.0)                                            # 19: ground truth
    ]
    
    return np.array(features)

def train_bot_classifier(users: List[User], posts: List[Post], edges: List[Edge]):
    """
    Simple linear classifier for bot detection
    Returns: weights, bias
    """
    # Extract features for all users
    X = []
    y = []
    
    for user in users[:min(1000, len(users))]:  # Sample for speed
        features = extract_user_features(user, posts, edges)
        X.append(features)
        y.append(1.0 if user.is_bot else 0.0)
    
    X = np.array(X)
    y = np.array(y)
    
    # Simple logistic regression via gradient descent
    # w = (X^T X)^-1 X^T y
    X_with_bias = np.column_stack([X, np.ones(len(X))])
    
    try:
        weights = np.linalg.lstsq(X_with_bias, y, rcond=None)[0]
        return weights[:-1], weights[-1]  # Separate weights and bias
    except:
        return np.zeros(X.shape[1]), 0.5

def predict_bot_probability(user: User, posts: List[Post], edges: List[Edge], 
                           weights: np.ndarray, bias: float) -> float:
    """Predict probability that user is bot (0-1)"""
    features = extract_user_features(user, posts, edges)
    
    # Sigmoid function
    z = np.dot(features[:len(weights)], weights) + bias
    probability = 1.0 / (1.0 + np.exp(-z))
    
    return float(probability)

# ============================================================================
# NLP - AUTHORSHIP ATTRIBUTION
# ============================================================================

def build_linguistic_profiles(users: List[User], posts: List[Post]) -> Dict[int, Dict]:
    """
    Build linguistic profile for each user
    Returns: user_id -> linguistic_profile
    """
    profiles = {}
    
    for user in users:
        user_posts = [p for p in posts if p.user_id == user.id]
        
        if not user_posts:
            profiles[user.id] = {cat: 0.0 for cat in LINGUISTIC_TEMPLATES.keys()}
            continue
        
        # Average linguistic markers across all user's posts
        avg_profile = defaultdict(float)
        
        for post in user_posts:
            fingerprint = calculate_linguistic_fingerprint(post.text)
            for cat, score in fingerprint.items():
                avg_profile[cat] += score
        
        for cat in avg_profile:
            avg_profile[cat] /= len(user_posts)
        
        profiles[user.id] = dict(avg_profile)
    
    return profiles

def find_alternate_accounts(users: List[User], posts: List[Post], 
                           linguistic_profiles: Dict[int, Dict]) -> Dict[int, List[int]]:
    """
    Find likely alternate accounts for each user
    Uses cosine similarity on linguistic profile
    Returns: user_id -> [similar_user_ids]
    """
    alts = {}
    
    for uid1 in range(1, len(users) + 1):
        if uid1 not in linguistic_profiles:
            alts[uid1] = []
            continue
        
        profile1 = list(linguistic_profiles[uid1].values())
        similarities = []
        
        for uid2 in range(uid1 + 1, len(users) + 1):
            if uid2 not in linguistic_profiles:
                continue
            
            profile2 = list(linguistic_profiles[uid2].values())
            sim = cosine_similarity(profile1, profile2)
            
            # High similarity + suspicious pattern = likely alt
            if sim > 0.7:
                similarities.append((uid2, sim))
        
        alts[uid1] = [uid for uid, _ in sorted(similarities, key=lambda x: -x[1])[:5]]
    
    return alts

# ============================================================================
# DATA GENERATION
# ============================================================================

def generate_users(num_users: int) -> List[User]:
    """Generate realistic users"""
    print(f"[Phase 1] Generating {num_users} users...")
    
    random.seed(SEED)
    np.random.seed(SEED)
    
    users = []
    
    for uid in range(1, num_users + 1):
        is_bot = random.random() < BOT_RATIO
        
        # Power-law distribution for followers
        followers = int(random.random() ** 2 * 100000)
        
        user = User(
            id=uid,
            username=generate_username(uid),
            created_at=random.randint(0, TIME_RANGE_DAYS),
            followers=followers,
            following=random.randint(0, 10000),
            posts_count=random.randint(0, 5000),
            location=generate_location(),
            bio=generate_bio(),
            is_bot=is_bot,
            account_age_days=TIME_RANGE_DAYS - random.randint(0, TIME_RANGE_DAYS),
            avg_post_length=random.randint(20, 280),
            post_frequency=(10.0 if is_bot else random.random() * 2.0),
            activity_entropy=(0.1 if is_bot else 0.6 + random.random() * 0.4),
            language_entropy=(0.2 if is_bot else 0.7 + random.random() * 0.3),
            timezone_offset=random.randint(-12, 12)
        )
        
        users.append(user)
    
    print(f"✓ Generated {len(users)} users ({sum(1 for u in users if u.is_bot)} bots)")
    return users

def generate_edges(num_users: int, sparsity: float = 0.02) -> List[Edge]:
    """Generate social graph edges"""
    print(f"[Phase 2] Generating social graph...")
    
    random.seed(SEED + 1)
    
    edges = []
    
    # Preferential attachment (Barabási–Albert)
    degree = defaultdict(int)
    
    # Start with connected component
    for i in range(2, min(100, num_users)):
        target = random.randint(1, i - 1)
        edges.append(Edge(
            user_a=i,
            user_b=target,
            interaction_count=random.randint(1, 50),
            mutual=random.random() > 0.7,
            edge_age_days=random.randint(0, TIME_RANGE_DAYS)
        ))
        degree[i] += 1
        degree[target] += 1
    
    # Add edges based on preferential attachment
    for i in range(101, num_users):
        total_degree = sum(degree.values()) + 1
        
        for _ in range(random.randint(1, 5)):
            r = random.random() * total_degree
            cumsum = 0
            
            target = random.randint(1, i - 1)
            for j, deg in degree.items():
                cumsum += deg
                if cumsum >= r:
                    target = j
                    break
            
            edges.append(Edge(
                user_a=i,
                user_b=target,
                interaction_count=random.randint(1, 50),
                mutual=random.random() > 0.7,
                edge_age_days=random.randint(0, TIME_RANGE_DAYS)
            ))
            degree[i] += 1
            degree[target] += 1
    
    print(f"✓ Generated {len(edges)} edges")
    return edges

def generate_posts(users: List[User], num_posts: int) -> List[Post]:
    """Generate realistic posts"""
    print(f"[Phase 3] Generating {num_posts} posts...")
    
    random.seed(SEED + 2)
    
    posts = []
    
    for pid in range(1, num_posts + 1):
        user_id = random.randint(1, len(users))
        user = users[user_id - 1]
        
        post = Post(
            id=pid,
            user_id=user_id,
            timestamp=random.randint(0, TIME_RANGE_DAYS),
            hour_of_day=random.randint(0, 23),
            text=generate_post_text(user_id, user.is_bot),
            text_length=random.randint(20, 280),
            sentiment=random.random() * 2.0 - 1.0,  # -1 to 1
            engagement_score=random.randint(0, 10000),
            language_markers=[random.random() for _ in range(5)],
            hashtags=random.randint(0, 5),
            urls=random.randint(0, 2),
            mentions=random.randint(0, 3)
        )
        
        posts.append(post)
    
    print(f"✓ Generated {len(posts)} posts")
    return posts

# ============================================================================
# ANALYSIS
# ============================================================================

def analyze_dataset(users: List[User], posts: List[Post], edges: List[Edge]):
    """Comprehensive dataset analysis"""
    print(f"\n{'═' * 70}")
    print("OSINT DATASET ANALYSIS")
    print(f"{'═' * 70}\n")
    
    # Phase 1: Graph analysis
    print("[Phase 4] Analyzing graph topology...")
    centrality = calculate_betweenness_centrality(edges, len(users))
    communities = louvain_community_detection(edges, len(users))
    
    for uid, cent in list(centrality.items())[:5]:
        users[uid - 1].centrality_score = cent
        users[uid - 1].community_id = communities.get(uid, -1)
    
    print(f"✓ Detected {len(set(communities.values()))} communities")
    print(f"✓ Calculated centrality for {len(users)} users")
    
    # Phase 2: Bot detection
    print("\n[Phase 5] Training bot classifier...")
    weights, bias = train_bot_classifier(users, posts, edges)
    
    bot_predictions = []
    for user in users[:min(1000, len(users))]:
        prob = predict_bot_probability(user, posts, edges, weights, bias)
        user.bot_probability = prob
        bot_predictions.append(prob)
    
    detected_bots = sum(1 for p in bot_predictions if p > 0.5)
    print(f"✓ Detected {detected_bots} likely bots (threshold: 0.5)")
    
    # Phase 3: Linguistic analysis
    print("\n[Phase 6] Building linguistic profiles...")
    linguistic_profiles = build_linguistic_profiles(users, posts)
    
    for uid, profile in linguistic_profiles.items():
        users[uid - 1].linguistic_profile = profile
    
    alts = find_alternate_accounts(users, posts, linguistic_profiles)
    print(f"✓ Found potential alternate accounts")
    
    # Phase 4: Coordination detection
    print("\n[Phase 7] Detecting coordinated behavior...")
    coordination_scores = detect_coordinated_behavior(users, posts)
    
    for uid, score in coordination_scores.items():
        if uid <= len(users):
            users[uid - 1].bot_probability = max(users[uid - 1].bot_probability, score * 0.3)
    
    highly_coordinated = sum(1 for s in coordination_scores.values() if s > 0.7)
    print(f"✓ Found {highly_coordinated} highly coordinated users")
    
    # Statistics
    print(f"\n{'─' * 70}")
    print("Dataset Statistics:")
    print(f"  • Total users: {len(users)}")
    print(f"  • Total posts: {len(posts)}")
    print(f"  • Total edges: {len(edges)}")
    print(f"  • Network density: {len(edges) / (len(users) * (len(users)-1) / 2):.6f}")
    print(f"  • Communities: {len(set(communities.values()))}")
    print(f"  • Average posts/user: {len(posts) / len(users):.2f}")
    print(f"  • Avg followers: {np.mean([u.followers for u in users]):.0f}")
    print(f"  • Likely bots: {sum(1 for u in users if u.is_bot)}/{len(users)}")
    print(f"{'─' * 70}\n")

# ============================================================================
# EXPORT
# ============================================================================

def export_to_json(users: List[User], posts: List[Post], edges: List[Edge], filename: str):
    """Export dataset to JSON"""
    print(f"[Phase 8] Exporting to {filename}...")
    
    # Convert to serializable format
    users_dict = []
    for u in users:
        user_data = asdict(u)
        user_data['hash_id'] = hash_id(u.id)
        users_dict.append(user_data)
    
    posts_dict = [asdict(p) for p in posts]
    edges_dict = [asdict(e) for e in edges]
    
    dataset = {
        "metadata": {
            "created_at": datetime.now().isoformat(),
            "num_users": len(users),
            "num_posts": len(posts),
            "num_edges": len(edges),
            "time_range_days": TIME_RANGE_DAYS,
            "bot_ratio": BOT_RATIO
        },
        "users": users_dict,
        "posts": posts_dict,
        "edges": edges_dict
    }
    
    with open(filename, 'w') as f:
        json.dump(dataset, f, indent=2, default=str)
    
    print(f"✓ Exported to {filename} ({len(users)}) users, {len(posts)} posts, {len(edges)} edges)")

# ============================================================================
# MAIN
# ============================================================================

def main():
    print(f"{'═' * 70}")
    print("OSINT REALISTIC DATASET GENERATOR - PYTHON")
    print(f"{'═' * 70}\n")
    
    # Generate data
    users = generate_users(NUM_USERS)
    edges = generate_edges(NUM_USERS)
    posts = generate_posts(users, NUM_POSTS)
    
    # Analyze
    analyze_dataset(users, posts, edges)
    
    # Export
    export_to_json(users, posts, edges, "OSINT_REALISTIC_DATASET.json")
    
    print("✅ Dataset generation complete!")
    print("   Ready for OSINT analysis with all advanced techniques")

if __name__ == "__main__":
    main()
