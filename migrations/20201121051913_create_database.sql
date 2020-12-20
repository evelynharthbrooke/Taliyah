-- Add migration script here
CREATE TABLE IF NOT EXISTS public.guild_info
(
    guild_id BIGINT NOT NULL,
    guild_name TEXT COLLATE pg_catalog."default" NOT NULL,
    guild_prefix TEXT COLLATE pg_catalog."default",
    CONSTRAINT guild_info_pkey PRIMARY KEY (guild_id)
)

TABLESPACE pg_default;

ALTER TABLE public.guild_info
    OWNER to postgres;

COMMENT ON TABLE public.guild_info
    IS 'Information on guilds.';

CREATE TABLE IF NOT EXISTS public.profile_data
(
    "user_id" BIGINT NOT NULL,
    user_tag TEXT COLLATE pg_catalog."default" NOT NULL,
    "user_name" TEXT COLLATE pg_catalog."default",
    user_location TEXT COLLATE pg_catalog."default",
    user_gender TEXT COLLATE pg_catalog."default",
    user_twitch_id TEXT COLLATE pg_catalog."default",
    user_twitter_id TEXT COLLATE pg_catalog."default",
    user_lastfm_id TEXT COLLATE pg_catalog."default",
    user_steam_id TEXT COLLATE pg_catalog."default",
    user_xbox_id TEXT COLLATE pg_catalog."default",
    user_playstation_id TEXT COLLATE pg_catalog."default",
    CONSTRAINT profile_pkey PRIMARY KEY ("user_id")
)

TABLESPACE pg_default;

ALTER TABLE public.profile_data
    OWNER TO postgres;

COMMENT ON TABLE public.profile_data
    IS 'Individual user profile data.';
