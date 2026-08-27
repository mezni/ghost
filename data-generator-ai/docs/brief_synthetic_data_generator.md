# Product Brief: Synthetic Data Generator

## Executive Summary

A personal productivity tool for data scientists that rapidly generates synthetic datasets for any topic using LLM-driven schema inference. Instead of spending hours searching for public datasets or manually crafting data generation scripts, the user provides a topic (e.g., "telecom router logs," "customer reviews"), and the app suggests a realistic schema, lets the user review and adjust it, then generates a complete dataset in minutes.

This tool solves a real pain: data science projects often stall because relevant training data is scarce, proprietary, or time-consuming to generate. By automating schema inference and data generation, the app accelerates prototyping, model testing, and data augmentation workflows.

## The Problem

Data scientists frequently need synthetic datasets for prototyping, testing, and augmentation—but the process is broken:

- **Scarcity:** Real-world datasets (especially for niche domains like telecom logs, medical records, or proprietary business data) are often unavailable, privacy-constrained, or low quality.
- **Time cost:** Manually writing Python scripts to generate data—defining schemas, crafting prompts, cleaning outputs—is tedious and slow.
- **Domain knowledge required:** To generate realistic data, you need to understand the domain's structure (fields, types, distributions), which takes research and expertise.

**Current workaround:** Write one-off scripts with LLM API calls, manually define schemas, iterate on prompts. This works but wastes hours on repetitive tasks.

**Cost of status quo:** Delayed prototyping, underperforming models due to lack of diverse training data, and frustration from repetitive data engineering tasks.

## The Solution

A Streamlit web app that uses LLMs to rapidly generate synthetic datasets:

1. **User provides a topic** (e.g., "telecom router logs," "customer reviews," "medical records").
2. **LLM infers a schema** (columns, data types, realistic distributions).
3. **User reviews and adjusts** the schema (add/remove columns, tweak distributions, set data characteristics).
4. **LLM generates the dataset** with optional imperfections (missing values, outliers, skewness).
5. **User downloads** the dataset as CSV or JSON.

The app eliminates the need for manual schema design and prompt engineering—just describe what you need, and the LLM handles the rest.

## What Makes This Different

- **General-purpose:** Works for any domain—no pre-built templates or hardcoded schemas.
- **LLM-driven schema inference:** The app suggests realistic fields and distributions based on the topic, reducing cognitive load.
- **Interactive review:** User can adjust the inferred schema before generation, ensuring the output matches their needs.
- **Rapid iteration:** From topic to downloadable dataset in under 5 minutes.
- **Built-in data characteristics:** Supports missing values, outliers, skewness, and uniqueness controls for realistic synthetic data.

**Unfair advantage:** The combination of LLM-driven schema inference + interactive review + rapid generation. Most tools either require manual schema definition or produce generic data without domain-specific structure.

## Who This Serves

**Primary user:** Dali (data scientist)
- Needs rapid prototyping and testing datasets for various projects
- Works across domains (telecom, customer analytics, etc.)
- Values speed and flexibility over enterprise features

**Secondary users (future):** Other data scientists who need synthetic data for prototyping, testing, or augmentation. Potential open-source community.

**Success looks like:** Going from "I need a dataset about X" to "I have a usable dataset" in under 5 minutes, without writing code or manually defining schemas.

## Success Criteria

- **Speed:** Topic to downloadable dataset in under 5 minutes.
- **Quality:** Generated data is realistic enough to train/evaluate models meaningfully.
- **Flexibility:** Works for any topic without pre-built templates.
- **Usability:** Non-technical users (or users unfamiliar with a domain) can generate useful datasets.
- **Iteration:** User can refine schema and regenerate until satisfied.

## Scope

**In for MVP:**
- Streamlit UI for topic input, schema review, and data generation
- FastAPI backend with LLM integration for schema inference and data generation
- Support for text classification and tabular data
- CSV and JSON download formats
- Basic data characteristics (missing values, outliers, skewness)
- Local deployment (Docker)

**Out for MVP:**
- User authentication or multi-user support
- Cloud deployment or scaling
- Advanced data characteristics (complex distributions, temporal patterns)
- Integration with external data sources
- Fine-tuning or custom LLM models
- Batch generation (large datasets > 10k rows)

## Vision

If this succeeds, it becomes a go-to tool for rapid synthetic data generation—first for personal use, then potentially shared as an open-source project or lightweight SaaS. The long-term vision is a platform where any data scientist can generate realistic training data for any domain in minutes, not hours.
