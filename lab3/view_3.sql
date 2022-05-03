CREATE OR REPLACE VIEW "Order Contacts" AS
SELECT concat(
        'Email: ',
        email,
        '    Phone Number: ',
        phone_number
    ) AS contacts
FROM orders;
SELECT * FROM "Order Contacts"