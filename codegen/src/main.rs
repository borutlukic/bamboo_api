use openapi_to_rust::{SchemaAnalyzer, CodeGenerator, GeneratorConfig, SchemaAnalysis};
use std::io::Read;
use std::path::PathBuf;

const BAMBOO_OPENAPI: &str = "openapi/swagger.json";
const BAMBOO_OPENAPI_URL: &str = "https://docs.atlassian.com/atlassian-bamboo/REST/10.2.3/swagger.json";

fn rename_in_schema_type(schema_type: &mut openapi_to_rust::analysis::SchemaType, original_name: &str, new_name: &str) {
    use openapi_to_rust::analysis::SchemaType;
    match schema_type {
        SchemaType::Reference { target } => {
            if target == original_name {
                *target = new_name.to_string();
            }
        }
        SchemaType::Array { item_type } => {
            rename_in_schema_type(item_type, original_name, new_name);
        }
        SchemaType::Union { variants } => {
            for v in variants.iter_mut() {
                if v.target == original_name {
                    v.target = new_name.to_string();
                }
            }
        }
        SchemaType::Composition { schemas } => {
            for s in schemas.iter_mut() {
                if s.target == original_name {
                    s.target = new_name.to_string();
                }
            }
        }
        SchemaType::DiscriminatedUnion { variants, .. } => {
            for v in variants.iter_mut() {
                if v.schema_ref == original_name {
                    v.schema_ref = new_name.to_string();
                }
            }
        }
        SchemaType::Object { properties, .. } => {
            for (_, property) in properties.iter_mut() {
                rename_in_schema_type(&mut property.schema_type, original_name, new_name);
            }
        }
        _ => {}
    }
}

fn rename_schema(analysis: &mut SchemaAnalysis, original_name: String, new_name: String) {
    println!("renaming api schema {original_name} to {new_name}");
    if let Some(mut schema) = analysis.schemas.remove(&original_name) {
        schema.name = new_name.clone();
        analysis.schemas.insert(new_name.clone(), schema);
    }

    for (_, schema) in analysis.schemas.iter_mut() {
        if schema.dependencies.remove(&original_name) {
            schema.dependencies.insert(new_name.clone());
        }
        match &mut schema.schema_type {
            openapi_to_rust::analysis::SchemaType::Object { required, properties, .. } => {
                if required.remove(&original_name) {
                    required.insert(new_name.clone());
                }
                for (_, property) in properties.iter_mut() {
                    rename_in_schema_type(&mut property.schema_type, &original_name, &new_name);
                }
            },
            _ => {},
        }
    }

    if let Some(deps) = analysis.dependencies.edges.remove(&"Option".to_string()) {
        analysis.dependencies.edges.insert(new_name.clone(), deps);
    }
    for (_, deps) in analysis.dependencies.edges.iter_mut() {
        if deps.remove(&original_name) {
            deps.insert(new_name.clone());
        }
    }

    for (_, op) in analysis.operations.iter_mut() {
        for (_, schema_name) in op.response_schemas.iter_mut() {
            if *schema_name == original_name {
                *schema_name = new_name.clone();
            }
        }
    }
}

fn fix_unused_path_params(analysis: &mut SchemaAnalysis) {
    use std::collections::HashSet;
    for (_, op) in analysis.operations.iter_mut() {
        let mut placeholders: HashSet<&str> = HashSet::new();
        let mut rest = op.path.as_str();
        while let Some(start) = rest.find('{') {
            rest = &rest[start + 1..];
            if let Some(end) = rest.find('}') {
                placeholders.insert(&rest[..end]);
                rest = &rest[end + 1..];
            }
        }
        let removed: Vec<_> = op
            .parameters
            .iter()
            .filter(|p| p.location == "path" && !placeholders.contains(p.name.as_str()))
            .map(|p| p.name.clone())
            .collect();
        for name in &removed {
            println!(
                "removing unused path param '{}' from operation '{}' (not in path '{}')",
                name, op.operation_id, op.path
            );
        }
        op.parameters
            .retain(|p| p.location != "path" || placeholders.contains(p.name.as_str()));
    }
}

fn patch_accept_header(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client_path = format!("{}/client.rs", output_dir);
    let src = std::fs::read_to_string(&client_path)?;

    let old = "let reqwest_client = reqwest::Client::new();";
    let new = r#"let reqwest_client = {
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(
                        reqwest::header::ACCEPT,
                        reqwest::header::HeaderValue::from_static("application/json"),
                    );
                    reqwest::Client::builder()
                        .default_headers(headers)
                        .build()
                        .expect("failed to build HTTP client")
                };"#;

    let count = src.matches(old).count();
    if count == 0 {
        eprintln!("warning: patch_accept_header: pattern not found in {client_path}");
        return Ok(());
    }

    let patched = src.replace(old, new);
    std::fs::write(&client_path, patched)?;
    println!("patched {count} occurrence(s) of reqwest::Client::new() to include Accept: application/json default header");
    Ok(())
}

