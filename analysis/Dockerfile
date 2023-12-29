# adapted from Jupyter Notebook Scientific Python Stack 
# https://github.com/jupyter/docker-stacks/tree/main/scipy-notebook

FROM  jupyter/minimal-notebook

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

USER root

RUN apt-get update --yes && \
    apt-get install --yes --no-install-recommends \
    build-essential \
    cm-super \
    dvipng \
    ffmpeg && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

USER ${NB_UID}

RUN mamba install --yes \
    'sqlalchemy' \
    'psycopg2-binary' \
    'matplotlib' \
    'seaborn' \
    'numpy' \
    'pandas' && \
    mamba clean --all -f -y && \
    fix-permissions "${CONDA_DIR}" && \
    fix-permissions "/home/${NB_USER}"

RUN pip install jupyter_contrib_nbextensions

WORKDIR /tmp
RUN git clone https://github.com/PAIR-code/facets.git && \
    # jupyter nbextension install facets/facets-dist/ --sys-prefix && \
    rm -rf /tmp/facets && \
    fix-permissions "${CONDA_DIR}" && \
    fix-permissions "/home/${NB_USER}"

ENV XDG_CACHE_HOME="/home/${NB_USER}/.cache/"

RUN MPLBACKEND=Agg python -c "import matplotlib.pyplot" && \
    fix-permissions "/home/${NB_USER}"

USER ${NB_UID}

WORKDIR "${HOME}"
