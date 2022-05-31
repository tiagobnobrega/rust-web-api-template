CREATE TABLE "users" (
                         "username" varchar PRIMARY KEY,
                         "primary_email" varchar UNIQUE NOT NULL,
                         "password" varchar NOT NULL,
                         "first_name" varchar NOT NULL,
                         "last_name" varchar NOT NULL
);

CREATE TABLE "roles" (
                         "role_id" SERIAL PRIMARY KEY,
                         "role_name" varchar UNIQUE NOT NULL
);

CREATE TABLE "users_roles" (
                               "users_roles_id" SERIAL PRIMARY KEY,
                               "username" varchar NOT NULL,
                               "role_id" int NOT NULL
);

CREATE TABLE "system_actions" (
                                  "action_id" SERIAL PRIMARY KEY,
                                  "action_name" varchar UNIQUE NOT NULL
);

CREATE TABLE "roles_system_actions" (
                                        "roles_system_actions_id" SERIAL PRIMARY KEY,
                                        "role_id" int NOT NULL,
                                        "action_id" int NOT NULL
);

ALTER TABLE "users_roles" ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE "users_roles" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("role_id");

ALTER TABLE "roles_system_actions" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("role_id");

ALTER TABLE "roles_system_actions" ADD FOREIGN KEY ("role_id") REFERENCES "system_actions" ("action_id");


-- === SEED DATA ===

INSERT INTO system_actions(action_name) VALUES('SECURITY/READ');
INSERT INTO system_actions(action_name) VALUES('SECURITY/EDIT');

INSERT INTO roles(role_name) VALUES('SECURITY/READER');
INSERT INTO roles(role_name) VALUES('SECURITY/EDITOR');

INSERT INTO roles_system_actions(role_id, action_id) VALUES(
                                                               (select role_id from roles where role_name='SECURITY/READER') ,
                                                               (select action_id from system_actions where action_name='SECURITY/READ')
                                                           );
INSERT INTO roles_system_actions(role_id, action_id) VALUES(
                                                               (select role_id from roles where role_name='SECURITY/EDITOR') ,
                                                               (select action_id from system_actions where action_name='SECURITY/READ')
                                                           );
INSERT INTO roles_system_actions(role_id, action_id) VALUES(
                                                               (select role_id from roles where role_name='SECURITY/EDITOR') ,
                                                               (select action_id from system_actions where action_name='SECURITY/EDIT')
                                                           );


