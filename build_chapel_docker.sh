#!/bin/bash
# Script: Build Chapel Docker image and push to AWS ECR
# Free-tier: 500 MB storage per month (gratuito 12 meses)

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Chapel Docker Builder for AWS ECR ===${NC}"

# Configuration
AWS_REGION="${1:-us-east-1}"
AWS_ACCOUNT_ID="${2:-YOUR_ACCOUNT_ID}"
ECR_REPO_NAME="chapel-compiler"
CHAPEL_VERSION="2.4.0"
IMAGE_NAME="chapel"
IMAGE_TAG="2.4.0-ubuntu22"

# 1. Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${YELLOW}⚠️  Docker no está instalado${NC}"
    echo "Instala Docker desde: https://docs.docker.com/get-docker/"
    exit 1
fi

echo -e "${GREEN}✓ Docker disponible${NC}"

# 2. Build local Docker image
echo -e "${BLUE}📦 Construyendo imagen local...${NC}"
docker build -f Dockerfile.chapel -t ${IMAGE_NAME}:${IMAGE_TAG} -t ${IMAGE_NAME}:latest .
echo -e "${GREEN}✓ Imagen construida: ${IMAGE_NAME}:${IMAGE_TAG}${NC}"

# 3. Test the image locally
echo -e "${BLUE}🧪 Testeando imagen...${NC}"
docker run --rm ${IMAGE_NAME}:${IMAGE_TAG} --version
echo -e "${GREEN}✓ Imagen funciona correctamente${NC}"

# 4. AWS ECR Login (requires AWS credentials configured)
echo -e "${BLUE}🔐 Configurando AWS ECR...${NC}"
if command -v aws &> /dev/null; then
    # Get ECR login token
    aws ecr get-login-password --region ${AWS_REGION} | \
        docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com
    
    # Create ECR repository if it doesn't exist
    aws ecr create-repository \
        --repository-name ${ECR_REPO_NAME} \
        --region ${AWS_REGION} \
        --image-scanning-configuration scanOnPush=true \
        --encryption-configuration encryptionType=AES \
        2>/dev/null || echo "Repository already exists"
    
    echo -e "${GREEN}✓ ECR configurado${NC}"
    
    # 5. Tag and push to ECR
    echo -e "${BLUE}📤 Subiendo a AWS ECR...${NC}"
    ECR_URI="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO_NAME}"
    
    docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${ECR_URI}:${IMAGE_TAG}
    docker tag ${IMAGE_NAME}:latest ${ECR_URI}:latest
    
    docker push ${ECR_URI}:${IMAGE_TAG}
    docker push ${ECR_URI}:latest
    
    echo -e "${GREEN}✓ Imagen subida a ECR${NC}"
    echo -e "${BLUE}📍 URI de imagen:${NC} ${ECR_URI}:${IMAGE_TAG}"
    
else
    echo -e "${YELLOW}⚠️  AWS CLI no instalado - saltando subida a ECR${NC}"
    echo "Instala con: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html"
    echo -e "${BLUE}Para subir manualmente:${NC}"
    echo "  1. aws configure"
    echo "  2. docker tag ${IMAGE_NAME}:${IMAGE_TAG} YOUR_ECR_URI:${IMAGE_TAG}"
    echo "  3. docker push YOUR_ECR_URI:${IMAGE_TAG}"
fi

# 6. Show available images
echo -e "${BLUE}📋 Imágenes Docker disponibles:${NC}"
docker images | grep -E "${IMAGE_NAME}|REPOSITORY"

echo -e "${GREEN}=== ✓ Proceso completado ===${NC}"
echo -e "${BLUE}Uso local:${NC}"
echo "  docker run --rm -v \$(pwd):/workspace ${IMAGE_NAME}:${IMAGE_TAG} chpl myfile.chpl"
echo ""
echo -e "${BLUE}Información de free-tier AWS ECR:${NC}"
echo "  • 500 MB storage/mes (gratis por 12 meses)"
echo "  • Después: \$0.10 por GB/mes"
echo "  • Transfer out: \$0.09 por GB"
