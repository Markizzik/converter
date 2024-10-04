# Используем официальный образ Nginx
FROM nginx:alpine

# Копируем HTML, CSS и статические файлы в директорию Nginx
COPY ./index.html /nginx/html/index.html
COPY ./styles.css /nginx/html/styles.css
COPY ./static /nginx/html/static

# Копируем Nginx конфигурацию
COPY ./nginx.conf /etc/nginx/conf.d/default.conf