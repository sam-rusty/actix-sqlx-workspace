BEGIN;
-- create ama record
INSERT INTO ama (content, effective_date, status)
VALUES ('test', now(), 'Active');
-- create plan
INSERT INTO PLAN (id, name, country, description, total_grid_percentage, default_level, initial_generation_level,
                  generations_perc, ama_id, price, created_at)
VALUES (1, 'Builder', 'US', 'Test', 82.5, 'TA' :: user_level, 'RMD' :: user_level, '{0.15, 0.7, 0.425}', 1, 199.99,
        NOW());

-- add plan level
INSERT INTO plan_level (id, plan_id, name, label, percentage, status, effective_date)
VALUES (1, 1, 'Training Associate', 'TA', 30.0, 'Active', now());
WITH create_user AS (
    INSERT INTO users (first_name, last_name, level, user_name, email, type, state, country, upline, phone, code)
        VALUES ('Hubert', 'Humphrey', 'TA', 'hubert', 'hubert@hgicrusade.com', 'Associate', 'GA', 'US', 'HGI',
                '+17786866393', 'HH123456') RETURNING code),
     create_meta AS (
         INSERT INTO user_meta (code, legal_first_name, legal_last_name, marital_status, perc, ssn, address, city, zip,
                                password, gender, dob)
             VALUES ((SELECT code FROM create_user),
                     'Hubert',
                     'Humphrey',
                     'Married',
                     78.25,
                     crypt('781781', gen_salt('bf')),
                     'Address',
                     'Atlanta',
                     '86000',
                     '$2a$12$cPW7dACHYXH9LEv3Hu5Q4erTkSMbcfeHafQEcDLdmcJWH8dPr9MEO',
                     'Male',
                     '01/01/145') RETURNING code)
INSERT
INTO enrolled_plan (code, plan_id)
VALUES ((SELECT code FROM create_meta), 1);
COMMIT;