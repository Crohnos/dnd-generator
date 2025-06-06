use crate::error::ApiResult;
use crate::models::Campaign;
use sqlx::{PgPool, Transaction, Postgres, Row};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use chrono::{Utc, NaiveDate};

pub struct DatabaseServiceEnhanced {
    pool: PgPool,
}

impl DatabaseServiceEnhanced {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn begin_transaction(&self) -> ApiResult<Transaction<'_, Postgres>> {
        Ok(self.pool.begin().await?)
    }

    // Phase tracking methods
    pub async fn initialize_generation_phases(&self, campaign_id: i32, total_phases: i32) -> ApiResult<()> {
        sqlx::query(
            "UPDATE campaigns SET status = 'generating', total_phases = $2, phase_progress = 0, generation_phase = 'world_building' WHERE id = $1"
        )
        .bind(campaign_id)
        .bind(total_phases)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_generation_phase(
        &self, 
        campaign_id: i32, 
        phase: &str, 
        progress: i32, 
        status: Option<&str>
    ) -> ApiResult<()> {
        sqlx::query(
            r#"
            UPDATE campaigns 
            SET generation_phase = $2, phase_progress = $3, current_phase_status = $4, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#
        )
        .bind(campaign_id)
        .bind(phase)
        .bind(progress)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_campaign_status_with_error(&self, campaign_id: i32, error_msg: &str) -> ApiResult<()> {
        sqlx::query(
            "UPDATE campaigns SET status = 'error', error_message = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(campaign_id)
        .bind(error_msg)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    
    pub async fn update_campaign_status_completed(&self, campaign_id: i32) -> ApiResult<()> {
        sqlx::query(
            "UPDATE campaigns SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(campaign_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_campaign_metadata(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        campaign_id: i32,
        phase: &str,
        content: &JsonValue,
    ) -> ApiResult<()> {
        let metadata_update = json!({
            phase: content
        });

        sqlx::query(
            "UPDATE campaigns SET metadata = metadata || $2, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(campaign_id)
        .bind(&metadata_update)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_campaign(&self, campaign_id: i32) -> ApiResult<Campaign> {
        let campaign = sqlx::query_as::<_, Campaign>(
            "SELECT * FROM campaigns WHERE id = $1"
        )
        .bind(campaign_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(campaign)
    }

    // Phase 1: World Building Methods
    pub async fn save_calendar_system(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, calendar: &JsonValue) -> ApiResult<i32> {
        // Extract and format months as JSONB
        let default_months = json!([
            {"name": "January", "days": 31},
            {"name": "February", "days": 28},
            {"name": "March", "days": 31},
            {"name": "April", "days": 30},
            {"name": "May", "days": 31},
            {"name": "June", "days": 30},
            {"name": "July", "days": 31},
            {"name": "August", "days": 31},
            {"name": "September", "days": 30},
            {"name": "October", "days": 31},
            {"name": "November", "days": 30},
            {"name": "December", "days": 31}
        ]);
        let months = calendar.get("months").unwrap_or(&default_months);
        
        // Extract and format weekdays as JSONB
        let default_weekdays = json!(["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]);
        let weekdays = calendar.get("weekdays").unwrap_or(&default_weekdays);
        
        // Calculate year length from months if not provided
        let year_length = calendar.get("year_length")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or_else(|| {
                // Calculate from months array if possible
                if let Some(months_arr) = months.as_array() {
                    months_arr.iter()
                        .filter_map(|m| m.get("days").and_then(|d| d.as_i64()))
                        .sum::<i64>() as i32
                } else {
                    365
                }
            });
        
        let calendar_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO calendar_systems (
                campaign_id, name, months, weekdays, year_length,
                current_year, current_month, current_day, 
                special_events, lunar_cycles
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(calendar.get("name").and_then(|v| v.as_str()).unwrap_or("Custom Calendar"))
        .bind(&months)
        .bind(&weekdays)
        .bind(year_length)
        .bind(calendar.get("current_year").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1))
        .bind(calendar.get("current_month").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1))
        .bind(calendar.get("current_day").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1))
        .bind(calendar.get("special_events").unwrap_or(&json!([])))
        .bind(calendar.get("lunar_cycles").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(calendar_id)
    }

    pub async fn save_world_history_period(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, period: &JsonValue) -> ApiResult<i32> {
        let period_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO world_history (campaign_id, era_name, start_year, end_year, description, major_events, significance)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(period.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Period"))
        .bind(period.get("start_year").and_then(|v| v.as_i64()).unwrap_or(-2000) as i32)
        .bind(period.get("end_year").and_then(|v| v.as_i64()).unwrap_or(-1000) as i32)
        .bind(period.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(period.get("major_events").unwrap_or(&json!([])))
        .bind(period.get("significance").and_then(|v| v.as_str()).unwrap_or("Historical period"))
        .fetch_one(&mut **tx)
        .await?;

        Ok(period_id)
    }

    pub async fn save_plane(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, plane: &JsonValue) -> ApiResult<i32> {
        let plane_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO planes (campaign_id, name, description, plane_type, accessibility, native_creatures, planar_traits, notable_locations)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(plane.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Plane"))
        .bind(plane.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(plane.get("type").and_then(|v| v.as_str()).unwrap_or("Material"))
        .bind(plane.get("accessibility").and_then(|v| v.as_str()).unwrap_or("Normal"))
        .bind(plane.get("native_creatures").unwrap_or(&json!({})))
        .bind(plane.get("planar_traits").unwrap_or(&json!({})))
        .bind(plane.get("notable_locations").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(plane_id)
    }

    pub async fn save_pantheon(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, pantheon: &JsonValue) -> ApiResult<i32> {
        let pantheon_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO pantheons (campaign_id, name, description, origin_culture, pantheon_type, influence_level)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(pantheon.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Pantheon"))
        .bind(pantheon.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(pantheon.get("origin_culture").and_then(|v| v.as_str()))
        .bind(pantheon.get("pantheon_type").and_then(|v| v.as_str()).unwrap_or("Polytheistic"))
        .bind(pantheon.get("influence_level").and_then(|v| v.as_str()).unwrap_or("Regional"))
        .fetch_one(&mut **tx)
        .await?;

        Ok(pantheon_id)
    }

    pub async fn save_deity(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, pantheon_id: Option<i32>, deity: &JsonValue) -> ApiResult<i32> {
        // Convert domains from JSON array to string array
        let domains = deity.get("domains")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert all required arrays for the actual schema
        let titles = deity.get("titles")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let portfolio = deity.get("portfolio")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let personality_traits = deity.get("personality_traits")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let worshiper_alignments = deity.get("worshiper_alignments")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let clergy_alignments = deity.get("clergy_alignments")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let holy_days = deity.get("holy_days")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert major_temples from JSON array to integer array
        let major_temples = deity.get("major_temples")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect::<Vec<i32>>())
            .unwrap_or_else(Vec::new);
            
        let deity_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO deities (campaign_id, pantheon_id, name, title, alignment, domains, symbol, description, titles, portfolio, favored_weapon, divine_rank, appearance, personality_traits, relationships_with_other_deities, worshiper_alignments, clergy_alignments, holy_days, creation_myths, major_temples, is_active, worshippers, clergy_structure)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(pantheon_id)
        .bind(deity.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Deity"))
        .bind(deity.get("title").and_then(|v| v.as_str()).unwrap_or("Divine Being"))
        .bind(deity.get("alignment").and_then(|v| v.as_str()).unwrap_or("True Neutral"))
        .bind(&domains)
        .bind(deity.get("symbol").or_else(|| deity.get("holy_symbol")).and_then(|v| v.as_str()).unwrap_or("None"))
        .bind(deity.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(&titles)
        .bind(&portfolio)
        .bind(deity.get("favored_weapon").and_then(|v| v.as_str()))
        .bind(deity.get("divine_rank").and_then(|v| v.as_str()).unwrap_or("lesser"))
        .bind(deity.get("appearance").and_then(|v| v.as_str()))
        .bind(&personality_traits)
        .bind(deity.get("relationships_with_other_deities").unwrap_or(&json!({})))
        .bind(&worshiper_alignments)
        .bind(&clergy_alignments)
        .bind(&holy_days)
        .bind(deity.get("creation_myths").and_then(|v| v.as_str()))
        .bind(&major_temples)
        .bind(deity.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true))
        .bind(deity.get("worshippers").unwrap_or(&json!({})))
        .bind(deity.get("clergy_structure").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(deity_id)
    }

    pub async fn save_geography_region(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, region: &JsonValue) -> ApiResult<i32> {
        let region_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO geography_regions (campaign_id, name, region_type, climate, terrain, notable_features, native_flora, native_fauna, resources, hazards)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(region.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Region"))
        .bind(region.get("type").and_then(|v| v.as_str()).unwrap_or("Varied"))
        .bind(region.get("climate").and_then(|v| v.as_str()).unwrap_or("Temperate"))
        .bind(region.get("terrain").and_then(|v| v.as_str()).unwrap_or("Mixed"))
        .bind(region.get("notable_features").or_else(|| region.get("terrain_features")).unwrap_or(&json!({})))
        .bind(region.get("native_flora").unwrap_or(&json!({})))
        .bind(region.get("native_fauna").unwrap_or(&json!({})))
        .bind(region.get("resources").unwrap_or(&json!({})))
        .bind(region.get("hazards").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(region_id)
    }

    pub async fn save_economic_system(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, economic: &JsonValue) -> ApiResult<i32> {
        let economic_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO economic_systems (campaign_id, name, economic_type, base_currency, currency_system, trade_routes, major_exports, major_imports, taxation_system)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(economic.get("name").and_then(|v| v.as_str()).unwrap_or("Standard Economy"))
        .bind(economic.get("type").and_then(|v| v.as_str()).unwrap_or("Market"))
        .bind(economic.get("base_currency").and_then(|v| v.as_str()).unwrap_or("Gold"))
        .bind(economic.get("currency_system").or_else(|| economic.get("currency_types")).unwrap_or(&json!({"gold": 1, "silver": 10, "copper": 100})))
        .bind(economic.get("trade_routes").unwrap_or(&json!({})))
        .bind(economic.get("major_exports").or_else(|| economic.get("trade_goods")).unwrap_or(&json!({})))
        .bind(economic.get("major_imports").unwrap_or(&json!({})))
        .bind(economic.get("taxation_system").unwrap_or(&json!({"rate": "10%", "type": "income"})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(economic_id)
    }

    pub async fn save_legal_system(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, legal: &JsonValue) -> ApiResult<i32> {
        let legal_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO legal_systems (campaign_id, name, government_type, ruling_body, law_enforcement, court_system, punishment_system, citizen_rights)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(legal.get("name").and_then(|v| v.as_str()).unwrap_or("Local Laws"))
        .bind(legal.get("type").and_then(|v| v.as_str()).unwrap_or("Feudal"))
        .bind(legal.get("ruling_body").and_then(|v| v.as_str()).unwrap_or("Local Lord"))
        .bind(legal.get("law_enforcement").or_else(|| legal.get("enforcement_methods")).unwrap_or(&json!({"guards": "city watch", "militia": "volunteer"})))
        .bind(legal.get("court_system").unwrap_or(&json!({"type": "magistrate", "appeals": "lord"})))
        .bind(legal.get("punishment_system").unwrap_or(&json!({"minor": "fines", "major": "imprisonment", "capital": "execution"})))
        .bind(legal.get("citizen_rights").unwrap_or(&json!(["property", "trial", "petition"])))
        .fetch_one(&mut **tx)
        .await?;

        Ok(legal_id)
    }

    pub async fn save_astronomy(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, astronomy: &JsonValue) -> ApiResult<i32> {
        let astronomy_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO astronomy (campaign_id, name, description, celestial_bodies, constellations, astronomical_events, calendar_influence)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(astronomy.get("name").and_then(|v| v.as_str()).unwrap_or("Standard Sky"))
        .bind(astronomy.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(astronomy.get("celestial_bodies").unwrap_or(&json!({})))
        .bind(astronomy.get("constellations").unwrap_or(&json!({})))
        .bind(astronomy.get("astronomical_events").unwrap_or(&json!({})))
        .bind(astronomy.get("calendar_influence").or_else(|| astronomy.get("calendar_influences")).unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(astronomy_id)
    }

    pub async fn save_zodiac_sign(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, _astronomy_id: i32, zodiac: &JsonValue) -> ApiResult<i32> {
        // Convert traits from JSON array to string array
        let traits = zodiac.get("traits")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let zodiac_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO zodiac_signs (campaign_id, name, symbol, dates_range, traits, element, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(zodiac.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Sign"))
        .bind(zodiac.get("symbol").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(zodiac.get("dates_range").or_else(|| zodiac.get("time_period")).and_then(|v| v.as_str()).unwrap_or(""))
        .bind(&traits)
        .bind(zodiac.get("element").and_then(|v| v.as_str()).unwrap_or("Neutral"))
        .bind(zodiac.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .fetch_one(&mut **tx)
        .await?;

        Ok(zodiac_id)
    }

    pub async fn save_language(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, language: &JsonValue) -> ApiResult<i32> {
        let language_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO languages (campaign_id, name, language_type, script, speakers, prevalence, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(language.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Language"))
        .bind(language.get("language_type").and_then(|v| v.as_str()).unwrap_or("Standard"))
        .bind(language.get("script").and_then(|v| v.as_str()).unwrap_or("Common"))
        .bind(language.get("speakers").unwrap_or(&json!({})))
        .bind(language.get("prevalence").and_then(|v| v.as_str()).unwrap_or("Regional"))
        .bind(language.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .fetch_one(&mut **tx)
        .await?;

        Ok(language_id)
    }

    pub async fn save_race(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, race: &JsonValue) -> ApiResult<i32> {
        // Convert languages from JSON array to string array
        let languages = race.get("languages")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let race_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO races (campaign_id, name, description, size, speed, languages, racial_traits, ability_score_increases, lifespan, alignment_tendencies)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(race.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Race"))
        .bind(race.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(race.get("size").and_then(|v| v.as_str()).unwrap_or("Medium"))
        .bind(race.get("speed").and_then(|v| v.as_i64()).unwrap_or(30) as i32)
        .bind(&languages)
        .bind(race.get("traits").or_else(|| race.get("racial_traits")).unwrap_or(&json!({})))
        .bind(race.get("ability_score_increases").or_else(|| race.get("ability_modifiers")).unwrap_or(&json!({})))
        .bind(race.get("lifespan").and_then(|v| v.as_str()))
        .bind(race.get("alignment_tendencies").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(race_id)
    }

    pub async fn save_subrace(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, race_id: i32, subrace: &JsonValue) -> ApiResult<i32> {
        let subrace_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO subraces (campaign_id, race_id, name, description, additional_traits, additional_ability_scores)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(race_id)
        .bind(subrace.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Subrace"))
        .bind(subrace.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(subrace.get("additional_traits").or_else(|| subrace.get("traits")).unwrap_or(&json!({})))
        .bind(subrace.get("additional_ability_scores").or_else(|| subrace.get("ability_score_increases")).unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(subrace_id)
    }

    pub async fn save_class(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, class: &JsonValue) -> ApiResult<i32> {
        // Convert primary_abilities to array if needed
        let primary_abilities = class.get("primary_abilities")
            .or_else(|| class.get("primary_ability"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert saving_throws to array if needed
        let saving_throws = class.get("saving_throws")
            .or_else(|| class.get("saving_throw_proficiencies"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let class_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO classes (campaign_id, name, description, hit_die, primary_ability, saving_throw_proficiencies, skill_proficiencies, equipment_proficiencies, class_features, spellcasting_ability)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(class.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Class"))
        .bind(class.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(format!("d{}", class.get("hit_die").and_then(|v| v.as_i64()).unwrap_or(8)))
        .bind(&primary_abilities)
        .bind(&saving_throws)
        .bind(class.get("skill_proficiencies").unwrap_or(&json!({})))
        .bind(class.get("equipment_proficiencies").or_else(|| class.get("starting_equipment")).unwrap_or(&json!({})))
        .bind(class.get("class_features").unwrap_or(&json!({})))
        .bind(class.get("spellcasting_ability").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(class_id)
    }

    pub async fn save_subclass(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, class_id: i32, subclass: &JsonValue) -> ApiResult<i32> {
        let subclass_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO subclasses (campaign_id, class_id, name, description, additional_features, additional_spells)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(class_id)
        .bind(subclass.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Subclass"))
        .bind(subclass.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(subclass.get("features").or_else(|| subclass.get("additional_features")).unwrap_or(&json!({})))
        .bind(subclass.get("additional_spells").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(subclass_id)
    }


    // Additional placeholder methods that will be fully implemented
    pub async fn save_entity(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, entity_type: &str, entity_data: &JsonValue) -> ApiResult<i32> {
        let entity_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO entities (campaign_id, entity_type, name, description, metadata)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(entity_type)
        .bind(entity_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unnamed"))
        .bind(entity_data.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(entity_data.get("metadata").or_else(|| entity_data.get("stats")).or_else(|| entity_data.get("abilities")).unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(entity_id)
    }

    pub async fn save_player_character(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, entity_id: i32, pc_data: &JsonValue) -> ApiResult<i32> {
        // Convert personality data to JSONB (database stores as JSONB not TEXT[])
        let default_array = json!([]);
        let personality_traits = pc_data.get("personality_traits").unwrap_or(&default_array);
        let ideals = pc_data.get("ideals").unwrap_or(&default_array);
        let bonds = pc_data.get("bonds").unwrap_or(&default_array);
        let flaws = pc_data.get("flaws").unwrap_or(&default_array);
        
        let pc_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO player_characters (campaign_id, entity_id, player_name, race_id, subrace_id, class_id, subclass_id, background_id, level, experience_points, personality_traits, ideals, bonds, flaws, backstory)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(entity_id)
        .bind(pc_data.get("player_name").and_then(|v| v.as_str()))
        .bind(pc_data.get("race_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(pc_data.get("subrace_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(pc_data.get("class_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(pc_data.get("subclass_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(pc_data.get("background_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(pc_data.get("level").and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(pc_data.get("experience_points").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(personality_traits)
        .bind(ideals)
        .bind(bonds)
        .bind(flaws)
        .bind(pc_data.get("backstory").and_then(|v| v.as_str()).unwrap_or(""))
        .fetch_one(&mut **tx)
        .await?;

        Ok(pc_id)
    }

    pub async fn save_non_player_character(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, npc_data: &JsonValue) -> ApiResult<i32> {
        // Convert motivations from JSON array to string array
        let motivations = npc_data.get("motivations")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert fears from JSON array to string array
        let fears = npc_data.get("fears")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert secrets from JSON array to string array
        let secrets = npc_data.get("secrets")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .or_else(|| {
                // If single string secret, convert to array
                npc_data.get("secret_info")
                    .and_then(|v| v.as_str())
                    .map(|s| vec![s.to_string()])
            })
            .unwrap_or_else(Vec::new);
        
        let npc_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO non_player_characters (entity_id, role, occupation, race_id, class_id, level, personality, motivations, fears, secrets, stat_block)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(npc_data.get("role").and_then(|v| v.as_str()).unwrap_or("Citizen"))
        .bind(npc_data.get("occupation").and_then(|v| v.as_str()))
        .bind(npc_data.get("race_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(npc_data.get("class_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(npc_data.get("level").and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(npc_data.get("personality").unwrap_or(&json!({})))
        .bind(&motivations)
        .bind(&fears)
        .bind(&secrets)
        .bind(npc_data.get("stat_block").or_else(|| npc_data.get("stats")).unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(npc_id)
    }

    // Location methods
    pub async fn save_location(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, location_data: &JsonValue) -> ApiResult<i32> {
        // Convert notable_features and resources from JSON to arrays
        let notable_features = location_data.get("notable_features")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let resources = location_data.get("resources")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let location_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO locations (campaign_id, name, type, description, properties, population, government_type, economy, climate, terrain, danger_level, notable_features, resources, is_secret)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(location_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unnamed Location"))
        .bind(location_data.get("type").and_then(|v| v.as_str()).unwrap_or("Settlement"))
        .bind(location_data.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(location_data.get("properties").unwrap_or(&json!({})))
        .bind(location_data.get("population").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(location_data.get("government_type").and_then(|v| v.as_str()))
        .bind(location_data.get("economy").unwrap_or(&json!({})))
        .bind(location_data.get("climate").and_then(|v| v.as_str()))
        .bind(location_data.get("terrain").and_then(|v| v.as_str()))
        .bind(location_data.get("danger_level").and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(&notable_features)
        .bind(&resources)
        .bind(location_data.get("is_secret").and_then(|v| v.as_bool()).unwrap_or(false))
        .fetch_one(&mut **tx)
        .await?;

        Ok(location_id)
    }

    // Stub implementations for other methods that would be called
    // These would need full implementation based on the enhanced schema

    pub async fn save_shop(&self, tx: &mut Transaction<'_, Postgres>, location_id: i32, shop_data: &JsonValue) -> ApiResult<i32> {
        // Convert specialties array
        let specialties = shop_data.get("specialties")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);

        // Convert inventory categories array
        let inventory_categories = shop_data.get("inventory_categories")
            .or_else(|| shop_data.get("notable_items"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);

        let shop_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO shops (location_id, shop_type, quality_level, price_modifier, specialties, inventory_categories, reputation)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(location_id)
        .bind(shop_data.get("shop_type").and_then(|v| v.as_str()).unwrap_or("general goods"))
        .bind(shop_data.get("quality_level").and_then(|v| v.as_str()).unwrap_or("standard"))
        .bind(shop_data.get("price_modifier").and_then(|v| v.as_f64()).unwrap_or(1.0))
        .bind(&specialties)
        .bind(&inventory_categories)
        .bind(shop_data.get("reputation").and_then(|v| v.as_str()).unwrap_or("neutral"))
        .fetch_one(&mut **tx)
        .await?;

        Ok(shop_id)
    }

    pub async fn save_tavern(&self, tx: &mut Transaction<'_, Postgres>, location_id: i32, tavern_data: &JsonValue) -> ApiResult<i32> {
        // Convert drink specialties array
        let drink_specialties = tavern_data.get("drink_specialties")
            .or_else(|| tavern_data.get("specialties"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);

        // Convert regular patrons array
        let regular_patrons = tavern_data.get("regular_patrons")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);

        // Convert rumors to JSONB
        let default_rumors = json!([]);
        let rumors = tavern_data.get("rumors_available")
            .or_else(|| tavern_data.get("rumors"))
            .unwrap_or(&default_rumors);

        let tavern_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO taverns (location_id, quality_level, room_count, room_price_cp, meal_price_cp, drink_specialties, regular_patrons, rumors)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(location_id)
        .bind(tavern_data.get("quality_level").and_then(|v| v.as_str()).unwrap_or("standard"))
        .bind(tavern_data.get("room_count").and_then(|v| v.as_i64()).unwrap_or(5) as i32)
        .bind(tavern_data.get("room_price_cp").and_then(|v| v.as_i64()).unwrap_or(200) as i32)
        .bind(tavern_data.get("meal_price_cp").and_then(|v| v.as_i64()).unwrap_or(30) as i32)
        .bind(&drink_specialties)
        .bind(&regular_patrons)
        .bind(rumors)
        .fetch_one(&mut **tx)
        .await?;

        Ok(tavern_id)
    }

    pub async fn save_temple(&self, tx: &mut Transaction<'_, Postgres>, location_id: i32, temple_data: &JsonValue) -> ApiResult<i32> {
        // Convert services offered array
        let services_offered = temple_data.get("services_offered")
            .or_else(|| temple_data.get("services"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);

        // Convert holy days to JSONB
        let default_holy_days = json!([]);
        let holy_days = temple_data.get("holy_days")
            .unwrap_or(&default_holy_days);

        // Convert relics to JSONB
        let default_relics = json!([]);
        let relics = temple_data.get("relics")
            .unwrap_or(&default_relics);

        // Find deity by name (if provided)
        let deity_name = temple_data.get("deity").and_then(|v| v.as_str());
        let deity_id: Option<i32> = if let Some(name) = deity_name {
            sqlx::query_scalar("SELECT id FROM deities WHERE LOWER(name) LIKE LOWER($1) LIMIT 1")
                .bind(format!("%{}%", name))
                .fetch_optional(&mut **tx)
                .await?
        } else {
            None
        };

        let temple_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO temples (location_id, deity_id, services_offered, holy_days, relics)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(location_id)
        .bind(deity_id)
        .bind(&services_offered)
        .bind(holy_days)
        .bind(relics)
        .fetch_one(&mut **tx)
        .await?;

        Ok(temple_id)
    }

    pub async fn save_dungeon(&self, tx: &mut Transaction<'_, Postgres>, location_id: i32, dungeon_data: &JsonValue) -> ApiResult<i32> {
        // Placeholder implementation
        Ok(1)
    }

    pub async fn save_item(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, item_data: &JsonValue) -> ApiResult<i32> {
        let item_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO items (campaign_id, item_type_id, name, description, rarity, value_cp, weight_lbs, properties, requires_attunement, cursed)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(item_data.get("item_type_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(item_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Item"))
        .bind(item_data.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(item_data.get("rarity").and_then(|v| v.as_str()).unwrap_or("common"))
        .bind(item_data.get("value_cp").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(item_data.get("weight_lbs").and_then(|v| v.as_f64()).unwrap_or(0.0))
        .bind(item_data.get("properties").unwrap_or(&json!({})))
        .bind(item_data.get("requires_attunement").and_then(|v| v.as_bool()).unwrap_or(false))
        .bind(item_data.get("cursed").and_then(|v| v.as_bool()).unwrap_or(false))
        .fetch_one(&mut **tx)
        .await?;

        Ok(item_id)
    }

    pub async fn save_weapon(&self, tx: &mut Transaction<'_, Postgres>, item_id: i32, weapon_data: &JsonValue) -> ApiResult<i32> {
        // Convert weapon_properties from JSON array to string array
        let weapon_properties = weapon_data.get("weapon_properties")
            .or_else(|| weapon_data.get("properties"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let weapon_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO weapons (item_id, weapon_category, damage_dice, damage_type, weapon_properties, range_normal, range_long)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(item_id)
        .bind(weapon_data.get("weapon_category").or_else(|| weapon_data.get("category")).and_then(|v| v.as_str()).unwrap_or("simple"))
        .bind(weapon_data.get("damage_dice").or_else(|| weapon_data.get("damage")).and_then(|v| v.as_str()))
        .bind(weapon_data.get("damage_type").and_then(|v| v.as_str()))
        .bind(&weapon_properties)
        .bind(weapon_data.get("range_normal").or_else(|| weapon_data.get("range")).and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(weapon_data.get("range_long").and_then(|v| v.as_i64()).map(|v| v as i32))
        .fetch_one(&mut **tx)
        .await?;

        Ok(weapon_id)
    }

    pub async fn save_armor(&self, tx: &mut Transaction<'_, Postgres>, item_id: i32, armor_data: &JsonValue) -> ApiResult<i32> {
        let armor_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO armor (item_id, armor_category, armor_class, strength_requirement, stealth_disadvantage)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(item_id)
        .bind(armor_data.get("armor_category").or_else(|| armor_data.get("category")).and_then(|v| v.as_str()).unwrap_or("light"))
        .bind(armor_data.get("armor_class").or_else(|| armor_data.get("ac")).and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(armor_data.get("strength_requirement").or_else(|| armor_data.get("str_req")).and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(armor_data.get("stealth_disadvantage").and_then(|v| v.as_bool()).unwrap_or(false))
        .fetch_one(&mut **tx)
        .await?;

        Ok(armor_id)
    }

    pub async fn save_potion(&self, tx: &mut Transaction<'_, Postgres>, item_id: i32, potion_data: &JsonValue) -> ApiResult<i32> {
        let potion_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO potions (item_id, potion_type, duration, effects, side_effects)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(item_id)
        .bind(potion_data.get("potion_type").or_else(|| potion_data.get("type")).and_then(|v| v.as_str()).unwrap_or("healing"))
        .bind(potion_data.get("duration").and_then(|v| v.as_str()))
        .bind(potion_data.get("effects").unwrap_or(&json!([])))
        .bind(potion_data.get("side_effects").unwrap_or(&json!([])))
        .fetch_one(&mut **tx)
        .await?;

        Ok(potion_id)
    }

    pub async fn save_spell_scroll(&self, tx: &mut Transaction<'_, Postgres>, item_id: i32, scroll_data: &JsonValue) -> ApiResult<i32> {
        // Convert components from JSON array to string array
        let components = scroll_data.get("components")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["V".to_string(), "S".to_string()]);
        
        let scroll_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO spell_scrolls (item_id, spell_name, spell_level, spell_school, casting_time, components, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(item_id)
        .bind(scroll_data.get("spell_name").or_else(|| scroll_data.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown Spell"))
        .bind(scroll_data.get("spell_level").or_else(|| scroll_data.get("level")).and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(scroll_data.get("spell_school").or_else(|| scroll_data.get("school")).and_then(|v| v.as_str()))
        .bind(scroll_data.get("casting_time").and_then(|v| v.as_str()))
        .bind(&components)
        .bind(scroll_data.get("description").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(scroll_id)
    }

    pub async fn save_sentient_item(&self, tx: &mut Transaction<'_, Postgres>, item_id: i32, sentient_data: &JsonValue) -> ApiResult<i32> {
        // Convert arrays
        let communication_methods = sentient_data.get("communication_methods")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["telepathy".to_string()]);
            
        let senses = sentient_data.get("senses")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["normal vision".to_string()]);
            
        let languages = sentient_data.get("languages")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["Common".to_string()]);
        
        let sentient_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO sentient_items (item_id, intelligence, wisdom, charisma, alignment, communication_methods, senses, languages, personality, purpose, conflict_behavior)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#
        )
        .bind(item_id)
        .bind(sentient_data.get("intelligence").and_then(|v| v.as_i64()).unwrap_or(10) as i32)
        .bind(sentient_data.get("wisdom").and_then(|v| v.as_i64()).unwrap_or(10) as i32)
        .bind(sentient_data.get("charisma").and_then(|v| v.as_i64()).unwrap_or(10) as i32)
        .bind(sentient_data.get("alignment").and_then(|v| v.as_str()))
        .bind(&communication_methods)
        .bind(&senses)
        .bind(&languages)
        .bind(sentient_data.get("personality").and_then(|v| v.as_str()))
        .bind(sentient_data.get("purpose").and_then(|v| v.as_str()))
        .bind(sentient_data.get("conflict_behavior").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(sentient_id)
    }

    pub async fn save_entity_item(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, item_id: i32) -> ApiResult<i32> {
        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO entity_items (entity_id, item_id, quantity, equipped)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(item_id)
        .bind(1)
        .bind(false)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn save_entity_relationship(&self, tx: &mut Transaction<'_, Postgres>, entity_a_id: i32, entity_b_id: i32, relationship: &JsonValue) -> ApiResult<i32> {
        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO entity_relationships (entity_a_id, entity_b_id, relationship_type, relationship_strength, history, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(entity_a_id)
        .bind(entity_b_id)
        .bind(relationship.get("relationship_type").and_then(|v| v.as_str()).unwrap_or("knows"))
        .bind(relationship.get("relationship_strength").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(relationship.get("history").and_then(|v| v.as_str()))
        .bind(relationship.get("notes").or_else(|| relationship.get("description")).and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn save_faction(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, faction_data: &JsonValue) -> ApiResult<i32> {
        // Convert goals and values to arrays
        let goals = faction_data.get("goals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let values = faction_data.get("values")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let faction_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO factions (campaign_id, name, faction_type, description, motto, symbols, goals, values, member_count, influence_level, resources)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(faction_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Faction"))
        .bind(faction_data.get("type").and_then(|v| v.as_str()).unwrap_or("organization"))
        .bind(faction_data.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(faction_data.get("motto").and_then(|v| v.as_str()))
        .bind(faction_data.get("symbols").unwrap_or(&json!({})))
        .bind(&goals)
        .bind(&values)
        .bind(faction_data.get("member_count").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(faction_data.get("influence_level").and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(faction_data.get("resources").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(faction_id)
    }

    pub async fn save_entity_faction(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, faction_id: i32, membership: &JsonValue) -> ApiResult<i32> {
        // Parse join_date if provided
        let join_date = membership.get("join_date")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
        
        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO entity_factions (entity_id, faction_id, rank, join_date, reputation, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(faction_id)
        .bind(membership.get("rank").and_then(|v| v.as_str()).unwrap_or("member"))
        .bind(join_date)
        .bind(membership.get("reputation").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(membership.get("notes").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn save_lore_entry(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, lore_data: &JsonValue) -> ApiResult<i32> {
        // Convert related arrays to i32 arrays
        let related_entities = lore_data.get("related_entities")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect::<Vec<i32>>())
            .unwrap_or_else(Vec::new);
            
        let related_locations = lore_data.get("related_locations")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect::<Vec<i32>>())
            .unwrap_or_else(Vec::new);
            
        let related_items = lore_data.get("related_items")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect::<Vec<i32>>())
            .unwrap_or_else(Vec::new);
        
        let lore_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO lore_entries (campaign_id, title, category, content, is_public, related_entities, related_locations, related_items)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(lore_data.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled"))
        .bind(lore_data.get("category").and_then(|v| v.as_str()).unwrap_or("general"))
        .bind(lore_data.get("content").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(lore_data.get("is_public").and_then(|v| v.as_bool()).unwrap_or(false))
        .bind(&related_entities)
        .bind(&related_locations)
        .bind(&related_items)
        .fetch_one(&mut **tx)
        .await?;

        Ok(lore_id)
    }

    pub async fn save_entity_language(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, language_data: &JsonValue) -> ApiResult<i32> {
        // WARNING: entity_languages table does not exist in database schema
        // This method is a no-op to prevent errors - table needs to be created
        tracing::warn!("save_entity_language called but entity_languages table does not exist");
        Ok(1)
    }

    pub async fn save_entity_condition(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, condition_data: &JsonValue) -> ApiResult<i32> {
        // WARNING: entity_conditions table does not exist in database schema
        // This method is a no-op to prevent errors - table needs to be created
        tracing::warn!("save_entity_condition called but entity_conditions table does not exist");
        Ok(1)
    }

    pub async fn get_campaign_entities(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32) -> ApiResult<Vec<JsonValue>> {
        let entities = sqlx::query(
            r#"
            SELECT id, name, entity_type, description
            FROM entities
            WHERE campaign_id = $1
            "#
        )
        .bind(campaign_id)
        .fetch_all(&mut **tx)
        .await?;
        
        let result: Vec<JsonValue> = entities.iter().map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or(0),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "entity_type": row.try_get::<String, _>("entity_type").unwrap_or_default(),
                "description": row.try_get::<Option<String>, _>("description").unwrap_or(None)
            })
        }).collect();
        
        Ok(result)
    }

    pub async fn save_creature(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, creature_data: &JsonValue) -> ApiResult<i32> {
        // Convert habitat from JSON array to string array
        let habitat = creature_data.get("habitat")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["various".to_string()]);
        
        let creature_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO creatures (entity_id, creature_type, size, challenge_rating, abilities, habitat, behavior_patterns)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(creature_data.get("type").and_then(|v| v.as_str()).unwrap_or("beast"))
        .bind(creature_data.get("size").and_then(|v| v.as_str()).unwrap_or("medium"))
        .bind(creature_data.get("challenge_rating").and_then(|v| v.as_str()).unwrap_or("1").parse::<f32>().unwrap_or(1.0))
        .bind(creature_data.get("abilities").or_else(|| creature_data.get("stats")).unwrap_or(&json!({})))
        .bind(&habitat)
        .bind(creature_data.get("behavior_patterns").or_else(|| creature_data.get("behavior")).and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(creature_id)
    }

    pub async fn save_flora(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, flora_data: &JsonValue) -> ApiResult<i32> {
        // Convert habitat from JSON array to string array
        let habitat = flora_data.get("habitat")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["various".to_string()]);
            
        // Convert uses from JSON array to string array
        let uses = flora_data.get("uses")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let flora_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO flora (entity_id, plant_type, rarity, habitat, uses, harvesting_requirements, properties)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(flora_data.get("type").and_then(|v| v.as_str()).unwrap_or("plant"))
        .bind(flora_data.get("rarity").and_then(|v| v.as_str()).unwrap_or("common"))
        .bind(&habitat)
        .bind(&uses)
        .bind(flora_data.get("harvesting_requirements").and_then(|v| v.as_str()))
        .bind(flora_data.get("properties").unwrap_or(&json!({})))
        .fetch_one(&mut **tx)
        .await?;

        Ok(flora_id)
    }

    pub async fn save_fauna(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, fauna_data: &JsonValue) -> ApiResult<i32> {
        // Convert habitat from JSON array to string array
        let habitat = fauna_data.get("habitat")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["various".to_string()]);
        
        let fauna_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO fauna (entity_id, animal_type, size, habitat, diet, behavior, domestication_difficulty)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(fauna_data.get("animal_type").or_else(|| fauna_data.get("type")).and_then(|v| v.as_str()))
        .bind(fauna_data.get("size").and_then(|v| v.as_str()))
        .bind(&habitat)
        .bind(fauna_data.get("diet").and_then(|v| v.as_str()))
        .bind(fauna_data.get("behavior").and_then(|v| v.as_str()))
        .bind(fauna_data.get("domestication_difficulty").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(fauna_id)
    }

    pub async fn save_quest(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, quest_data: &JsonValue, entity_mapping: &HashMap<String, i32>, location_mapping: &HashMap<String, i32>) -> ApiResult<i32> {
        // WARNING: quests table does not exist in database schema
        // This method is a no-op to prevent errors - table needs to be created
        tracing::warn!("save_quest called but quests table does not exist");
        Ok(1)
    }

    pub async fn save_faction_relationship(&self, tx: &mut Transaction<'_, Postgres>, faction_a_id: i32, faction_b_id: i32, relationship: &JsonValue) -> ApiResult<i32> {
        // Ensure faction_a_id < faction_b_id to satisfy CHECK constraint
        let (faction_a, faction_b) = if faction_a_id < faction_b_id {
            (faction_a_id, faction_b_id)
        } else {
            (faction_b_id, faction_a_id)
        };
        
        let rel_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO faction_relationships (faction_a_id, faction_b_id, relationship_type, standing, history, recent_events)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(faction_a)
        .bind(faction_b)
        .bind(relationship.get("relationship_type").or_else(|| relationship.get("type")).and_then(|v| v.as_str()).unwrap_or("neutral"))
        .bind(relationship.get("standing").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(relationship.get("history").and_then(|v| v.as_str()))
        .bind(relationship.get("recent_events").unwrap_or(&json!([])))
        .fetch_one(&mut **tx)
        .await?;

        Ok(rel_id)
    }

    pub async fn save_location_connection(&self, tx: &mut Transaction<'_, Postgres>, location_a_id: i32, location_b_id: i32, connection: &JsonValue) -> ApiResult<i32> {
        // Convert dangers from JSON array to string array (note: column is 'dangers' not 'hazards')
        let dangers = connection.get("dangers")
            .or_else(|| connection.get("hazards"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let conn_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO location_connections (from_location_id, to_location_id, connection_type, travel_time_hours, difficulty, dangers, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#
        )
        .bind(location_a_id)
        .bind(location_b_id)
        .bind(connection.get("connection_type").or_else(|| connection.get("type")).and_then(|v| v.as_str()).unwrap_or("road"))
        .bind(connection.get("travel_time_hours").or_else(|| connection.get("travel_time")).and_then(|v| v.as_f64()))
        .bind(connection.get("difficulty").and_then(|v| v.as_str()).unwrap_or("normal"))
        .bind(&dangers)
        .bind(connection.get("description").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(conn_id)
    }

    pub async fn save_entity_location(&self, tx: &mut Transaction<'_, Postgres>, entity_id: i32, location_id: i32, entity_location: &JsonValue) -> ApiResult<i32> {
        // Parse arrival_date if provided (let database default handle if None)
        let arrival_date = entity_location.get("arrival_date")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
        
        let id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO entity_locations (entity_id, location_id, arrival_date, purpose, notes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
        .bind(entity_id)
        .bind(location_id)
        .bind(arrival_date)
        .bind(entity_location.get("purpose").or_else(|| entity_location.get("relationship_type")).and_then(|v| v.as_str()))
        .bind(entity_location.get("notes").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn save_random_encounter_table(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, encounter_table: &JsonValue) -> ApiResult<i32> {
        let table_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO random_encounter_tables (campaign_id, name, environment_type, level_range_min, level_range_max, encounters)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(encounter_table.get("name").and_then(|v| v.as_str()).unwrap_or("Random Encounters"))
        .bind(encounter_table.get("environment_type").or_else(|| encounter_table.get("environment")).and_then(|v| v.as_str()))
        .bind(encounter_table.get("level_range_min").or_else(|| encounter_table.get("min_level")).and_then(|v| v.as_i64()).unwrap_or(1) as i32)
        .bind(encounter_table.get("level_range_max").or_else(|| encounter_table.get("max_level")).and_then(|v| v.as_i64()).unwrap_or(20) as i32)
        .bind(encounter_table.get("encounters").unwrap_or(&json!([])))
        .fetch_one(&mut **tx)
        .await?;

        Ok(table_id)
    }

    pub async fn save_weather_pattern(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, weather_pattern: &JsonValue) -> ApiResult<i32> {
        // Convert typical_conditions from JSON array to string array
        let typical_conditions = weather_pattern.get("typical_conditions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let weather_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO weather_patterns (campaign_id, region_id, season, typical_conditions, temperature_range_low, temperature_range_high, precipitation_chance, extreme_weather_events)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(weather_pattern.get("region_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(weather_pattern.get("season").and_then(|v| v.as_str()).unwrap_or("spring"))
        .bind(&typical_conditions)
        .bind(weather_pattern.get("temperature_range_low").or_else(|| weather_pattern.get("temp_low")).and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(weather_pattern.get("temperature_range_high").or_else(|| weather_pattern.get("temp_high")).and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(weather_pattern.get("precipitation_chance").and_then(|v| v.as_i64()).unwrap_or(30) as i32)
        .bind(weather_pattern.get("extreme_weather_events").unwrap_or(&json!([])))
        .fetch_one(&mut **tx)
        .await?;

        Ok(weather_id)
    }
    
    // Additional save methods for Phase 1 completeness
    pub async fn save_background(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, background: &JsonValue) -> ApiResult<i32> {
        // Convert skill_proficiencies from JSON array to string array
        let skill_profs = background.get("skill_proficiencies")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert languages from JSON array to string array
        let lang_profs = background.get("languages")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert tool_proficiencies and equipment to arrays
        let tool_profs = background.get("tool_proficiencies")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        let equipment = background.get("equipment")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Handle personality traits, ideals, bonds, flaws
        let personality_traits = background.get("personality_traits")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let ideals = background.get("ideals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let bonds = background.get("bonds")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let flaws = background.get("flaws")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
        
        // Format feature as JSONB
        let feature = json!({
            "name": background.get("feature").and_then(|f| f.get("name")).and_then(|v| v.as_str()).unwrap_or("Feature"),
            "description": background.get("feature").and_then(|f| f.get("description")).and_then(|v| v.as_str()).unwrap_or("")
        });
        
        // Format suggested_characteristics as JSONB
        let suggested_characteristics = json!({
            "personality_traits": personality_traits,
            "ideals": ideals,
            "bonds": bonds,
            "flaws": flaws
        });
        
        let background_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO backgrounds (campaign_id, name, description, skill_proficiencies, language_proficiencies, tool_proficiencies, equipment, feature, suggested_characteristics)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(background.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Background"))
        .bind(background.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(&skill_profs)
        .bind(&lang_profs)
        .bind(&tool_profs)
        .bind(&json!(equipment))
        .bind(&feature)
        .bind(&suggested_characteristics)
        .fetch_one(&mut **tx)
        .await?;

        Ok(background_id)
    }
    
    pub async fn save_culture(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, culture: &JsonValue) -> ApiResult<i32> {
        // Convert values from JSON array to string array
        let values = culture.get("values")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert taboos from JSON array to string array
        let taboos = culture.get("taboos")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert typical_occupations from JSON array to string array
        let typical_occupations = culture.get("typical_occupations")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert art_forms from JSON array to string array
        let art_forms = culture.get("art_forms")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        // Convert cuisine from JSON array to string array
        let cuisine = culture.get("cuisine")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(Vec::new);
            
        let culture_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO cultures (campaign_id, name, description, values, traditions, taboos, common_names, language_id, typical_occupations, art_forms, cuisine, architecture_style)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(culture.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Culture"))
        .bind(culture.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(&values)
        .bind(culture.get("traditions").unwrap_or(&json!([])))
        .bind(&taboos)
        .bind(culture.get("common_names").unwrap_or(&json!({})))
        .bind(culture.get("language_id").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(&typical_occupations)
        .bind(&art_forms)
        .bind(&cuisine)
        .bind(culture.get("architecture_style").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(culture_id)
    }
    
    pub async fn save_feat(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, feat: &JsonValue) -> ApiResult<i32> {
        let feat_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO feats (campaign_id, name, description, prerequisites, benefits, feat_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(feat.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Feat"))
        .bind(feat.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(feat.get("prerequisites").unwrap_or(&json!({})))
        .bind(feat.get("benefits").unwrap_or(&json!({})))
        .bind(feat.get("feat_type").and_then(|v| v.as_str()).unwrap_or("General"))
        .fetch_one(&mut **tx)
        .await?;

        Ok(feat_id)
    }
    
    pub async fn save_spell(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, spell: &JsonValue) -> ApiResult<i32> {
        // Convert components from JSON to string array
        let components = spell.get("components")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
            .unwrap_or_else(|| vec!["V".to_string(), "S".to_string()]);
        
        let spell_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO spells (campaign_id, name, level, school, casting_time, range, components, duration, description, higher_levels)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(spell.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Spell"))
        .bind(spell.get("level").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
        .bind(spell.get("school").and_then(|v| v.as_str()).unwrap_or("Unknown"))
        .bind(spell.get("casting_time").and_then(|v| v.as_str()).unwrap_or("1 action"))
        .bind(spell.get("range").and_then(|v| v.as_str()).unwrap_or("Self"))
        .bind(&components)
        .bind(spell.get("duration").and_then(|v| v.as_str()).unwrap_or("Instantaneous"))
        .bind(spell.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(spell.get("higher_levels").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(spell_id)
    }

    // Missing functions for tool-based generation
    
    pub async fn save_magic_item(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, magic_item: &JsonValue) -> ApiResult<i32> {
        // Combine properties and effects into mechanical_effects text
        let mechanical_effects = if let Some(properties) = magic_item.get("properties") {
            if let Some(arr) = properties.as_array() {
                arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("; ")
            } else if let Some(s) = properties.as_str() {
                s.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Determine if this is a sentient item
        let is_sentient = magic_item.get("personality").is_some() || 
                         magic_item.get("communication_method").is_some() ||
                         magic_item.get("intelligence").is_some() ||
                         magic_item.get("goals").is_some();

        let magic_item_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO magic_items (campaign_id, name, item_type, rarity, attunement_required, description, mechanical_effects, activation_method, charges, charge_recovery, curse_description, creator_name, historical_significance, physical_description, is_sentient, intelligence, personality)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(magic_item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Magic Item"))
        .bind(magic_item.get("item_type").and_then(|v| v.as_str()).unwrap_or("wondrous item"))
        .bind(magic_item.get("rarity").and_then(|v| v.as_str()).unwrap_or("common"))
        .bind(magic_item.get("attunement_required").and_then(|v| v.as_bool()).unwrap_or(false))
        .bind(magic_item.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(mechanical_effects)
        .bind(magic_item.get("activation_method").and_then(|v| v.as_str()))
        .bind(magic_item.get("charges").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(magic_item.get("recharge_method").or_else(|| magic_item.get("charge_recovery")).and_then(|v| v.as_str()))
        .bind(magic_item.get("curse_description").and_then(|v| v.as_str()))
        .bind(magic_item.get("creator_name").and_then(|v| v.as_str()))
        .bind(magic_item.get("history").or_else(|| magic_item.get("historical_significance")).and_then(|v| v.as_str()))
        .bind(magic_item.get("physical_description").and_then(|v| v.as_str()))
        .bind(is_sentient)
        .bind(magic_item.get("intelligence").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(magic_item.get("personality").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(magic_item_id)
    }


    pub async fn save_encounter(&self, tx: &mut Transaction<'_, Postgres>, campaign_id: i32, location_id: i32, encounter: &JsonValue) -> ApiResult<i32> {
        let encounter_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO encounters (campaign_id, location_id, encounter_type, name, description, difficulty, trigger_conditions, enemies, environmental_factors, treasure, experience_reward, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#
        )
        .bind(campaign_id)
        .bind(if location_id > 0 { Some(location_id) } else { None })
        .bind(encounter.get("encounter_type").and_then(|v| v.as_str()).unwrap_or("combat"))
        .bind(encounter.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Encounter"))
        .bind(encounter.get("description").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(encounter.get("difficulty").and_then(|v| v.as_str()).unwrap_or("medium"))
        .bind(encounter.get("triggers").or_else(|| encounter.get("trigger_conditions")).unwrap_or(&json!([])))
        .bind(encounter.get("enemies").unwrap_or(&json!([])))
        .bind(encounter.get("environmental_factors").unwrap_or(&json!([])))
        .bind(encounter.get("treasure").unwrap_or(&json!([])))
        .bind(encounter.get("experience_reward").and_then(|v| v.as_i64()).map(|v| v as i32))
        .bind(encounter.get("notes").and_then(|v| v.as_str()))
        .fetch_one(&mut **tx)
        .await?;

        Ok(encounter_id)
    }

    // Context building methods for phase-to-phase data passing
    pub async fn get_phase_1a_context(&self, _campaign_id: i32) -> ApiResult<JsonValue> {
        // Phase 1A is the first phase, no prior context needed
        Ok(json!({}))
    }

    pub async fn get_phase_1_context(&self, campaign_id: i32) -> ApiResult<JsonValue> {
        // Get Phase 1A data for Phase 1B and 1C
        let response = sqlx::query_scalar::<_, JsonValue>(
            "SELECT to_jsonb(row_to_json(t)) FROM (
                SELECT 
                    (SELECT json_agg(cs) FROM calendar_systems cs WHERE cs.campaign_id = $1) as calendar_systems,
                    (SELECT json_agg(p) FROM planes p WHERE p.campaign_id = $1) as planes,
                    (SELECT json_agg(gr) FROM geography_regions gr WHERE gr.campaign_id = $1) as geography_regions,
                    (SELECT json_agg(hp) FROM historical_periods hp WHERE hp.campaign_id = $1) as historical_periods,
                    (SELECT json_agg(es) FROM economic_systems es WHERE es.campaign_id = $1) as economic_systems,
                    (SELECT json_agg(ls) FROM legal_systems ls WHERE ls.campaign_id = $1) as legal_systems,
                    (SELECT json_agg(cb) FROM celestial_bodies cb WHERE cb.campaign_id = $1) as celestial_bodies
            ) t"
        )
        .bind(campaign_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(response.unwrap_or_else(|| json!({})))
    }

    pub async fn get_phase_2_context(&self, campaign_id: i32) -> ApiResult<JsonValue> {
        // Get Phase 1 + Phase 2A data for Phase 2B and 2C
        let response = sqlx::query_scalar::<_, JsonValue>(
            "SELECT to_jsonb(row_to_json(t)) FROM (
                SELECT 
                    -- Phase 1A context
                    (SELECT json_agg(cs) FROM calendar_systems cs WHERE cs.campaign_id = $1) as calendar_systems,
                    (SELECT json_agg(p) FROM planes p WHERE p.campaign_id = $1) as planes,
                    (SELECT json_agg(gr) FROM geography_regions gr WHERE gr.campaign_id = $1) as geography_regions,
                    -- Phase 1B context
                    (SELECT json_agg(r) FROM races r WHERE r.campaign_id = $1) as races,
                    (SELECT json_agg(cc) FROM character_classes cc WHERE cc.campaign_id = $1) as character_classes,
                    (SELECT json_agg(f) FROM feats f WHERE f.campaign_id = $1) as feats,
                    (SELECT json_agg(b) FROM backgrounds b WHERE b.campaign_id = $1) as backgrounds,
                    -- Phase 1C context
                    (SELECT json_agg(l) FROM languages l WHERE l.campaign_id = $1) as languages,
                    (SELECT json_agg(c) FROM cultures c WHERE c.campaign_id = $1) as cultures,
                    (SELECT json_agg(fa) FROM factions fa WHERE fa.campaign_id = $1) as factions,
                    (SELECT json_agg(pa) FROM pantheons pa WHERE pa.campaign_id = $1) as pantheons,
                    (SELECT json_agg(d) FROM deities d WHERE d.campaign_id = $1) as deities,
                    -- Phase 2A context
                    (SELECT json_agg(e) FROM entities e WHERE e.campaign_id = $1) as entities
            ) t"
        )
        .bind(campaign_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(response.unwrap_or_else(|| json!({})))
    }

    pub async fn get_phase_3_context(&self, campaign_id: i32) -> ApiResult<JsonValue> {
        // Get all Phase 1 + Phase 2 data for Phase 3
        let response = sqlx::query_scalar::<_, JsonValue>(
            "SELECT to_jsonb(row_to_json(t)) FROM (
                SELECT 
                    -- Phase 1 context (key elements)
                    (SELECT json_agg(cs) FROM calendar_systems cs WHERE cs.campaign_id = $1) as calendar_systems,
                    (SELECT json_agg(gr) FROM geography_regions gr WHERE gr.campaign_id = $1) as geography_regions,
                    (SELECT json_agg(r) FROM races r WHERE r.campaign_id = $1) as races,
                    (SELECT json_agg(c) FROM cultures c WHERE c.campaign_id = $1) as cultures,
                    (SELECT json_agg(fa) FROM factions fa WHERE fa.campaign_id = $1) as factions,
                    (SELECT json_agg(d) FROM deities d WHERE d.campaign_id = $1) as deities,
                    -- Phase 2 context
                    (SELECT json_agg(e) FROM entities e WHERE e.campaign_id = $1) as entities,
                    (SELECT json_agg(l) FROM locations l WHERE l.campaign_id = $1) as locations,
                    (SELECT json_agg(bu) FROM buildings bu WHERE bu.campaign_id = $1) as buildings,
                    (SELECT json_agg(du) FROM dungeons du WHERE du.campaign_id = $1) as dungeons,
                    (SELECT json_agg(i) FROM items i WHERE i.campaign_id = $1) as items
            ) t"
        )
        .bind(campaign_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(response.unwrap_or_else(|| json!({})))
    }

    pub async fn get_world_building_data(&self, campaign_id: i32) -> ApiResult<JsonValue> {
        // Legacy method - redirect to Phase 1 context
        self.get_phase_1_context(campaign_id).await
    }

    pub async fn get_pc_connected_data(&self, campaign_id: i32) -> ApiResult<JsonValue> {
        // Legacy method - redirect to Phase 2 context
        self.get_phase_2_context(campaign_id).await
    }
}