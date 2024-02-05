-- custom types
CREATE TYPE network_type AS ENUM ('ethereum', 'zksync');
CREATE TYPE platform_type AS ENUM ('discord', 'slack', 'ses');
CREATE TYPE trigger_type AS ENUM ('each_block');

-- tables
CREATE TABLE IF NOT EXISTS public.network (
    id SERIAL,
    type network_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
INSERT INTO public.network (type) VALUES ('ethereum');
INSERT INTO public.network (type) VALUES ('zksync');


CREATE TABLE IF NOT EXISTS public.platform (
    id SERIAL,
    type platform_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
INSERT INTO public.platform (type) VALUES ('discord');
INSERT INTO public.platform (type) VALUES ('slack');
INSERT INTO public.platform (type) VALUES ('ses');


CREATE TABLE IF NOT EXISTS public.trigger (
    id SERIAL,
    type trigger_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
INSERT INTO public.trigger (type) VALUES ('each_block');


CREATE TABLE IF NOT EXISTS public.notification (
    id SERIAL,
    name VARCHAR(255) NOT NULL,
    network_id INT NOT NULL,
    platform_id INT NOT NULL,
    trigger_id INT NOT NULL,
    channel VARCHAR(255) NOT NULL,
    message VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),
    CONSTRAINT fk_network FOREIGN KEY(network_id) REFERENCES network(id),
    CONSTRAINT fk_platform FOREIGN KEY(platform_id) REFERENCES platform(id),
    CONSTRAINT fk_trigger FOREIGN KEY(trigger_id) REFERENCES trigger(id)
);
INSERT INTO public.notification (name, network_id, platform_id, trigger_id, channel, message) VALUES ('test', 1, 1, 1, '1111111111111111111', 'test message');
