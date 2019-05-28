FROM centos


# Prepare system

RUN yum update -y

RUN yum install -y openssl openssl-devel

RUN mkdir /var/mbudget


# Copy required files

ADD mbudget.tar /var/mbudget


EXPOSE 8080

CMD ["/var/mbudget/mbudget", "/var/mbudget/config.toml"]
