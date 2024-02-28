-- custom types
CREATE TYPE network_type AS ENUM ('ethereum', 'zksync');
CREATE TYPE platform_type AS ENUM ('discord', 'slack', 'ses');
CREATE TYPE trigger_type AS ENUM ('each_block', 'erc20_transfer');

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
INSERT INTO public.trigger (type) VALUES ('erc20_transfer');


CREATE TABLE IF NOT EXISTS public.notification (
    id SERIAL,
    network_id INT NOT NULL,
    platform_id INT NOT NULL,
    trigger_id INT NOT NULL,
    webhook_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),
    CONSTRAINT fk_network FOREIGN KEY(network_id) REFERENCES network(id),
    CONSTRAINT fk_platform FOREIGN KEY(platform_id) REFERENCES platform(id),
    CONSTRAINT fk_trigger FOREIGN KEY(trigger_id) REFERENCES trigger(id)
);
INSERT INTO public.notification
    (network_id, platform_id, trigger_id, webhook_url)
VALUES
    (
        (SELECT id FROM public.network WHERE type='ethereum'),
        (SELECT id FROM public.platform WHERE type='discord'),
        (SELECT id FROM public.trigger WHERE type='each_block'),
        'https://webhook.com/'
    );
INSERT INTO public.notification
    (network_id, platform_id, trigger_id, webhook_url)
VALUES
    (
        (SELECT id FROM public.network WHERE type='ethereum'),
        (SELECT id FROM public.platform WHERE type='discord'),
        (SELECT id FROM public.trigger WHERE type='erc20_transfer'),
        'https://rand-webhook.org/url'
    );

CREATE TABLE IF NOT EXISTS public.notification_details (
    notification_id INT NOT NULL,
    detail_key VARCHAR(255) NOT NULL,
    detail_value VARCHAR(255) NOT NULL,

    PRIMARY KEY (notification_id, detail_key),
    CONSTRAINT fk_notification FOREIGN KEY(notification_id) REFERENCES public.notification(id) ON DELETE CASCADE
);
--INSERT INTO public.notification_details (notification_id, detail_key, detail_value)
--VALUES
--    (transfer_notification_id, 'from', '0x123...'),
--    (transfer_notification_id, 'to', '0x456...'),
--    (transfer_notification_id, 'value', '1000');
