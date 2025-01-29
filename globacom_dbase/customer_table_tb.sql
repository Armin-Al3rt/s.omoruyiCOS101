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
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cmobile character(15),
    cemail text NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (cid, cname, cage, cmobile, cemail, eid, data_id) FROM stdin;
110	Musta_karim	35	8055089112     	m-karim@gmail.com	102	5
111	Liliam_jaiye	43	805518341      	l_jaiye@gmail.com	100	3
112	Arthur_musa	50	7055282813     	a_musa@gmail.com	107	10
113	Philip_akonjo	41	9052356772     	p_okonjo@gmail.com	100	2
114	Marylene_mapa	33	805333351      	m_mapa@gmail.com	120	5
115	Oghenero_agor	50	7055566774     	o_agor@gmail.com	117	11
116	Adams_bree	33	8056765424     	a_bree@gmail.com	102	1
117	Okafor_mathias	45	8056763367     	o_mathias@gmail.com	120	10
118	Samson_adeleke	65	7056774423     	s_adeleke@gmail.com	117	11
119	Lawal_tamire	35	9052111101     	l_tamire@gmail.com	107	5
120	James_job	44	8059693919     	j_job@gmail.com	100	8
121	Matthew_jakande	21	7051232144     	m_jakande@gmail.com	120	2
122	Jimila_adegbaye	20	8054921923     	j_adegbaye@gmail.com	107	5
\.


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (cid);


--
-- PostgreSQL database dump complete
--

