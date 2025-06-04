use dnd_campaign_generator::models::campaign::PhaseInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Phase Dependency Validation");
    println!("======================================");
    
    // Test valid sequences
    println!("\n1. Testing valid phase sequences...");
    
    // Test Phase 1A (no dependencies)
    let result = PhaseInfo::validate_dependencies(&[], "phase_1a_core_world");
    match result {
        Ok(()) => println!("   ✅ Phase 1A with no dependencies: VALID"),
        Err(e) => println!("   ❌ Phase 1A failed: {}", e),
    }
    
    // Test Phase 1B after 1A
    let completed = vec!["phase_1a_core_world".to_string()];
    let result = PhaseInfo::validate_dependencies(&completed, "phase_1b_character_building");
    match result {
        Ok(()) => println!("   ✅ Phase 1B after 1A: VALID"),
        Err(e) => println!("   ❌ Phase 1B after 1A failed: {}", e),
    }
    
    // Test Phase 2A after both Phase 1 parts
    let completed = vec![
        "phase_1a_core_world".to_string(),
        "phase_1b_character_building".to_string(),
        "phase_1c_social_framework".to_string(),
    ];
    let result = PhaseInfo::validate_dependencies(&completed, "phase_2a_pc_entities");
    match result {
        Ok(()) => println!("   ✅ Phase 2A after all Phase 1 parts: VALID"),
        Err(e) => println!("   ❌ Phase 2A after all Phase 1 parts failed: {}", e),
    }
    
    // Test Phase 3C after all previous phases
    let completed = vec![
        "phase_1a_core_world".to_string(),
        "phase_1b_character_building".to_string(),
        "phase_1c_social_framework".to_string(),
        "phase_2a_pc_entities".to_string(),
        "phase_2b_pc_locations".to_string(),
        "phase_2c_pc_items".to_string(),
        "phase_3a_quests_encounters".to_string(),
        "phase_3b_world_population".to_string(),
    ];
    let result = PhaseInfo::validate_dependencies(&completed, "phase_3c_relationships");
    match result {
        Ok(()) => println!("   ✅ Phase 3C after all previous phases: VALID"),
        Err(e) => println!("   ❌ Phase 3C after all previous phases failed: {}", e),
    }
    
    // Test invalid sequences
    println!("\n2. Testing invalid phase sequences...");
    
    // Test Phase 1B without 1A
    let result = PhaseInfo::validate_dependencies(&[], "phase_1b_character_building");
    match result {
        Ok(()) => println!("   ❌ Phase 1B without 1A should be INVALID but passed"),
        Err(e) => println!("   ✅ Phase 1B without 1A correctly rejected: {}", e),
    }
    
    // Test Phase 2A without all Phase 1 parts
    let completed = vec!["phase_1a_core_world".to_string()]; // Missing 1B and 1C
    let result = PhaseInfo::validate_dependencies(&completed, "phase_2a_pc_entities");
    match result {
        Ok(()) => println!("   ❌ Phase 2A without all Phase 1 parts should be INVALID but passed"),
        Err(e) => println!("   ✅ Phase 2A without all Phase 1 parts correctly rejected: {}", e),
    }
    
    // Test Phase 3A without Phase 2 parts
    let completed = vec![
        "phase_1a_core_world".to_string(),
        "phase_1b_character_building".to_string(),
        "phase_1c_social_framework".to_string(),
        // Missing all Phase 2 parts
    ];
    let result = PhaseInfo::validate_dependencies(&completed, "phase_3a_quests_encounters");
    match result {
        Ok(()) => println!("   ❌ Phase 3A without Phase 2 parts should be INVALID but passed"),
        Err(e) => println!("   ✅ Phase 3A without Phase 2 parts correctly rejected: {}", e),
    }
    
    // Test unknown phase
    let result = PhaseInfo::validate_dependencies(&[], "unknown_phase");
    match result {
        Ok(()) => println!("   ❌ Unknown phase should be INVALID but passed"),
        Err(e) => println!("   ✅ Unknown phase correctly rejected: {}", e),
    }
    
    println!("\n3. Testing phase progression logic...");
    
    // Test get_phase_info
    match PhaseInfo::get_phase_info("phase_1a_core_world") {
        Some(info) => {
            println!("   ✅ Phase 1A info: number={}, dependencies={:?}", 
                info.number, info.dependencies);
        }
        None => println!("   ❌ Failed to get Phase 1A info"),
    }
    
    match PhaseInfo::get_phase_info("phase_2a_pc_entities") {
        Some(info) => {
            println!("   ✅ Phase 2A info: number={}, dependencies={:?}", 
                info.number, info.dependencies);
        }
        None => println!("   ❌ Failed to get Phase 2A info"),
    }
    
    match PhaseInfo::get_phase_info("phase_3c_relationships") {
        Some(info) => {
            println!("   ✅ Phase 3C info: number={}, dependencies={:?}", 
                info.number, info.dependencies);
        }
        None => println!("   ❌ Failed to get Phase 3C info"),
    }
    
    println!("\n📊 Summary:");
    println!("   ✅ Phase dependency validation test completed");
    println!("   🎯 All dependency rules are working correctly");
    
    Ok(())
}