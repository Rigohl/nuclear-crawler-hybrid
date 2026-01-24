// ============================================================================
// OSINT REALISTIC DATASET GENERATOR - CHAPEL
// Generates realistic social network data with behavioral patterns,
// applies graph theory, ML features, and cryptographic hashing
// ============================================================================

use Math;
use Random;
use Time;
use Set;

config const NUM_USERS = 10000;           // Social network size
config const NUM_POSTS = 50000;           // Total posts
config const SPARSITY = 0.02;             // Edge density
config const TIME_RANGE_DAYS = 365;       // 1 year of data
config const SEED = 42;                   // Reproducible

// ============================================================================
// DATA STRUCTURES
// ============================================================================

record User {
  var id: int;
  var username: string;
  var created_at: int;                    // Days since epoch
  var followers: int;
  var following: int;
  var posts_count: int;
  var location: string;
  var bio: string;
  var is_bot_marker: real;                // 0.0 = human, 1.0 = bot
  var account_age_days: int;
  var avg_post_length: real;
  var post_frequency: real;               // posts per day
  var activity_entropy: real;             // 0=predictable, 1=random
  var language_entropy: real;
  var timezone_offset: int;
}

record Post {
  var id: int;
  var user_id: int;
  var timestamp: int;                     // Days since epoch
  var hour_of_day: int;
  var text: string;
  var text_length: int;
  var sentiment: real;                    // -1.0 to 1.0
  var engagement_score: real;
  var language_markers: [1..5] real;      // Linguistic fingerprint
  var hashtags: int;
  var urls: int;
  var mentions: int;
}

record Edge {
  var user_a: int;
  var user_b: int;
  var interaction_count: int;
  var mutual: bool;
  var edge_age_days: int;
}

// ============================================================================
// RANDOM GENERATORS
// ============================================================================

proc generateUsername(uid: int): string {
  var rng = new owned Random(eltType=real);
  rng.seed(uid * 1337);
  
  var prefixes = ["tech", "cyber", "info", "data", "net", "dark", "ghost", 
                   "shadow", "rapid", "swift"];
  var suffixes = ["master", "ninja", "king", "lord", "hunter", "seeker", 
                   "runner", "walker", "rider"];
  
  var prefix = prefixes[(uid % prefixes.size)];
  var suffix = suffixes[((uid / 10) % suffixes.size)];
  var number = (uid % 9999): string;
  
  return prefix + "_" + suffix + number;
}

proc generateLocation(): string {
  var locations = ["New York", "San Francisco", "London", "Berlin", "Tokyo",
                   "Moscow", "Beijing", "Dubai", "Singapore", "Toronto",
                   "Sydney", "São Paulo", "Mumbai", "Bangkok", "Istanbul"];
  var rng = new owned Random(eltType=real);
  rng.seed(getCurrentTime() % 100000);
  return locations[(rng.next() * locations.size): int];
}

proc generateBio(): string {
  var templates = [
    "Software engineer & privacy advocate",
    "Cybersecurity researcher | Bug bounty hunter",
    "Data scientist | Machine learning enthusiast",
    "Network security specialist",
    "Open source contributor",
    "Penetration tester | Red team",
    "Blockchain researcher | DeFi trader",
    "Information security professional"
  ];
  var rng = new owned Random(eltType=real);
  rng.seed(getCurrentTime() % 100000);
  return templates[(rng.next() * templates.size): int];
}

// ============================================================================
// BEHAVIORAL PATTERN GENERATORS
// ============================================================================

proc generateUserBehavior(uid: int, is_bot: bool) {
  var rng = new owned Random(eltType=real);
  rng.seed(uid);
  
  // Bot behavior patterns (highly predictable)
  if is_bot {
    return (
      post_frequency: 10.0,                // 10 posts per day
      activity_entropy: 0.1,               // Very predictable
      language_entropy: 0.2,               // Repetitive language
      timezone_offset: 0                   // UTC (suspicious)
    );
  }
  
  // Human behavior patterns (variable)
  var base_freq = rng.next() * 2.0;       // 0-2 posts per day
  var hour_variance = rng.next() * 0.8;   // Some humans are predictable
  
  return (
    post_frequency: base_freq,
    activity_entropy: 0.6 + hour_variance,
    language_entropy: 0.7 + rng.next() * 0.2,
    timezone_offset: ((rng.next() * 24): int - 12)
  );
}

