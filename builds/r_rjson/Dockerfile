FROM rocker/r-ver:4.2.2
RUN install2.r --error rjson purrr
COPY implementations/R_rjson/process.R main.R
CMD R -e "source('/main.R')"
