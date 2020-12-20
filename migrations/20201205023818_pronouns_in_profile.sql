-- Add migration script here
ALTER TABLE public.profile_data
ADD COLUMN user_pronouns TEXT COLLATE pg_catalog."default"
