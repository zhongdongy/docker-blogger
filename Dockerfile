FROM dongsxyz/python-uwsgi:alpine

ENV LOG_LEVEL=INFO

USER root
WORKDIR /app
COPY requirements.txt requirements.txt
COPY app.py app.py
COPY flask_app.py flask_app.py
COPY utils utils
COPY libs libs
COPY models models
COPY static static
COPY templates templates
COPY blueprints blueprints
RUN curl https://sh.rustup.rs | sh -y

#RUN python3 -m pip install -i https://pypi.tuna.tsinghua.edu.cn/simple --upgrade pip
#RUN python3 -m pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple
RUN python3 -m pip install --upgrade pip
RUN python3 -m pip install -r /app/requirements.txt