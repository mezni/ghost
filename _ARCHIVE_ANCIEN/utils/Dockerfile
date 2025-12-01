FROM python:3.9-slim-bullseye

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    gcc \
    build-essential \    
    unzip \
    zip \    
    ca-certificates \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN pip install --no-cache-dir spacy
RUN python -m spacy download en_core_web_lg    