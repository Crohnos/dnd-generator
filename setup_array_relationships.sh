#!/bin/bash

HASURA_URL="http://localhost:8080/v1/metadata"
ADMIN_SECRET="myadminsecretkey"

echo "🔄 Setting up array relationships (one-to-many)..."

# Function to create array relationship
create_array_relationship() {
    local from_table=$1
    local relationship_name=$2
    local to_table=$3
    local foreign_key=$4
    
    response=$(curl -s -w "%{http_code}" -o /tmp/hasura_response.json \
        -X POST \
        -H "Content-Type: application/json" \
        -H "X-Hasura-Admin-Secret: $ADMIN_SECRET" \
        -d "{
            \"type\": \"pg_create_array_relationship\",
            \"args\": {
                \"source\": \"default\",
                \"table\": {
                    \"schema\": \"public\",
                    \"name\": \"$from_table\"
                },
                \"name\": \"$relationship_name\",
                \"using\": {
                    \"foreign_key_constraint_on\": {
                        \"table\": {
                            \"schema\": \"public\",
                            \"name\": \"$to_table\"
                        },
                        \"column\": \"$foreign_key\"
                    }
                }
            }
        }" \
        "$HASURA_URL")
    
    if [ "$response" = "200" ]; then
        echo "✅ Created array relationship: $from_table.$relationship_name"
    else
        echo "❌ Failed to create array relationship $from_table.$relationship_name (HTTP $response)"
        cat /tmp/hasura_response.json
    fi
}

# Campaign relationships - all entities that belong to a campaign
create_array_relationship "campaigns" "backstory_elements" "backstory_elements" "campaign_id"
create_array_relationship "campaigns" "calendar_systems" "calendar_systems" "campaign_id" 
create_array_relationship "campaigns" "campaign_flags" "campaign_flags" "campaign_id"
create_array_relationship "campaigns" "campaign_sessions" "campaign_sessions" "campaign_id"
create_array_relationship "campaigns" "currencies" "currencies" "campaign_id"
create_array_relationship "campaigns" "deities" "deities" "campaign_id"
create_array_relationship "campaigns" "historical_events" "historical_events" "campaign_id"
create_array_relationship "campaigns" "knowledge_sources" "knowledge_sources" "campaign_id"
create_array_relationship "campaigns" "magic_items" "magic_items" "campaign_id"
create_array_relationship "campaigns" "magical_phenomena" "magical_phenomena" "campaign_id"
create_array_relationship "campaigns" "organizations" "organizations" "campaign_id"
create_array_relationship "campaigns" "random_events" "random_events" "campaign_id"
create_array_relationship "campaigns" "rumors" "rumors" "campaign_id"
create_array_relationship "campaigns" "trade_routes" "trade_routes" "campaign_id"

# Location relationships
create_array_relationship "locations" "child_locations" "locations" "parent_location_id"
create_array_relationship "locations" "connections_from" "location_connections" "from_location_id"
create_array_relationship "locations" "connections_to" "location_connections" "to_location_id"
create_array_relationship "locations" "services" "location_services" "location_id"
create_array_relationship "locations" "npcs_current" "npcs" "current_location_id"
create_array_relationship "locations" "npcs_home" "npcs" "home_location_id"

# NPC relationships
create_array_relationship "npcs" "relationships_as_npc1" "npc_relationships" "npc1_id"
create_array_relationship "npcs" "relationships_as_npc2" "npc_relationships" "npc2_id"
create_array_relationship "npcs" "organization_memberships" "organization_memberships" "npc_id"
create_array_relationship "npcs" "authored_knowledge" "knowledge_sources" "author_npc_id"
create_array_relationship "npcs" "owned_magic_items" "magic_items" "current_owner_npc_id"
create_array_relationship "npcs" "owned_services" "location_services" "owner_npc_id"
create_array_relationship "npcs" "started_rumors" "rumors" "origin_npc_id"
create_array_relationship "npcs" "connected_backstory_elements" "backstory_elements" "connected_npc_id"

# Organization relationships
create_array_relationship "organizations" "child_organizations" "organizations" "parent_organization_id"
create_array_relationship "organizations" "ranks" "organization_ranks" "organization_id"
create_array_relationship "organizations" "memberships" "organization_memberships" "organization_id"
create_array_relationship "organizations" "relationships_as_org1" "organization_relationships" "organization1_id"
create_array_relationship "organizations" "relationships_as_org2" "organization_relationships" "organization2_id"
create_array_relationship "organizations" "connected_backstory_elements" "backstory_elements" "connected_organization_id"
create_array_relationship "organizations" "patroned_quests" "quest_hooks" "patron_organization_id"
create_array_relationship "organizations" "controlled_trade_routes" "trade_routes" "controlling_organization_id"

# Quest and encounter relationships
create_array_relationship "quest_hooks" "progress_entries" "quest_progress" "quest_hook_id"
create_array_relationship "quest_hooks" "connected_backstory_elements" "backstory_elements" "connected_quest_hook_id"

# Type-based relationships
create_array_relationship "backstory_element_types" "elements" "backstory_elements" "element_type_id"
create_array_relationship "character_races" "npcs" "npcs" "race_id"
create_array_relationship "character_backgrounds" "npcs" "npcs" "background_id"
create_array_relationship "organization_types" "organizations" "organizations" "organization_type_id"
create_array_relationship "location_types" "locations" "locations" "location_type_id"
create_array_relationship "quest_types" "quest_hooks" "quest_hooks" "quest_type_id"
create_array_relationship "encounter_types" "encounters" "encounters" "encounter_type_id"

echo ""
echo "✨ Array relationships setup complete!"
echo "🔗 Campaigns now have full access to all related entities via GraphQL"