"""
MOJO: EXPORT DATASET & UPLOAD TO HF
====================================
1. Exporta dataset creado en Mojo a JSON
2. Trigger HF CLI para upload
"""

from sys import exit
from python import Python

fn export_dataset_to_json():
    """Exportar dataset a JSON usando FFI con Python."""
    try:
        var py = Python()
        var json = py.import_module("json")
        var subprocess = py.import_module("subprocess")
        
        # Dataset info
        var dataset_info = py.dict()
        dataset_info["name"] = "powershell-devops-mega-dataset"
        dataset_info["entries"] = 10000
        dataset_info["categories"] = py.list()
        
        var cats = py.list()
        cats.append("powershell_security")
        cats.append("azure_cli")
        cats.append("aws_tools")
        cats.append("github_cli")
        cats.append("docker_kubernetes")
        cats.append("terraform")
        cats.append("dotnet_framework")
        cats.append("git_advanced")
        
        dataset_info["categories"] = cats
        
        # Content sample
        var content_samples = py.list()
        content_samples.append("PowerShell Injection Prevention with ValidatePattern")
        content_samples.append("Azure PowerShell Az Module automation")
        content_samples.append("AWS Tools for PowerShell EC2/S3 management")
        content_samples.append("GitHub CLI gh integration")
        content_samples.append("Docker & Kubernetes with PowerShell")
        
        dataset_info["samples"] = content_samples
        
        # Training results
        var training = py.dict()
        training["jax_model"] = py.dict()
        training["jax_model"]["initial_loss"] = 0.2226
        training["jax_model"]["final_loss"] = 0.1007
        training["jax_model"]["improvement"] = 54.7
        training["jax_model"]["epochs"] = 30
        
        training["chapel_model"] = py.dict()
        training["chapel_model"]["epochs"] = 20
        training["chapel_model"]["architecture"] = "256->384->256"
        
        dataset_info["training"] = training
        
        # Write JSON
        var file_path = "D:/models/powershell_dataset/dataset_metadata.json"
        with open(file_path, "w") as f:
            json.dump(dataset_info, f, indent=2)
        
        print("Dataset metadata exportado a:", file_path)
        
        # Upload via HF CLI
        print("\nSubiendo a HuggingFace...")
        var result = subprocess.run([
            "huggingface-cli", "upload",
            "Kimberlyindiva/powershell-devops-mega-dataset",
            file_path,
            "--repo-type", "dataset"
        ], capture_output=True, text=True)
        
        if result.returncode == 0:
            print("SUBIDO EXITOSAMENTE!")
            print("URL: https://huggingface.co/datasets/Kimberlyindiva/powershell-devops-mega-dataset")
        else:
            print("Error:", result.stderr)
        
    except:
        print("Error en export/upload. Usar manualmente:")
        print("huggingface-cli upload Kimberlyindiva/powershell-devops-mega-dataset dataset_metadata.json --repo-type dataset")

fn main():
    print("="*80)
    print("MOJO: EXPORT & UPLOAD TO HUGGING FACE")
    print("="*80)
    
    export_dataset_to_json()
    
    print("\n" + "="*80)
    print("PROCESO COMPLETADO")
    print("="*80)
