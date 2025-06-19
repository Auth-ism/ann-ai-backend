
CREATE TABLE IF NOT EXISTS user_info(
    id SERIAL NOT NULL,
    username varchar(50) NOT NULL,
    full_name varchar(100) NOT NULL,
    email varchar(100) NOT NULL,
    password_hash varchar(255) NOT NULL,
    phone_number varchar(20),
    token_balance numeric(10,2) DEFAULT 0.0000,
    user_role varchar(20) NOT NULL DEFAULT 'user'::character varying,
    subscription_expries timestamp with time zone,
    email_verified boolean DEFAULT false,
    phone_verified boolean DEFAULT false,
    last_login timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(id)
);-- Her index için IF NOT EXISTS ekleyin
CREATE UNIQUE INDEX IF NOT EXISTS user_info_username_key ON public.user_info USING btree (username);
CREATE UNIQUE INDEX IF NOT EXISTS user_info_email_key ON public.user_info USING btree (email);
CREATE INDEX IF NOT EXISTS idx_user_info_email ON public.user_info USING btree (email);
CREATE INDEX IF NOT EXISTS idx_user_info_username ON public.user_info USING btree (username);
CREATE INDEX IF NOT EXISTS idx_user_info_type_active ON public.user_info USING btree (user_role, is_active);