# syntax=docker/dockerfile:1

FROM quay.io/jupyter/base-notebook

RUN pip install --no-cache-dir numpy pandas matplotlib scikit-learn torch transformers langchain langchain-huggingface nltk spacy    

RUN python -m spacy download en_core_web_sm && \
    python -m spacy download en_core_web_lg && \
    python -m nltk.downloader punkt

#RUN apt-get remove -y build-essential && \
#    apt-get autoremove -y && \
#    apt-get clean && \
#    rm -rf /var/lib/apt/lists/*
