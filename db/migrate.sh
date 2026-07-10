#!/bin/bash

# Defina suas credenciais e informações do banco de dados
DB_USER="root"
DB_PASS="root"
DO_HOST="localhost"
DB_NAME="clients_rust_db"

# Caminho para o arquivo SQL
SQL_FILE="restore.sql"

echo "Iniciando o proceso de restauração de banco de dados..."

# Conecte-se ao banco de MySQL/MariaDB e execute o script SQL
mysql -u"$DB_USER" -p"$DB_PASS" -h "$DO_HOST" < "$SQL_FILE"

if [ $? -eq 0 ]; then
  echo "Restauração concluída com sucesso."
else
  echo "Falha ao restaurar o banco de dados."
fi
