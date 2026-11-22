-- The state of each injected item when it was injected (records: unverified,
-- verified, suspect), for the stale-injection metric.
ALTER TABLE usage ADD COLUMN status TEXT;
