--
-- PostgreSQL database dump
--

\restrict D6d2hDl3n8RLBbNAH068j8RssdkvUqcydE54Au8gkUb1RUXUXxM8HrqVWCvpQVO

-- Dumped from database version 16.1
-- Dumped by pg_dump version 17.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: sessions; Type: TABLE; Schema: public; Owner: pavuk
--

CREATE TABLE public.sessions (
    id integer NOT NULL,
    session_id bytea NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    expires_at timestamp with time zone DEFAULT (now() + '00:30:00'::interval) NOT NULL,
    last_seen_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.sessions OWNER TO pavuk;

--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: pavuk
--

CREATE SEQUENCE public.sessions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.sessions_id_seq OWNER TO pavuk;

--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pavuk
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


--
-- Name: useraccounts; Type: TABLE; Schema: public; Owner: pavuk
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


ALTER TABLE public.useraccounts OWNER TO pavuk;

--
-- Name: useraccounts_account_id_seq; Type: SEQUENCE; Schema: public; Owner: pavuk
--

CREATE SEQUENCE public.useraccounts_account_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.useraccounts_account_id_seq OWNER TO pavuk;

--
-- Name: useraccounts_account_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pavuk
--

ALTER SEQUENCE public.useraccounts_account_id_seq OWNED BY public.useraccounts.id;


--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);


--
-- Name: useraccounts id; Type: DEFAULT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts ALTER COLUMN id SET DEFAULT nextval('public.useraccounts_account_id_seq'::regclass);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: useraccounts unique_username; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT unique_username UNIQUE (username);


--
-- Name: useraccounts useraccounts_pkey; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT useraccounts_pkey PRIMARY KEY (id);


--
-- Name: sessions fk_sessions_user; Type: FK CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT fk_sessions_user FOREIGN KEY (user_id) REFERENCES public.useraccounts(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict D6d2hDl3n8RLBbNAH068j8RssdkvUqcydE54Au8gkUb1RUXUXxM8HrqVWCvpQVO

