BEGIN;

INSERT INTO ama (content, effective_date, status)
VALUES ('This is a test content', DATE('2023-01-01'), 'Active');

COMMIT;