FROM python:3.8-slim
WORKDIR /test_app
COPY . .

RUN echo "docker built"
CMD ["python", "app.py"]