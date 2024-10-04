FROM python:3.9-slim

RUN apt-get update && apt-get install -y \
    libjpeg-dev zlib1g-dev

WORKDIR /app

COPY . /app

RUN pip install --no-cache-dir -r requirements.txt

EXPOSE 5000

CMD ["python", "app.py"]
