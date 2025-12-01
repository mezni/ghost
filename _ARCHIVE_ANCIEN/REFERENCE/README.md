ports:
    - api: 3000
    - frontend: 8080
    - postgres: 5432


db connection:
- docker exec -it database psql -U myuser -d roamdb