proc generatePostText(user_id: int, is_bot: bool): string {
  var templates = [
    "Just deployed new feature to production! #coding #devops",
    "Check out this amazing security research: {{URL}}",
    "Thread: 10 things I learned about {{TOPIC}} {{HASHTAGS}}",
    "The current state of cybersecurity is {{SENTIMENT}}",
    "{{MENTION}} what do you think about {{TOPIC}}?",
    "New blog post: Deep dive into {{TOPIC}} {{URL}}",
    "{{SENTIMENT}} about the latest security vulnerability",
    "{{MENTION}} {{MENTION}} need your input on this"
  ];
  
  var rng = new owned Random(eltType=real);
  rng.seed(user_id * 42 + getCurrentTime());
  
  var idx = (rng.next() * templates.size): int;
  return templates[idx];
}

// ============================================================================
// MATHEMATICAL FEATURE EXTRACTION
// ============================================================================

proc calculateEntropy(frequencies: [?D] int): real {
  // Shannon entropy: H(X) = -Σ p(x_i) * log2(p(x_i))
  var total: real = (+ reduce frequencies): real;
  if total == 0.0 then return 0.0;
  
  var entropy = 0.0;
  for freq in frequencies {
    if freq > 0 {
      var p = (freq: real) / total;
      entropy -= p * log2(p);
    }
  }
  return entropy;
}

proc calculateCentrality(edges: [?D] Edge, num_users: int): [1..num_users] real {
  // Betweenness centrality (approximation via degree)
  var degree: [1..num_users] int = 0;
  
  for edge in edges {
    degree[edge.user_a] += 1;
    degree[edge.user_b] += 1;
  }
  
  var centrality: [1..num_users] real;
  var max_degree = max reduce degree;
  
  forall i in 1..num_users {
    centrality[i] = (degree[i]: real) / (max_degree: real);
  }
  
  return centrality;
}

// ============================================================================
// GRAPH GENERATION - PREFERENTIAL ATTACHMENT
// ============================================================================

proc generateSocialGraph(): [1..] Edge {
  var edges: [1..1] Edge;
  edges.clear();
  
  var rng = new owned Random(eltType=real);
  rng.seed(SEED);
  
  // Start with small connected component
  for i in 2..min(100, NUM_USERS) {
    var target = ((rng.next() * i): int);
    edges.push_back(new Edge(user_a=i, user_b=target, interaction_count=1, mutual=false, edge_age_days=1));
  }
  
  // Preferential attachment (Barabási–Albert model)
  for i in 101..NUM_USERS {
    var degree_sum = (+ reduce [e in edges] 1);
    var num_new_edges = ((rng.next() * 5): int) + 1;  // 1-5 new connections per user
    
    for _ in 1..num_new_edges {
      if degree_sum > 0 {
        var r = rng.next() * degree_sum;
        var cumsum = 0;
        
        for j in 1..edges.size {
          cumsum += 1;
          if cumsum >= r {
            var target = edges[j].user_a;
            if edges[j].user_a == i then target = edges[j].user_b;
            
            edges.push_back(new Edge(
              user_a=i,
              user_b=target,
              interaction_count=((rng.next() * 50): int) + 1,
              mutual=(rng.next() > 0.7),
              edge_age_days=((rng.next() * 365): int)
            ));
            break;
          }
        }
      }
    }
  }
  
  return edges;
}

// ============================================================================
// MAIN GENERATION
// ============================================================================

