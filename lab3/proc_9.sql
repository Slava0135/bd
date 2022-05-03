CREATE OR REPLACE PROCEDURE update_section_costs()
LANGUAGE SQL
AS
$$
UPDATE route_sections
SET cost = cost + 1000
WHERE route_id = 1;
$$;
CALL update_section_costs();