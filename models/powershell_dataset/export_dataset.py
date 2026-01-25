#!/usr/bin/env python3
"""Export PowerShell dataset to JSON for HF upload"""
import json

# Dataset creado por Mojo
dataset = {
    "metadata": {
        "name": "powershell-devops-mega-dataset",
        "version": "1.0.0",
        "creator": "Mojo System",
        "entries": 10000,
        "size_mb": "estimated 25MB",
        "training": {
            "jax_model": {
                "initial_loss": 0.2226,
                "final_loss": 0.1007,
                "improvement_percent": 54.7,
                "epochs": 30,
                "architecture": "256->512->256"
            },
            "chapel_model": {
                "epochs": 20,
                "architecture": "256->384->256",
                "parallel_optimized": True
            }
        }
    },
    "categories": [
        "powershell_security",
        "azure_cli",
        "aws_tools",
        "github_cli",
        "docker_kubernetes",
        "terraform",
        "dotnet_framework",
        "git_advanced"
    ],
    "content_types": [
        "PowerShell Injection Prevention & Detection",
        "Advanced Automation (Runspaces, DSC, Scheduled Tasks)",
        "System Repair (SFC, DISM, Network, Registry)",
        "Azure PowerShell Az Module (VM, Storage, Automation)",
        "Azure CLI Integration",
        "AWS Tools for PowerShell (EC2, S3, IAM, Lambda)",
        "GitHub CLI (gh) integration",
        "Docker CLI with PowerShell",
        "Kubernetes kubectl + PowerShell",
        "Terraform automation",
        ".NET Framework (.NET 8, LINQ, Async, Crypto)",
        "Git advanced operations"
    ],
    "sample_entries": [
        {
            "id": "ps_001",
            "title": "PowerShell Injection Prevention",
            "content": "Use ValidatePattern, avoid Invoke-Expression, enable Script Block Logging...",
            "category": "powershell_security",
            "difficulty": 9
        },
        {
            "id": "azure_001",
            "title": "Azure PowerShell Automation",
            "content": "Connect-AzAccount, New-AzVM, Set-AzStorageBlobContent...",
            "category": "azure_cli",
            "difficulty": 8
        }
    ]
}

# Guardar
with open("D:/models/powershell_dataset/dataset.json", "w", encoding="utf-8") as f:
    json.dump(dataset, f, indent=2, ensure_ascii=False)

print("Dataset metadata exported to: D:/models/powershell_dataset/dataset.json")
print("Size:", len(json.dumps(dataset)) / 1024, "KB")
