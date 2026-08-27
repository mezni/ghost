# Brainstorming: Synthetic Data Generation Application

## 1. Project Overview
*   **Goal:** Build a small application to generate synthetic training examples for text classification using an LLM.
*   **Purpose:** Data Science (augmenting datasets, testing model robustness).
*   **Tech Stack:** Python, FastAPI (Backend), Streamlit (Frontend/UI).

## 2. User Interface Flow (Streamlit)
The application will guide the user through a step-by-step process:

### Step 1: Dataset Definition
*   **Topic & Goal:** User inputs the dataset topic (e.g., "customer reviews") and the primary goal (e.g., "augment a small dataset").
*   **Task Type:** Confirm "Text Classification."

### Step 2: Column Management
*   **Default Columns:** `text_content` (Text), `label` (Categorical).
*   **Dynamic Interaction:** Users can **add** or **remove** columns.
*   **Smart Suggestions:**
    *   Based on the topic, the system proposes initial columns.
    *   Users define **Column Name**, **Purpose** (e.g., Metadata, Feature, Label), and **Data Type** (Text, Categorical, Numerical, etc.).
    *   The system suggests the most likely Data Type based on the Purpose.

### Step 3: Data Characteristics Configuration
*   **Ordinal Labels:** If a column is categorical, the user proposes a list of labels (e.g., "Positive", "Neutral", "Negative") and chooses the length of the list.
*   **Data Imperfections:** Users can toggle and configure:
    *   **Empty Values:** Percentage of missing data.
    *   **Outliers:** Percentage of unusual/nonsensical data.
    *   **Skewness/Distribution:** Label imbalance (e.g., 90% positive) and text style distribution.
    *   **Repetition/Uniqueness:** Control over duplicate or highly similar entries.
*   **UI Controls:** Sliders, checkboxes, and dropdowns for fine-tuning these parameters.

### Step 4: Review & Generate
*   **Preview:** Display a preview of the generated data (e.g., first 50 rows).
*   **Summary Stats:** Show actual label distribution, missing value counts, etc.
*   **Download:** Options to download as CSV or JSON.
*   **Refine:** Option to go back and tweak parameters.

## 3. Backend Architecture (FastAPI)
*   **Endpoint:** `POST /generate_data`
*   **Input:** JSON payload mirroring the Streamlit configuration (topic, columns, characteristics).
*   **Process:**
    1.  **Validation:** Check input constraints (e.g., unique column names, sensible percentages).
    2.  **Prompt Construction:** Dynamically build a detailed prompt based on user choices (Topic, Columns, Distribution, Tone).
    3.  **LLM Call:** Send prompt to the LLM API.
    4.  **Post-Processing:** Use `pandas`/`numpy` to enforce exact distributions (e.g., ensuring exactly 10% missing values if requested) and inject outliers/missing values programmatically.
    5.  **Response:** Return the generated JSON array.

## 4. Guardrails & Safety
### Explainability
*   **Traceability:** Store and display the exact prompt used for each generation batch.
*   **LLM Explanations:** Ask the LLM to provide a brief explanation of how it adhered to the requested characteristics.
*   **Tagging:** Identify which rows are "synthetic noise" (outliers/missing) vs. "clean" examples.

### LLM Interaction
*   **Strict Schema Enforcement:** Use `Pydantic` models to validate LLM output. Retry with a more explicit prompt if validation fails.
*   **Output Filtering:** Use heuristic checks or a secondary LLM call to verify the tone/style of generated text.
*   **Parameter Controls:** Expose `temperature` and `top-p` settings for advanced users.
*   **Rate Limiting & Backoff:** Implement robust retry logic for API errors (429/500).

### Input Validation
*   **Sanitization:** Prevent prompt injection via user inputs (topic, column names).
*   **Content Moderation:** Flag or block sensitive/harmful topics using safety models.
*   **Sensible Ranges:** Limit dataset size (e.g., max 10k rows) and imperfection percentages (e.g., max 50% missing) to ensure usability.

## 5. Deployment Strategy
*   **Containerization:** Use Docker to package the Streamlit and FastAPI applications.
*   **Orchestration:** Docker Compose for local development; Kubernetes or managed PaaS (e.g., Render, Heroku) for production.
*   **Environment Variables:** Securely manage API keys and configurations.

## 6. Next Steps
1.  **Detailed API Design:** Define the exact JSON schema for requests and responses.
2.  **Prompt Engineering:** Develop and test prompt templates for various classification tasks.
3.  **UI Prototyping:** Build the Streamlit interface skeleton.
4.  **Backend Implementation:** Set up FastAPI with basic endpoints and LLM integration.
