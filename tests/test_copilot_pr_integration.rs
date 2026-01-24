//! 🤖 Test de Integración: GitHub Pull Request Copilot Coding Agent
//!
//! Este test valida la compatibilidad del repo nuclear-crawler-hybrid
//! con PRs automatizados creados por GitHub Copilot Coding Agent.
//!
//! Validaciones:
//! 1. ✅ Las reglas del repo (AGENTS.md) son compatibles con PRs automatizados
//! 2. ✅ Los workflows CI/CD permiten PRs de Copilot
//! 3. ✅ La validación de "no mocks" se ejecuta en PRs
//! 4. ✅ El test de "exactly 5 tools" se ejecuta en PRs
//! 5. ✅ Los templates de PR son compatibles con Copilot
//! 6. ✅ Los workflows validan correctamente cambios de Copilot

use std::fs;
use std::path::Path;

#[test]
fn test_pr_template_exists() {
    let template_path = ".github/PULL_REQUEST_TEMPLATE.md";
    assert!(
        Path::new(template_path).exists(),
        "PR template debe existir para guiar PRs de Copilot"
    );

    let content = fs::read_to_string(template_path)
        .expect("Debe poder leer el PR template");
    
    // Verificar que contiene secciones importantes para Copilot
    assert!(content.contains("## Description"), "Debe tener sección de descripción");
    assert!(content.contains("## Type of change"), "Debe tener tipos de cambio");
    assert!(content.contains("## Checklist"), "Debe tener checklist");
    
    // Verificar que menciona validaciones MCP
    assert!(
        content.contains("MCP") || content.contains("tools"),
        "Template debe mencionar validaciones MCP/tools"
    );
    
    // Verificar que menciona explícitamente las 5 tools MCP correctas
    let expected_tools = [
        "websearch",
        "premium",
        "file_search",
        "scan",
        "ai_dataset_trainer",
    ];
    for tool in &expected_tools {
        assert!(
            content.contains(tool),
            "Template debe mencionar la tool MCP '{}'",
            tool
        );
    }
}

#[test]
fn test_agents_rules_are_clear() {
    let agents_path = "AGENTS.md";
    assert!(
        Path::new(agents_path).exists(),
        "AGENTS.md debe existir con reglas para agentes"
    );

    let content = fs::read_to_string(agents_path)
        .expect("Debe poder leer AGENTS.md");
    
    // Verificar reglas críticas para Copilot
    assert!(content.contains("No mocks"), "Debe prohibir mocks explícitamente");
    assert!(
        content.contains("5 tools") || content.contains("5 herramientas"),
        "Debe especificar exactamente 5 tools"
    );
    assert!(
        content.contains("test_exactly_5_tools"),
        "Debe mencionar el test de validación de tools"
    );
}

#[test]
fn test_ci_workflows_trigger_on_pr() {
    let workflows = [
        ".github/workflows/ci.yml",
        ".github/workflows/mcp-validation.yml",
        ".github/workflows/security.yml",
    ];

    for workflow in &workflows {
        assert!(
            Path::new(workflow).exists(),
            "Workflow {} debe existir",
            workflow
        );

        let content = fs::read_to_string(workflow)
            .expect(&format!("Debe poder leer {}", workflow));
        
        // Verificar que se ejecuta en pull_request
        assert!(
            content.contains("pull_request:"),
            "Workflow {} debe ejecutarse en pull_request",
            workflow
        );
    }
}

#[test]
fn test_ci_validates_no_mocks() {
    let mcp_validation = ".github/workflows/mcp-validation.yml";
    let content = fs::read_to_string(mcp_validation)
        .expect("Debe poder leer mcp-validation.yml");
    
    // Verificar que hay un paso que busca mocks
    assert!(
        content.contains("mock") || content.contains("Mock"),
        "CI debe verificar ausencia de mocks"
    );
    
    assert!(
        content.contains("REAL") || content.contains("real"),
        "CI debe enfatizar uso de datos REALES"
    );
}

#[test]
fn test_ci_validates_exactly_5_tools() {
    let ci_yml = ".github/workflows/ci.yml";
    let content = fs::read_to_string(ci_yml)
        .expect("Debe poder leer ci.yml");
    
    // Verificar que ejecuta test_exactly_5_tools
    assert!(
        content.contains("test_exactly_5_tools"),
        "CI debe ejecutar test_exactly_5_tools para validar número de tools"
    );
}

#[test]
fn test_copilot_instructions_exist() {
    let instructions_path = ".github/copilot-instructions.md";
    assert!(
        Path::new(instructions_path).exists(),
        "Debe existir copilot-instructions.md para guiar a Copilot"
    );

    let content = fs::read_to_string(instructions_path)
        .expect("Debe poder leer copilot-instructions.md");
    
    // Verificar que contiene información sobre las 5 tools
    assert!(
        content.contains("5 tools") || content.contains("five tools"),
        "Instrucciones deben mencionar las 5 tools"
    );
    
    // Verificar que menciona "no mocks"
    assert!(
        content.contains("mocks") || content.contains("stubs"),
        "Instrucciones deben mencionar política de no mocks"
    );
}

