--
-- PostgreSQL database dump
--

\restrict TXmGmOfAeICqq6ZWqGCHhfrO39eGgopLRUofwhwFvJMGS2mb3xqFvwECFNnMSls

-- Dumped from database version 14.22 (Ubuntu 14.22-0ubuntu0.22.04.1)
-- Dumped by pg_dump version 14.22 (Ubuntu 14.22-0ubuntu0.22.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: belt_rank; Type: TYPE; Schema: public; Owner: mungtung
--

CREATE TYPE public.belt_rank AS ENUM (
    'white',
    'blue',
    'purple',
    'brown',
    'black'
);


ALTER TYPE public.belt_rank OWNER TO mungtung;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: mungtung
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO mungtung;

--
-- Name: fighters; Type: TABLE; Schema: public; Owner: mungtung
--

CREATE TABLE public.fighters (
    user_id integer NOT NULL,
    name text DEFAULT 'No Name'::text NOT NULL,
    rank text DEFAULT 'white'::text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    description character varying(255) DEFAULT '???'::character varying,
    wins integer DEFAULT 0 NOT NULL,
    losses integer DEFAULT 0 NOT NULL,
    draws integer DEFAULT 0 NOT NULL,
    weight_kg integer DEFAULT 0 NOT NULL,
    picture_url text DEFAULT 'default.png'::text,
    gym_id_fk integer
);


ALTER TABLE public.fighters OWNER TO mungtung;

--
-- Name: fighters_user_id_seq; Type: SEQUENCE; Schema: public; Owner: mungtung
--

CREATE SEQUENCE public.fighters_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.fighters_user_id_seq OWNER TO mungtung;

--
-- Name: fighters_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: mungtung
--

ALTER SEQUENCE public.fighters_user_id_seq OWNED BY public.fighters.user_id;


--
-- Name: sessions; Type: TABLE; Schema: public; Owner: mungtung
--

CREATE TABLE public.sessions (
    id integer NOT NULL,
    session_id bytea NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    expires_at timestamp with time zone DEFAULT (now() + '00:30:00'::interval) NOT NULL,
    last_seen_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.sessions OWNER TO mungtung;

--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: mungtung
--

CREATE SEQUENCE public.sessions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.sessions_id_seq OWNER TO mungtung;

--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: mungtung
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


--
-- Name: useraccounts; Type: TABLE; Schema: public; Owner: mungtung
--

CREATE TABLE public.useraccounts (
    id integer NOT NULL,
    username text NOT NULL,
    password_hash text NOT NULL,
    verified boolean DEFAULT false NOT NULL,
    email_nonce bytea NOT NULL,
    user_key bytea NOT NULL,
    user_key_nonce bytea NOT NULL,
    email bytea NOT NULL
);


ALTER TABLE public.useraccounts OWNER TO mungtung;

--
-- Name: useraccounts_account_id_seq; Type: SEQUENCE; Schema: public; Owner: mungtung
--

CREATE SEQUENCE public.useraccounts_account_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.useraccounts_account_id_seq OWNER TO mungtung;

--
-- Name: useraccounts_account_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: mungtung
--

ALTER SEQUENCE public.useraccounts_account_id_seq OWNED BY public.useraccounts.id;


--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);


--
-- Name: useraccounts id; Type: DEFAULT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.useraccounts ALTER COLUMN id SET DEFAULT nextval('public.useraccounts_account_id_seq'::regclass);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: fighters fighters_pkey; Type: CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.fighters
    ADD CONSTRAINT fighters_pkey PRIMARY KEY (user_id);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: useraccounts unique_username; Type: CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT unique_username UNIQUE (username);


--
-- Name: useraccounts useraccounts_pkey; Type: CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT useraccounts_pkey PRIMARY KEY (id);


--
-- Name: fighters fk_fighter_user; Type: FK CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.fighters
    ADD CONSTRAINT fk_fighter_user FOREIGN KEY (user_id) REFERENCES public.useraccounts(id) ON DELETE CASCADE;


--
-- Name: sessions fk_sessions_user; Type: FK CONSTRAINT; Schema: public; Owner: mungtung
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT fk_sessions_user FOREIGN KEY (user_id) REFERENCES public.useraccounts(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict TXmGmOfAeICqq6ZWqGCHhfrO39eGgopLRUofwhwFvJMGS2mb3xqFvwECFNnMSls

