ALTER TABLE public.events
    ADD COLUMN description TEXT NOT NULL DEFAULT '',
    ADD COLUMN short_description TEXT NOT NULL DEFAULT '';

