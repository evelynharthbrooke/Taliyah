-- Add migration script here
CREATE TABLE IF NOT EXISTS public.guild_info
(
    guild_id bigint NOT NULL,
    guild_name text COLLATE pg_catalog."default" NOT NULL,
    guild_prefix text COLLATE pg_catalog."default",
    CONSTRAINT guild_info_pkey PRIMARY KEY (guild_id)
)

TABLESPACE pg_default;

ALTER TABLE public.guild_info
    OWNER to postgres;

COMMENT ON TABLE public.guild_info
    IS 'Information on guilds.';

CREATE TABLE IF NOT EXISTS public.profile_data
(
    "user_id" bigint NOT NULL,
    user_tag text COLLATE pg_catalog."default" NOT NULL,
    "user_name" text COLLATE pg_catalog."default",
    user_location text COLLATE pg_catalog."default",
    user_gender text COLLATE pg_catalog."default",
    user_twitch_id text COLLATE pg_catalog."default",
    user_twitter_id text COLLATE pg_catalog."default",
    user_lastfm_id text COLLATE pg_catalog."default",
    user_steam_id text COLLATE pg_catalog."default",
    user_xbox_id text COLLATE pg_catalog."default",
    user_playstation_id text COLLATE pg_catalog."default",
    CONSTRAINT profile_pkey PRIMARY KEY ("user_id")
)

TABLESPACE pg_default;

ALTER TABLE public.profile_data
    OWNER to postgres;

COMMENT ON TABLE public.profile_data
    IS 'Individual user profile data.';