/// Bamboo's OpenAPI spec has two classes of bugs in list wrapper types:
///
/// 1. **xml.name mismatch**: a property is named e.g. `plans` in the spec but carries
///    `xml: { name: "plan" }`. The REST API uses the XML element name as the JSON key,
///    so the actual response has `"plan": [...]`, not `"plans": [...]`. We rename the
///    spec property to the xml.name so the generated struct field matches reality.
///
/// 2. **Missing primary field**: some list types only have `allElements` (a generic
///    Java fallback) but the API returns data under a named key (`project`, `action`).
///    We add those fields explicitly.
fn patch_list_fields(spec: &mut serde_json::Value) {
    // --- Pass 1: rename array properties where xml.name != property name ---
    let schema_names: Vec<String> = spec
        .pointer("/components/schemas")
        .and_then(|s| s.as_object())
        .map(|m| m.keys().filter(|k| k.contains("List")).cloned().collect())
        .unwrap_or_default();

    for schema_name in &schema_names {
        let pointer = format!("/components/schemas/{}/properties", schema_name);
        let props = match spec.pointer(&pointer).and_then(|p| p.as_object()) {
            Some(p) => p.clone(),
            None => continue,
        };

        let renames: Vec<(String, String, serde_json::Value)> = props
            .iter()
            .filter_map(|(field, defn)| {
                let xml_name = defn.get("xml")?.get("name")?.as_str()?;
                if xml_name == field { return None; }
                if defn.get("type")?.as_str()? != "array" { return None; }
                Some((field.clone(), xml_name.to_string(), defn.clone()))
            })
            .collect();

        if renames.is_empty() { continue; }

        let props_mut = spec.pointer_mut(&pointer)
            .and_then(|p| p.as_object_mut())
            .unwrap();
        for (old_name, new_name, value) in &renames {
            props_mut.remove(old_name);
            props_mut.insert(new_name.clone(), value.clone());
            println!("patched {schema_name}: renamed '{old_name}' -> '{new_name}' (xml.name)");
        }
    }

    // --- Pass 2: add completely missing primary fields ---
    let missing: &[(&str, &str, &str)] = &[
        // (schema name, missing field name, $ref target)
        ("RestProjectList", "project", "RestProject"),
        ("RestPlanActionList", "action", "RestPlanAction"),
    ];

    for (schema_name, field_name, ref_target) in missing {
        let pointer = format!("/components/schemas/{}/properties", schema_name);
        if let Some(properties) = spec.pointer_mut(&pointer) {
            if properties.get(field_name).is_none() {
                properties[field_name] = serde_json::json!({
                    "type": "array",
                    "items": { "$ref": format!("#/components/schemas/{}", ref_target) }
                });
                println!("patched {schema_name}: added missing '{field_name}' property -> {ref_target}");
            }
        } else {
            eprintln!("warning: patch_list_fields: schema '{schema_name}' not found in spec");
        }
    }
}

/// Fix field type mismatches where the spec declares a primitive but the API returns an object.
/// These cause hard deserialization failures at runtime.
fn patch_field_types(spec: &mut serde_json::Value) {
    let fixes: &[(&str, &str, &str)] = &[
        // RestPlan.planKey is declared as `string` in the spec but the API returns
        // an object {"key":"CL-AA"} which matches RestKey.
        ("RestPlan", "planKey", "RestKey"),
    ];

    for (schema_name, field_name, ref_target) in fixes {
        let pointer = format!("/components/schemas/{}/properties/{}", schema_name, field_name);
        if let Some(field) = spec.pointer_mut(&pointer) {
            *field = serde_json::json!({ "$ref": format!("#/components/schemas/{}", ref_target) });
            println!("patched {schema_name}.{field_name}: changed type to $ref {ref_target}");
        } else {
            eprintln!("warning: patch_field_types: {schema_name}.{field_name} not found in spec");
        }
    }
}

fn download_spec_if_missing() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(BAMBOO_OPENAPI);
    if path.exists() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    println!("Downloading Bamboo OpenAPI spec from {}", BAMBOO_OPENAPI_URL);
    let response = ureq::get(BAMBOO_OPENAPI_URL).call()?;
    let mut body = String::new();
    response.into_reader().read_to_string(&mut body)?;
    std::fs::write(path, body)?;
    println!("Saved Bamboo OpenAPI spec to {}", BAMBOO_OPENAPI);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_spec_if_missing()?;

    let spec_content = std::fs::read_to_string(BAMBOO_OPENAPI)?;
    let mut spec_value: serde_json::Value = serde_json::from_str(&spec_content)?;

    patch_list_fields(&mut spec_value);
    patch_field_types(&mut spec_value);

    let mut analyzer = SchemaAnalyzer::new(spec_value)?;
    let mut analysis = analyzer.analyze()?;

    // Atlassian has created a struct called Result which interferes with rust's std::Result so we rename it
    rename_schema(&mut analysis, "Result".to_string(), "BuildResult".to_string());

    // Remove path parameters declared in the spec but absent from the path template
    fix_unused_path_params(&mut analysis);

    let config = GeneratorConfig {
        spec_path: PathBuf::from(BAMBOO_OPENAPI),
        output_dir: PathBuf::from("src/generated"),
        module_name: "bamboo_api".to_string(),
        enable_sse_client: false,
        enable_async_client: true,
        tracing_enabled: true,
        ..Default::default()
    };

    let generator = CodeGenerator::new(config);
    let result = generator.generate_all(&mut analysis)?;
    generator.write_files(&result)?;
    patch_accept_header("src/generated")?;

    println!("Generated code written to src/generated/");
    Ok(())
}
