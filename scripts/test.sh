#! /usr/bin/env bash

source "$(dirname "$0")/aoc.rc"
aoc_parse_year_day "$@"

shift 2
TC="${1:-ALL}"

pushd "${AOC_DIR}/${YEAR}"
./bin/compile "${DAY}"

NONE=0
PROBLEM=1
ANSWER=2
STATE="${NONE}"

CUR_TC=0
CUR_TEST="$(mktemp -u)"
trap "rm -f ${CUR_TEST}" exit

cat "${INPUT_TEST}" | while read LINE; do
    case "${STATE}" in
        "${NONE}")
            if [[ "${LINE}" =~ -{10}" "Example ]]; then
                CUR_TC="$((CUR_TC+1))"
                if [ "${CUR_TC}" == "${TC}" -o "${TC}" == "ALL" ]; then
                    echo "${LINE}"
                    STATE="${PROBLEM}"
                    > "${CUR_TEST}"
                fi
            fi
            ;;
        "${PROBLEM}")
            if [[ "${LINE}" =~ -{30} ]]; then
                STATE="${ANSWER}"
                ./bin/run "${DAY}" < "${CUR_TEST}"
            else
                echo $LINE >> "${CUR_TEST}"
            fi
            ;;
        "${ANSWER}")
            if [[ "${LINE}" =~ -{30} ]]; then
                STATE="${NONE}"
            else
                echo "${LINE}"
            fi
            ;;
    esac
done
