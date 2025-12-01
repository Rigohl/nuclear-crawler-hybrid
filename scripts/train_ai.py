#!/usr/bin/env python3
"""
Script de Entrenamiento Inicial para AI Smart
Genera datos de entrenamiento y entrena el modelo
"""

import json
import random
from datetime import datetime
from pathlib import Path

def generate_training_data(num_examples=100):
    """Genera datos de entrenamiento sintéticos"""
    
    user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    ]
    
    domains = [
        "example.com",
        "test.com",
        "demo.org",
        "sample.net",
    ]
    
    results = ["success", "ban", "timeout", "403", "429", "success", "success"]
    
    training_data = []
    
    for i in range(num_examples):
        domain = random.choice(domains)
        url = f"https://{domain}/page{i}"
        user_agent = random.choice(user_agents)
        
        # Headers variados
        headers = {
            "Accept": "text/html,application/xhtml+xml",
            "Accept-Language": "en-US,en;q=0.9",
        }
        
        # Resultado basado en probabilidades
        result = random.choice(results)
        
        # Features
        features = {
            "response_time": random.uniform(100, 5000),
            "content_length": random.randint(1000, 100000),
            "links_count": random.randint(0, 100),
        }
        
        data = {
            "url": url,
            "headers": headers,
            "user_agent": user_agent,
            "result": result,
            "response_time_ms": random.randint(100, 5000),
            "status_code": 200 if result == "success" else random.choice([403, 429, 500]),
            "features": features,
        }
        
        training_data.append(data)
    
    return training_data

def save_training_data(data, output_file="training_data.json"):
    """Guarda datos de entrenamiento"""
    with open(output_file, 'w') as f:
        json.dump(data, f, indent=2)
    print(f"✅ Guardados {len(data)} ejemplos en {output_file}")

def main():
    print("🧠 Generando datos de entrenamiento inicial...")
    
    # Generar datos
    training_data = generate_training_data(200)
    
    # Guardar
    output_dir = Path("training")
    output_dir.mkdir(exist_ok=True)
    
    output_file = output_dir / "initial_training.json"
    save_training_data(training_data, str(output_file))
    
    print(f"\n📊 Estadísticas:")
    results_count = {}
    for data in training_data:
        results_count[data["result"]] = results_count.get(data["result"], 0) + 1
    
    for result, count in results_count.items():
        print(f"   {result}: {count}")
    
    print(f"\n✅ Entrenamiento inicial listo!")
    print(f"   Usa estos datos con: ai_smart.initial_training()")

if __name__ == "__main__":
    main()


