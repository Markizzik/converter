from flask import Flask, render_template, request, send_file, redirect, url_for
import os
from PIL import Image
from werkzeug.utils import secure_filename

app = Flask(__name__)

# Путь для сохранения загруженных файлов и конвертированных файлов
UPLOAD_FOLDER = 'uploads'
CONVERTED_FOLDER = 'converted'
ALLOWED_EXTENSIONS = {'png', 'jpg', 'jpeg', 'gif', 'bmp', 'tiff', 'webp'}
app.config['UPLOAD_FOLDER'] = UPLOAD_FOLDER
app.config['CONVERTED_FOLDER'] = CONVERTED_FOLDER

# Создаём папки, если они не существуют
if not os.path.exists(UPLOAD_FOLDER):
    os.makedirs(UPLOAD_FOLDER)
if not os.path.exists(CONVERTED_FOLDER):
    os.makedirs(CONVERTED_FOLDER)

# Проверка допустимости расширения файла
def allowed_file(filename):
    return '.' in filename and filename.rsplit('.', 1)[1].lower() in ALLOWED_EXTENSIONS

# Конвертация изображения в целевой формат
def convert_image(input_path, output_format):
    output_file = os.path.splitext(input_path)[0] + f".{output_format}"
    with Image.open(input_path) as img:
        img = img.convert("RGB")
        img.save(output_file)
    return output_file

@app.route('/')
def index():
    return render_template('index.html')  # Используется ваш файл index.html

@app.route('/upload', methods=['POST'])
def upload_file():
    if 'file' not in request.files:
        return 'No file part'
    
    file = request.files['file']
    if file.filename == '':
        return 'No selected file'
    
    if file and allowed_file(file.filename):
        filename = secure_filename(file.filename)
        input_path = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        file.save(input_path)  # Сохранение загруженного файла в папку uploads

        return f"Файл успешно загружен: {filename}"

    return 'File type not allowed'

@app.route('/convert', methods=['POST'])
def convert_file():
    if 'file' not in request.files or 'format' not in request.form:
        return "Файл или формат не указан", 400

    file = request.files['file']
    format = request.form['format'].lower()

    if format not in ALLOWED_EXTENSIONS:
        return "Неподдерживаемый формат", 400

    if file and allowed_file(file.filename):
        # Сохранение исходного файла в папку uploads
        filename = secure_filename(file.filename)
        input_path = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        file.save(input_path)

        # Конвертация файла в нужный формат
        output_file = convert_image(input_path, format)
        
        return send_file(output_file, as_attachment=True, download_name=f'converted.{format}')

    return 'File type not allowed'

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