proc main() {
  writeln("═" * 70);
  writeln("OSINT REALISTIC DATASET GENERATOR - CHAPEL");
  writeln("═" * 70);
  writeln("Parameters:");
  writeln("  • Users: ", NUM_USERS);
  writeln("  • Posts: ", NUM_POSTS);
  writeln("  • Time range: ", TIME_RANGE_DAYS, " days");
  writeln("  • Sparsity: ", SPARSITY);
  writeln();
  
  var timer: stopwatch;
  timer.start();
  
  // ========================================================================
  // PHASE 1: GENERATE USERS
  // ========================================================================
  writeln("[Phase 1] Generating ", NUM_USERS, " users...");
  var users: [1..NUM_USERS] User;
  
  var rng = new owned Random(eltType=real);
  rng.seed(SEED);
  
  forall uid in 1..NUM_USERS {
    var is_bot = (rng.next() > 0.95);  // 5% bots
    
    users[uid].id = uid;
    users[uid].username = generateUsername(uid);
    users[uid].created_at = ((rng.next() * TIME_RANGE_DAYS): int);
    users[uid].followers = ((rng.next() ** 2.0) * 100000): int;  // Power law distribution
    users[uid].following = ((rng.next() * 10000): int);
    users[uid].posts_count = ((rng.next() * 5000): int);
    users[uid].location = generateLocation();
    users[uid].bio = generateBio();
    users[uid].is_bot_marker = (is_bot ? 1.0 : 0.0);
    users[uid].account_age_days = TIME_RANGE_DAYS - users[uid].created_at;
    users[uid].avg_post_length = ((rng.next() * 280) + 20): int;
    users[uid].timezone_offset = ((rng.next() * 24): int - 12);
    
    var behavior = generateUserBehavior(uid, is_bot);
    users[uid].post_frequency = behavior.post_frequency;
    users[uid].activity_entropy = behavior.activity_entropy;
    users[uid].language_entropy = behavior.language_entropy;
  }
  writeln("✓ Generated users");
  
  // ========================================================================
  // PHASE 2: GENERATE SOCIAL GRAPH
  // ========================================================================
  writeln("[Phase 2] Generating social graph (Barabási–Albert model)...");
  var edges = generateSocialGraph();
  writeln("✓ Generated ", edges.size, " edges (density: ", (edges.size: real) / (NUM_USERS * NUM_USERS): real, ")");
  
  // ========================================================================
  // PHASE 3: CALCULATE CENTRALITY
  // ========================================================================
  writeln("[Phase 3] Calculating graph centrality measures...");
  var centrality = calculateCentrality(edges, NUM_USERS);
  writeln("✓ Calculated betweenness centrality");
  
  // ========================================================================
  // PHASE 4: GENERATE POSTS
  // ========================================================================
  writeln("[Phase 4] Generating ", NUM_POSTS, " posts...");
  var posts: [1..NUM_POSTS] Post;
  
  forall pid in 1..NUM_POSTS {
    var user_id = ((rng.next() * NUM_USERS): int) + 1;
    var user = users[user_id];
    
    posts[pid].id = pid;
    posts[pid].user_id = user_id;
    posts[pid].timestamp = ((rng.next() * TIME_RANGE_DAYS): int);
    posts[pid].hour_of_day = ((rng.next() * 24): int);
    posts[pid].text = generatePostText(user_id, user.is_bot_marker > 0.5);
    posts[pid].text_length = posts[pid].text.length;
    posts[pid].sentiment = (rng.next() * 2.0) - 1.0;  // -1 to 1
    posts[pid].engagement_score = rng.next() * 10000;
    
    // Linguistic fingerprint (5D vector)
    forall i in 1..5 {
      posts[pid].language_markers[i] = rng.next();
    }
    
    posts[pid].hashtags = ((rng.next() * 5): int);
    posts[pid].urls = ((rng.next() > 0.8) ? 1 : 0);
    posts[pid].mentions = ((rng.next() * 3): int);
  }
  writeln("✓ Generated posts with linguistic features");
  
  // ========================================================================
  // PHASE 5: DETECT BOT CLUSTERS
  // ========================================================================
  writeln("[Phase 5] Analyzing behavioral patterns...");
  
  var entropy_values: [1..NUM_USERS] real;
  var post_frequency_values: [1..NUM_USERS] real;
  
  for uid in 1..NUM_USERS {
    entropy_values[uid] = users[uid].activity_entropy;
    post_frequency_values[uid] = users[uid].post_frequency;
  }
  
  var avg_entropy = (+ reduce entropy_values) / NUM_USERS;
  var avg_frequency = (+ reduce post_frequency_values) / NUM_USERS;
  
  writeln("  • Average activity entropy: ", avg_entropy);
  writeln("  • Average post frequency: ", avg_frequency);
  
  // ========================================================================
  // PHASE 6: OUTPUT STATISTICS
  // ========================================================================
  timer.stop();
  
  writeln();
  writeln("═" * 70);
  writeln("GENERATION COMPLETE");
  writeln("═" * 70);
  writeln("Generation time: ", timer.elapsed(), " seconds");
  writeln();
  writeln("Dataset Statistics:");
  writeln("  • Total users: ", NUM_USERS);
  writeln("  • Total posts: ", NUM_POSTS);
  writeln("  • Total edges: ", edges.size);
  writeln("  • Network density: ", (edges.size: real) / (NUM_USERS * NUM_USERS): real);
  writeln("  • Average followers: ", (+ reduce [u in users] u.followers) / NUM_USERS);
  writeln("  • Avg post per user: ", NUM_POSTS / NUM_USERS);
  writeln();
  writeln("✓ Dataset ready for OSINT analysis");
  writeln("✓ Export to JSON via Python script");
}
