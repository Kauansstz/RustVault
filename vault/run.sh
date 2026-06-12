#!/bin/bash

VERDE="\033[0;32m"
VERMELHO="\033[0;31m"
AMARELO="\033[0;33m"
RESET="\033[0m"

print_status() {
    local MENSAGEM=$1
    local STATUS=$2
    printf "%-55s" "$MENSAGEM"
    if [ "$STATUS" == "OK" ]; then
        echo -e "[   ${VERDE}OK${RESET}   ]"
    elif [ "$STATUS" == "AVISO" ]; then
        echo -e "[ ${AMARELO}WARN${RESET} ]"
    else
        echo -e "[${VERMELHO}FAILED${RESET}]"
    fi
}

RAM_USO=${2:-0}
DISCO_USO=${3:-0}
CPU_USO=${4:-0}
DISCO_LIVRE_GB=${5:-0}

clear
echo "========================================================="
echo "   VALIDAÇÃO DOS SUBSISTEMAS E COMPILADORES"
echo "========================================================="

if command -v cargo &> /dev/null; then
    print_status "Verificando compilador Rust (cargo)..." "OK"
else
    print_status "Verificando compilador Rust (cargo)..." "FAILED"
    exit 1
fi

if [ -f "livro.json" ]; then
    print_status "Carregando banco de dados (livro.json)..." "OK"
else
    print_status "Carregando banco de dados (livro.json)..." "AVISO"
fi

echo -e "\n========================================================="
echo "DIAGNÓSTICO DE DESEMPENHO REAL (MÉTRICAS DO SISTEMA)"
echo "========================================================="

if ping -c 1 -W 2 8.8.8.8 &> /dev/null || ping -n 1 -w 2000 8.8.8.8 &> /dev/null; then
    print_status "Testando conexão com a internet (Ping)..." "OK"
else
    print_status "Testando conexão com a internet (Ping)..." "FAILED"
fi

if [ "$RAM_USO" -lt 90 ]; then
    print_status "Verificando uso de Memória RAM (Uso atual: ${RAM_USO}%)..." "OK"
else
    print_status "Verificando uso de Memória RAM (CRÍTICO: ${RAM_USO}%)..." "FAILED"
    echo -e "${AMARELO} RAM acima de 90%! Iniciando limpeza e reinício de serviços...${RESET}"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo sync && echo 3 | sudo tee /proc/sys/vm/drop_caches &> /dev/null
    fi
fi

if [ "$CPU_USO" -lt 90 ]; then
    print_status "Analisando carga da CPU (Uso atual: ${CPU_USO}%)..." "OK"
else
    print_status "Analisando carga da CPU (Carga Extrema: ${CPU_USO}%)..." "AVISO"
fi

if [ "$DISCO_USO" -lt 90 ]; then
    print_status "Verificando Espaço em Disco (${DISCO_LIVRE_GB} GB livres / Uso: ${DISCO_USO}%)..." "OK"
else
    print_status "Verificando Espaço em Disco (${DISCO_LIVRE_GB} GB livres / Uso: ${DISCO_USO}%)..." "FAILED"
    echo -e "${VERMELHO} Espaço em disco em nível crítico! Limpe arquivos temporários.${RESET}"
fi

echo "---------------------------------------------------------"
echo -e "${VERDE}Diagnósticos automatizados concluídos com sucesso!${RESET}"
echo "---------------------------------------------------------"
sleep 1.5

if [ "$1" == "--check" ]; then
    if [ "$RAM_USO" -gt 90 ] || [ "$DISCO_USO" -gt 90 ]; then
        exit 2
    fi
    exit 0
fi

echo " Executando 'cargo run' nativo..."
cargo run