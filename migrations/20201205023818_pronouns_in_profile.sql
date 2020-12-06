-- Add migration script here
ALTER TABLE public.profile_data
ADD COLUMN user_pronouns text COLLATE pg_catalog."default"
