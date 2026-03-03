--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: useraccounts; Type: TABLE; Schema: public; Owner: pavuk
--

CREATE TABLE public.useraccounts (
    account_id integer NOT NULL,
    username character varying(50) NOT NULL,
    email character varying(50) NOT NULL,
    pword character varying(255) NOT NULL,
    verified boolean DEFAULT false NOT NULL
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

ALTER SEQUENCE public.useraccounts_account_id_seq OWNED BY public.useraccounts.account_id;


--
-- Name: useraccounts account_id; Type: DEFAULT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts ALTER COLUMN account_id SET DEFAULT nextval('public.useraccounts_account_id_seq'::regclass);


--
-- Name: useraccounts unique_email; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT unique_email UNIQUE (email);


--
-- Name: useraccounts unique_username; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT unique_username UNIQUE (username);


--
-- Name: useraccounts useraccounts_pkey; Type: CONSTRAINT; Schema: public; Owner: pavuk
--

ALTER TABLE ONLY public.useraccounts
    ADD CONSTRAINT useraccounts_pkey PRIMARY KEY (account_id);


--
-- PostgreSQL database dump complete
--