#[test]
fn test_workflow_permissions_allow_pr_creation() {
    // Verificar que los workflows tienen permisos apropiados
    let workflows = [
        ".github/workflows/ci.yml",
        ".github/workflows/mcp-validation.yml",
    ];

    for workflow in &workflows {
        let content = fs::read_to_string(workflow)
            .expect(&format!("Debe poder leer {}", workflow));
        
        // No debe tener permissions que bloqueen PRs
        if content.contains("permissions:") {
            // Si define permissions, debe permitir al menos read
            assert!(
                content.contains("contents: read") || 
                content.contains("pull-requests:"),
                "Workflow {} debe tener permisos apropiados para PRs",
                workflow
            );
        }
    }
}

#[test]
fn test_repo_has_required_validation_tools() {
    // Verificar que el repo tiene las herramientas necesarias para validar PRs de Copilot
    
    // 1. Debe tener tests de protocolo MCP
    assert!(
        Path::new("src/mcp/protocol.rs").exists(),
        "Debe existir protocol.rs con definiciones MCP"
    );
    
    // 2. Debe tener tests de integración real
    assert!(
        Path::new("tests/integration_real_mcp.rs").exists(),
        "Debe existir integration_real_mcp.rs para validación real"
    );
    
    // 3. Debe tener configuración de Rust
    assert!(
        Path::new("Cargo.toml").exists(),
        "Debe existir Cargo.toml"
    );
}

#[test]
fn test_gitignore_allows_source_commits() {
    let gitignore = fs::read_to_string(".gitignore")
        .expect("Debe poder leer .gitignore");
    
    // Verificar que NO bloquea src/bin/
    assert!(
        !gitignore.contains("src/bin/") && !gitignore.contains("src/"),
        ".gitignore no debe bloquear directorio src/"
    );
    
    // Verificar que bloquea target/ (binarios compilados)
    assert!(
        gitignore.contains("target/") || gitignore.contains("target"),
        ".gitignore debe bloquear target/ para evitar commits de binarios"
    );
}

#[test]
fn test_workflow_continue_on_error_for_known_issues() {
    let ci_yml = fs::read_to_string(".github/workflows/ci.yml")
        .expect("Debe poder leer ci.yml");
    
    // Verificar que pasos conocidos como problemáticos tienen continue-on-error
    assert!(
        ci_yml.contains("continue-on-error"),
        "CI debe usar continue-on-error para pasos que pueden fallar"
    );
}

/// Test de integración completo: simula el flujo de un PR de Copilot
#[test]
fn test_copilot_pr_workflow_simulation() {
    // Este test simula el flujo completo de un PR creado por Copilot
    
    // 1. Verificar que las reglas están documentadas
    assert!(
        Path::new("AGENTS.md").exists() && Path::new(".github/copilot-instructions.md").exists(),
        "Paso 1: Copilot debe tener acceso a reglas (AGENTS.md, copilot-instructions.md)"
    );
    
    // 2. Verificar que el template de PR existe
    assert!(
        Path::new(".github/PULL_REQUEST_TEMPLATE.md").exists(),
        "Paso 2: Debe existir template para guiar descripción del PR"
    );
    
    // 3. Verificar que CI se ejecutará en el PR
    let ci_yml = fs::read_to_string(".github/workflows/ci.yml")
        .expect("Paso 3: CI debe existir");
    assert!(
        ci_yml.contains("pull_request:"),
        "Paso 3: CI debe ejecutarse en pull_request"
    );
    
    // 4. Verificar que se validarán las 5 tools
    assert!(
        ci_yml.contains("test_exactly_5_tools"),
        "Paso 4: CI debe validar exactamente 5 tools"
    );
    
    // 5. Verificar que se detectarán mocks
    let mcp_validation = fs::read_to_string(".github/workflows/mcp-validation.yml")
        .expect("Paso 5: MCP validation debe existir");
    assert!(
        mcp_validation.contains("mock") || mcp_validation.contains("Mock"),
        "Paso 5: CI debe detectar mocks en tests"
    );
    
    println!("✅ Simulación completa: El workflow permite y valida correctamente PRs de Copilot");
}

#[test]
fn test_multi_agent_pipeline_supports_pr_comments() {
    let pipeline = ".github/workflows/nuclear-advanced-pipeline.yml";
    
    if !Path::new(pipeline).exists() {
        println!("⚠️ Nuclear advanced pipeline no existe, saltando test");
        return;
    }
    
    let content = fs::read_to_string(pipeline)
        .expect("Debe poder leer nuclear-advanced-pipeline.yml");
    
    // Verificar que puede comentar en PRs
    if content.contains("pull_request") {
        // Si se ejecuta en PRs, debería poder comentar
        assert!(
            content.contains("createComment") || 
            content.contains("github-script") ||
            content.contains("comment"),
            "Pipeline debe poder comentar resultados en PRs"
        );
    }
}
