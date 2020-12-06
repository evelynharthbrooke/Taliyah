-- Add migration script here
COMMENT ON COLUMN profile_data.user_id IS 'The user''s Discord identifier.';
COMMENT ON COLUMN profile_data.user_tag IS 'The user''s Discord discriminator.';
COMMENT ON COLUMN profile_data.user_name IS 'The user''s display name.';
COMMENT ON COLUMN profile_data.user_location IS 'The user''s location.';
COMMENT ON COLUMN profile_data.user_lastfm_id IS 'The user''s username for the Last.fm service.';
COMMENT ON COLUMN profile_data.user_pronouns IS 'The user''s chosen pronouns.';
