FROM python:3.11-slim

WORKDIR /app

COPY tools/requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY tools/. .

ENTRYPOINT ["python", "generator.py"]
