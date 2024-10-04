# Используем официальный образ Python
FROM python:3.9-slim

# Устанавливаем зависимости для Pillow (работа с изображениями)
RUN apt-get update && apt-get install -y \
    libjpeg-dev zlib1g-dev

# Устанавливаем рабочую директорию в контейнере
WORKDIR /app

# Копируем файлы в рабочую директорию контейнера
COPY . /app

# Устанавливаем зависимости из requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Открываем порт 5000 для Flask
EXPOSE 5000

# Команда для запуска Flask-приложения
CMD ["python", "app.py"]
