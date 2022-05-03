CREATE OR REPLACE PROCEDURE delete_cheapest_tickets()
LANGUAGE SQL
AS
$$
DELETE FROM tickets
WHERE price_paid = (
        SELECT min(price_paid)
        FROM tickets
    );
$$;
CALL delete_cheapest_tickets();