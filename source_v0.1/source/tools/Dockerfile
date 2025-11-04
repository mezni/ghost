FROM python:3.11-slim

WORKDIR /app

# Copy from the correct path relative to build context
COPY source/tools/requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application from correct path
COPY source/tools/. .

CMD ["python", "generator.py"]