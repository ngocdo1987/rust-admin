#!/bin/bash

#Host address
#host='locahost'
#Database name
name='rust_admin'
#user name
user='rust_admin'
#login password
password='rust-x-lsl'

#Backup file name, format: year, month, day.sql
sql_file="`date '+%Y%M%d'`.SQL"
if [ -f $sql_file ]; then
    rm -rf $sql_file
fi

#mysqldump path
dump_bin='mysqldump'

#Execute backup
$dump_bin -u$user -p"$password" $name > $sql_file
