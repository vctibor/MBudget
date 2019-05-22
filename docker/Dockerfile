FROM centos


# Prepare system

RUN yum update -y

RUN yum install -y openssl openssl-devel

RUN mkdir /var/mcalendar

RUN mkdir /var/mcalendar/mcalendar.sled


# Copy required files

ADD mcalendar.tar /var/mcalendar


EXPOSE 8000

CMD ["/var/mcalendar/mcalendar", "/var/mcalendar/config.toml"]
